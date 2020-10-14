//! Pkmnapi database module
//!
//! # Example
//!
//! ```
//! use pkmnapi_db::*;
//! use std::fs;
//! # use std::env;
//! # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
//!
//! let rom = fs::read(rom_path).unwrap();
//! let db = PkmnapiDB::new(&rom, None).unwrap();
//! ```

pub mod cry;
pub mod error;
pub mod header;
pub mod img;
pub mod map;
pub mod patch;
pub mod pic;
pub mod sav;
pub mod string;
pub mod types;

use byteorder::{LittleEndian, ReadBytesExt};
use cry::*;
use error::Result;
use header::*;
use img::*;
use map::*;
use patch::*;
use pic::*;
use sav::*;
use std::cmp;
use std::io::Cursor;
use std::num::Wrapping;
use types::*;

/// Pkmnapi database
///
/// # Example
///
/// ```
/// use pkmnapi_db::*;
/// use std::fs;
/// # use std::env;
/// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
///
/// let rom = fs::read(rom_path).unwrap();
/// let db = PkmnapiDB::new(&rom, None).unwrap();
/// ```
#[derive(Debug)]
pub struct PkmnapiDB {
    pub rom: Vec<u8>,
    pub sav: Option<Sav>,
    pub hash: String,
    pub header: Header,
}

impl PkmnapiDB {
    pub const ROM_PAGE: usize = 0x2000;

    /// Create new database
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    /// ```
    pub fn new(rom: &Vec<u8>, sav: Option<Vec<u8>>) -> Result<PkmnapiDB> {
        let hash = format!("{:x}", md5::compute(&rom));
        let header = Header::from(&rom)?;
        let rom = rom[..].to_vec();
        let sav = match sav {
            Some(sav) => Some(Sav::new(&sav)?),
            None => None,
        };

        Ok(PkmnapiDB {
            rom,
            sav,
            hash,
            header,
        })
    }

    /// Verify global checksum
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// assert_eq!(db.verify_checksum(), true);
    /// ```
    pub fn verify_checksum(&self) -> bool {
        let rom = [&self.rom[..0x014E], &self.rom[0x0150..]].concat();
        let checksum = rom
            .iter()
            .fold(Wrapping(0u16), |acc, x| acc + Wrapping(*x as u16));

        checksum.0 == self.header.global_checksum
    }

    /// Generate global checksum
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db.generate_checksum();
    ///
    /// // RED
    /// # #[cfg(feature = "PKMN_RED")]
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x014E,
    ///         length: 0x02,
    ///         data: vec![0x91, 0xE6]
    ///     }
    /// );
    ///
    /// // BLUE
    /// # #[cfg(not(feature = "PKMN_RED"))]
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x014E,
    ///         length: 0x02,
    ///         data: vec![0x9D, 0x0A]
    ///     }
    /// );
    /// ```
    pub fn generate_checksum(&self) -> Patch {
        let rom = [&self.rom[..0x014E], &self.rom[0x0150..]].concat();
        let checksum = rom
            .iter()
            .fold(Wrapping(0u16), |acc, x| acc + Wrapping(*x as u16));

        let checksum = checksum.0.to_be_bytes().to_vec();

        Patch::new(&0x014E, &checksum)
    }

    /// Verify ROM hash
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// // RED
    /// # #[cfg(feature = "PKMN_RED")]
    /// assert_eq!(db.verify_hash("3d45c1ee9abd5738df46d2bdda8b57dc"), true);
    ///
    /// // BLUE
    /// # #[cfg(not(feature = "PKMN_RED"))]
    /// assert_eq!(db.verify_hash("50927e843568814f7ed45ec4f944bd8b"), true);
    /// ```
    pub fn verify_hash<S: Into<String>>(&self, hash: S) -> bool {
        self.hash == hash.into()
    }

    /// Apply ROM patch
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let mut db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// assert_eq!(db.rom[..4], [0xFF, 0x00, 0x00, 0x00]);
    ///
    /// let patch = Patch::new(&0x00, &vec![0x13, 0x37]);
    ///
    /// db.apply_patch(patch);
    ///
    /// assert_eq!(db.rom[..4], [0x13, 0x37, 0x00, 0x00]);
    /// ```
    pub fn apply_patch<S: Into<Patch>>(&mut self, patch: S) {
        let patch = patch.into();

        self.rom = [
            &self.rom[..patch.offset],
            &patch.data[..],
            &self.rom[(patch.offset + patch.length)..],
        ]
        .concat();
    }

    /// Pokémon internal max
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let pokemon_internal_max = db.pokemon_internal_max();
    ///
    /// assert_eq!(pokemon_internal_max, 190);
    /// ```
    pub fn pokemon_internal_max(&self) -> usize {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x38;
        let offset = offset_base + 0x1E5F;

        (self.rom[offset] as usize) - 1
    }

    /// Pokémon name to Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let pokemon_name = PokemonName {
    ///     name: ROMString::from("BULBASAUR"),
    /// };
    ///
    /// let pokedex_id = db.pokemon_name_to_pokedex_id(&pokemon_name).unwrap();
    ///
    /// assert_eq!(pokedex_id, 1);
    /// ```
    pub fn pokemon_name_to_pokedex_id(&self, pokemon_name: &PokemonName) -> Option<u8> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x0E;
        let offset = offset_base + 0x021E;
        let pokemon_internal_max = self.pokemon_internal_max();

        return (0..pokemon_internal_max)
            .map(|i| offset + (i * 0x0A))
            .enumerate()
            .filter_map(|(internal_id, offset)| {
                let internal_id = internal_id as u8;
                let name = PokemonName::from(&self.rom[offset..(offset + 0x0A)]);

                if name == *pokemon_name {
                    let pokedex_id = self.internal_id_to_pokedex_id(&internal_id).unwrap();

                    return Some(pokedex_id);
                }

                None
            })
            .take(1)
            .next();
    }

    /// Pokédex ID to internal ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let pokedex_id = 151;
    /// let internal_id = db.pokedex_id_to_internal_id(&pokedex_id).unwrap();
    ///
    /// assert_eq!(internal_id, 0x14);
    /// ```
    pub fn pokedex_id_to_internal_id(&self, pokedex_id: &u8) -> Result<u8> {
        if pokedex_id < &1 {
            return Err(error::Error::PokedexIDInvalid(*pokedex_id));
        }

        let offset_base = PkmnapiDB::ROM_PAGE * 0x20;
        let offset = offset_base + 0x1024;
        let pokemon_internal_max = self.pokemon_internal_max();

        let internal_id = match (&self.rom[offset..(offset + pokemon_internal_max)])
            .iter()
            .position(|r| pokedex_id == r)
        {
            Some(internal_id) => internal_id,
            None => return Err(error::Error::PokedexIDInvalid(*pokedex_id)),
        };

        Ok(internal_id as u8)
    }

    /// Internal ID to Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let internal_id = 0x14;
    /// let pokedex_id = db.internal_id_to_pokedex_id(&internal_id).unwrap();
    ///
    /// assert_eq!(pokedex_id, 151);
    /// ```
    pub fn internal_id_to_pokedex_id(&self, internal_id: &u8) -> Result<u8> {
        let pokemon_internal_max = self.pokemon_internal_max();

        if internal_id >= &(pokemon_internal_max as u8) {
            return Err(error::Error::InternalIDInvalid(*internal_id));
        }

        let offset_base = PkmnapiDB::ROM_PAGE * 0x20;
        let offset = (offset_base + 0x1024) + (*internal_id as usize);

        Ok(self.rom[offset])
    }

    /// Validate type ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let type_id = 0;
    ///
    /// match db.type_id_validate(&type_id) {
    ///     Ok(min_max) => assert_eq!(min_max, (0, 26)),
    ///     Err(_) => unreachable!()
    /// };
    ///
    /// let type_id = 100;
    ///
    /// match db.type_id_validate(&type_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::TypeIDInvalid(type_id, 0, 26))
    /// };
    /// ```
    pub fn type_id_validate(&self, type_id: &u8) -> Result<(usize, usize)> {
        let min_id = 0usize;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x10;
        let pointer_base = offset_base + 0x7DAE;

        let max_index = (&self.rom[pointer_base..])
            .iter()
            .position(|&r| r == 0x8D)
            .unwrap();
        let max_id = (((max_index as f32) / 2.0) as usize) - 1;

        if *type_id > (max_id as u8) {
            return Err(error::Error::TypeIDInvalid(*type_id, min_id, max_id));
        }

        Ok((min_id, max_id))
    }

    /// Validate type effect ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let type_effect_id = 0;
    ///
    /// match db.type_effect_id_validate(&type_effect_id) {
    ///     Ok(min_max) => assert_eq!(min_max, (0, 81)),
    ///     Err(_) => unreachable!()
    /// };
    ///
    /// let type_effect_id = 100;
    ///
    /// match db.type_effect_id_validate(&type_effect_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::TypeEffectIDInvalid(type_effect_id, 0, 81))
    /// };
    /// ```
    pub fn type_effect_id_validate(&self, type_effect_id: &u8) -> Result<(usize, usize)> {
        let min_id = 0usize;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1F;
        let pointer = offset_base + 0x0474;

        let max_index = (&self.rom[pointer..])
            .iter()
            .position(|&r| r == 0xFF)
            .unwrap();
        let max_id = (((max_index as f32) / 3.0) as usize) - 1;

        if *type_effect_id > (max_id as u8) {
            return Err(error::Error::TypeEffectIDInvalid(
                *type_effect_id,
                min_id,
                max_id,
            ));
        }

        Ok((min_id, max_id))
    }

    /// Validate trainer ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let trainer_id = 1;
    ///
    /// match db.trainer_id_validate(&trainer_id) {
    ///     Ok(min_max) => assert_eq!(min_max, (1, 47)),
    ///     Err(_) => unreachable!()
    /// };
    ///
    /// let trainer_id = 100;
    ///
    /// match db.trainer_id_validate(&trainer_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::TrainerIDInvalid(trainer_id, 1, 47))
    /// };
    /// ```
    pub fn trainer_id_validate(&self, trainer_id: &u8) -> Result<(usize, usize)> {
        let min_id = 1usize;

        let offset_base = (PkmnapiDB::ROM_PAGE * 0x1C) + 0x19FF;

        let max_offset = (&self.rom[offset_base..])
            .iter()
            .position(|&r| r == 0x21)
            .unwrap();
        let max_id = (&self.rom[offset_base..(offset_base + max_offset)])
            .iter()
            .filter(|&x| *x == 0x50)
            .count();

        if *trainer_id < (min_id as u8) || *trainer_id > (max_id as u8) {
            return Err(error::Error::TrainerIDInvalid(*trainer_id, min_id, max_id));
        }

        Ok((min_id, max_id))
    }

    /// Validate HM ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let hm_id = 1;
    ///
    /// match db.hm_id_validate(&hm_id) {
    ///     Ok(min_max) => assert_eq!(min_max, (1, 5)),
    ///     Err(_) => unreachable!()
    /// };
    ///
    /// let hm_id = 100;
    ///
    /// match db.hm_id_validate(&hm_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::HMIDInvalid(hm_id, 1, 5))
    /// };
    /// ```
    pub fn hm_id_validate(&self, hm_id: &u8) -> Result<(usize, usize)> {
        let min_id = 1usize;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x01;
        let offset_base = offset_base + 0x1052;

        let max_id = (&self.rom[offset_base..])
            .iter()
            .position(|&r| r == 0xFF)
            .unwrap();

        if *hm_id < (min_id as u8) || *hm_id > (max_id as u8) {
            return Err(error::Error::HMIDInvalid(*hm_id, min_id, max_id));
        }

        Ok((min_id, max_id))
    }

    /// Validate TM ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let tm_id = 1;
    ///
    /// match db.tm_id_validate(&tm_id) {
    ///     Ok(min_max) => assert_eq!(min_max, (1, 50)),
    ///     Err(_) => unreachable!()
    /// };
    ///
    /// let tm_id = 100;
    ///
    /// match db.tm_id_validate(&tm_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::TMIDInvalid(tm_id, 1, 50))
    /// };
    /// ```
    pub fn tm_id_validate(&self, tm_id: &u8) -> Result<(usize, usize)> {
        let min_id = 1usize;
        let max_id = 50usize;

        if *tm_id < (min_id as u8) || *tm_id > (max_id as u8) {
            return Err(error::Error::TMIDInvalid(*tm_id, min_id, max_id));
        }

        Ok((min_id, max_id))
    }

    /// Validate item ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let item_id = 1;
    ///
    /// match db.item_id_validate(&item_id) {
    ///     Ok(min_max) => assert_eq!(min_max, (1, 97)),
    ///     Err(_) => unreachable!()
    /// };
    ///
    /// let item_id = 100;
    ///
    /// match db.item_id_validate(&item_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::ItemIDInvalid(item_id, 1, 97))
    /// };
    /// ```
    pub fn item_id_validate(&self, item_id: &u8) -> Result<(usize, usize)> {
        let min_id = 1usize;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x02;
        let offset_base = offset_base + 0x072B;

        let max_offset = (&self.rom[offset_base..])
            .iter()
            .position(|&r| r == 0xD0)
            .unwrap();
        let max_id = (&self.rom[offset_base..(offset_base + max_offset)])
            .iter()
            .filter(|&x| *x == 0x50)
            .count();

        if *item_id < (min_id as u8) || (item_id - 1) >= (max_id as u8) {
            return Err(error::Error::ItemIDInvalid(*item_id, min_id, max_id));
        }

        Ok((min_id, max_id))
    }

    /// Validate move ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let move_id = 1;
    ///
    /// match db.move_id_validate(&move_id) {
    ///     Ok(min_max) => assert_eq!(min_max, (1, 165)),
    ///     Err(_) => unreachable!()
    /// };
    ///
    /// let move_id = 200;
    ///
    /// match db.move_id_validate(&move_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::MoveIDInvalid(move_id, 1, 165))
    /// };
    /// ```
    pub fn move_id_validate(&self, move_id: &u8) -> Result<(usize, usize)> {
        let min_id = 1usize;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;

        let max_index = self.rom[offset_base..]
            .chunks(2)
            .position(|r| r == [0x01, 0x2D])
            .unwrap();
        let max_id = ((max_index as f32) / 3.0) as usize;

        if *move_id < (min_id as u8) || *move_id > (max_id as u8) {
            return Err(error::Error::MoveIDInvalid(*move_id, min_id, max_id));
        }

        Ok((min_id, max_id))
    }

    /// Validate map ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let map_id = 0;
    ///
    /// match db.map_id_validate(&map_id) {
    ///     Ok(min_max) => assert_eq!(min_max, (0, 247)),
    ///     Err(_) => unreachable!()
    /// };
    ///
    /// let map_id = 255;
    ///
    /// match db.map_id_validate(&map_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::MapIDInvalid(map_id, 0, 247))
    /// };
    /// ```
    pub fn map_id_validate(&self, map_id: &u8) -> Result<(usize, usize)> {
        let min_id = 0usize;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x06;
        let offset = offset_base + 0x0EEB;

        let max_id = self.rom[offset..]
            .chunks(2)
            .position(|r| r == [0xFF, 0xFF])
            .unwrap()
            - 1 as usize;

        if *map_id > (max_id as u8) {
            return Err(error::Error::MapIDInvalid(*map_id, min_id, max_id));
        }

        Ok((min_id, max_id))
    }

    /// Validate icon ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let icon_id = 0;
    ///
    /// match db.icon_id_validate(&icon_id) {
    ///     Ok(min_max) => assert_eq!(min_max, (0, 9)),
    ///     Err(_) => unreachable!()
    /// };
    ///
    /// let icon_id = 100;
    ///
    /// match db.icon_id_validate(&icon_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::IconIDInvalid(icon_id, 0, 9))
    /// };
    /// ```
    pub fn icon_id_validate(&self, icon_id: &u8) -> Result<(usize, usize)> {
        let min_id = 0usize;
        let max_id = 9usize;

        if icon_id > &(max_id as u8) {
            return Err(error::Error::IconIDInvalid(*icon_id, min_id, max_id));
        }

        Ok((min_id, max_id))
    }

    /// Get type name by type ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let type_name = db.get_type_name(&0).unwrap();
    ///
    /// assert_eq!(
    ///     type_name,
    ///     TypeName {
    ///         name: ROMString::from("NORMAL")
    ///     }
    /// );
    /// ```
    pub fn get_type_name(&self, type_id: &u8) -> Result<TypeName> {
        let _max_id = self.type_id_validate(type_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x10;
        let pointer_base = offset_base + 0x7DAE;
        let pointer_offset = pointer_base + ((*type_id as usize) * 2);
        let pointer = offset_base + {
            let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let type_name = TypeName::from(&self.rom[pointer..=(pointer + 9)]);

        Ok(type_name)
    }

    /// Set type name by type ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db
    ///     .set_type_name(
    ///         &0,
    ///         &TypeName {
    ///             name: ROMString::from("BORING"),
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x27DE4,
    ///         length: 0x06,
    ///         data: vec![0x81, 0x8E, 0x91, 0x88, 0x8D, 0x86]
    ///     }
    /// );
    /// ```
    pub fn set_type_name(&self, type_id: &u8, type_name: &TypeName) -> Result<Patch> {
        let old_type_name = self.get_type_name(type_id)?;
        let old_type_name_len = old_type_name.name.value.len();
        let type_name_raw = type_name.to_raw();
        let type_name_len = type_name_raw.len();

        if old_type_name_len < type_name_len {
            return Err(error::Error::TypeNameWrongSize(
                old_type_name_len,
                type_name_len,
            ));
        }

        let offset_base = PkmnapiDB::ROM_PAGE * 0x10;
        let pointer_offset = (offset_base + 0x7DAE) + ((*type_id as usize) * 2);
        let pointer = offset_base + {
            let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let data = [type_name_raw, vec![0x50; old_type_name_len - type_name_len]].concat();

        Ok(Patch::new(&pointer, &data))
    }

    /// Get type effect by type effect ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let type_effect = db.get_type_effect(&0).unwrap();
    ///
    /// assert_eq!(
    ///     type_effect,
    ///     TypeEffect {
    ///         attacking_type_id: 0x15,
    ///         defending_type_id: 0x14,
    ///         multiplier: 2.0
    ///     }
    /// );
    /// ```
    pub fn get_type_effect(&self, type_effect_id: &u8) -> Result<TypeEffect> {
        let _max_id = self.type_effect_id_validate(type_effect_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1F;
        let pointer = (offset_base + 0x0474) + ((*type_effect_id as usize) * 0x03);

        let type_effect = TypeEffect::from(&self.rom[pointer..(pointer + 3)]);

        Ok(type_effect)
    }

    /// Set type effect by type effect ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db
    ///     .set_type_effect(
    ///         &0,
    ///         &TypeEffect {
    ///             attacking_type_id: 0x13,
    ///             defending_type_id: 0x37,
    ///             multiplier: 0.5,
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x3E474,
    ///         length: 0x03,
    ///         data: vec![0x13, 0x37, 0x05]
    ///     }
    /// );
    /// ```
    pub fn set_type_effect(&self, type_effect_id: &u8, type_effect: &TypeEffect) -> Result<Patch> {
        let _max_id = self.type_effect_id_validate(type_effect_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1F;
        let pointer = offset_base + 0x0474 + ((*type_effect_id as usize) * 3);

        let type_effect_raw = type_effect.to_raw();

        Ok(Patch::new(&pointer, &type_effect_raw))
    }

    /// Get Pokémon stats by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let pokemon_stats = db.get_pokemon_stats(&1).unwrap();
    ///
    /// assert_eq!(
    ///     pokemon_stats,
    ///     PokemonStats {
    ///         pokedex_id: 1,
    ///         base_hp: 45,
    ///         base_attack: 49,
    ///         base_defence: 49,
    ///         base_speed: 45,
    ///         base_special: 65,
    ///         type_ids: vec![22, 3],
    ///         catch_rate: 45,
    ///         base_exp_yield: 64
    ///     }
    /// );
    /// ```
    pub fn get_pokemon_stats(&self, pokedex_id: &u8) -> Result<PokemonStats> {
        let _internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset = {
            if pokedex_id == &151 {
                0x425B
            } else {
                let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;

                (offset_base + 0x03DE) + (((*pokedex_id as usize) - 1) * 0x1C)
            }
        };

        let pokemon_stats = PokemonStats::from(&self.rom[offset..(offset + 0x1C)]);

        Ok(pokemon_stats)
    }

    /// Set Pokémon stats by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db
    ///     .set_pokemon_stats(
    ///         &1,
    ///         &PokemonStats {
    ///             pokedex_id: 0x01,
    ///             base_hp: 0x42,
    ///             base_attack: 0x13,
    ///             base_defence: 0x37,
    ///             base_speed: 0x13,
    ///             base_special: 0x37,
    ///             type_ids: vec![0x13, 0x37],
    ///             catch_rate: 0x13,
    ///             base_exp_yield: 0x37,
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x383DE,
    ///         length: 0x0A,
    ///         data: vec![0x01, 0x42, 0x13, 0x37, 0x13, 0x37, 0x13, 0x37, 0x13, 0x37]
    ///     }
    /// );
    /// ```
    pub fn set_pokemon_stats(
        &self,
        pokedex_id: &u8,
        pokemon_stats: &PokemonStats,
    ) -> Result<Patch> {
        let _internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
        let offset = (offset_base + 0x03DE) + (((*pokedex_id as usize) - 1) * 0x1C);

        let pokemon_stats_raw = pokemon_stats.to_raw();

        Ok(Patch::new(&offset, &pokemon_stats_raw))
    }

    /// Get Pokémon machines by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let pokemon_machines = db.get_pokemon_machines(&1).unwrap();
    ///
    /// assert_eq!(
    ///     pokemon_machines,
    ///     vec![
    ///         PokemonMachine::TM(0x03),
    ///         PokemonMachine::TM(0x06),
    ///         PokemonMachine::TM(0x08),
    ///         PokemonMachine::TM(0x09),
    ///         PokemonMachine::TM(0x0A),
    ///         PokemonMachine::TM(0x14),
    ///         PokemonMachine::TM(0x15),
    ///         PokemonMachine::TM(0x16),
    ///         PokemonMachine::TM(0x1F),
    ///         PokemonMachine::TM(0x20),
    ///         PokemonMachine::TM(0x21),
    ///         PokemonMachine::TM(0x22),
    ///         PokemonMachine::TM(0x2C),
    ///         PokemonMachine::TM(0x32),
    ///         PokemonMachine::HM(0x01),
    ///     ]
    /// );
    /// ```
    pub fn get_pokemon_machines(&self, pokedex_id: &u8) -> Result<Vec<PokemonMachine>> {
        let _internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset = {
            if pokedex_id == &151 {
                0x425B
            } else {
                let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;

                (offset_base + 0x03DE) + (((*pokedex_id as usize) - 1) * 0x1C)
            }
        } + 0x14;

        let (_min_tm_id, max_tm_id) = self.tm_id_validate(&0x01)?;
        let (_min_hm_id, max_hm_id) = self.hm_id_validate(&0x01)?;

        let machines: Vec<PokemonMachine> = self.rom[offset..(offset + 0x07)]
            .iter()
            .map(|byte| {
                (0..8)
                    .map(|i| (byte & (0x01 << i)) >> i)
                    .collect::<Vec<u8>>()
            })
            .flatten()
            .enumerate()
            .filter_map(|(i, bit)| {
                if bit == 0 {
                    return None;
                }

                if i >= max_tm_id {
                    let hm_id = (i + 1) - max_tm_id;

                    if hm_id > max_hm_id {
                        None
                    } else {
                        Some(PokemonMachine::HM(hm_id as u8))
                    }
                } else {
                    Some(PokemonMachine::TM((i as u8) + 1))
                }
            })
            .collect::<Vec<PokemonMachine>>();

        Ok(machines)
    }

    /// Set Pokémon machines by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db
    ///     .set_pokemon_machines(
    ///         &1,
    ///         &vec![
    ///             PokemonMachine::TM(0x02),
    ///             PokemonMachine::TM(0x07),
    ///             PokemonMachine::TM(0x09),
    ///             PokemonMachine::TM(0x0A),
    ///             PokemonMachine::TM(0x0D),
    ///             PokemonMachine::TM(0x11),
    ///             PokemonMachine::TM(0x12),
    ///             PokemonMachine::TM(0x13),
    ///             PokemonMachine::TM(0x15),
    ///             PokemonMachine::TM(0x16),
    ///             PokemonMachine::TM(0x19),
    ///             PokemonMachine::TM(0x21),
    ///             PokemonMachine::TM(0x24),
    ///             PokemonMachine::TM(0x26),
    ///             PokemonMachine::TM(0x27),
    ///             PokemonMachine::TM(0x2A),
    ///             PokemonMachine::TM(0x2F),
    ///             PokemonMachine::TM(0x31),
    ///             PokemonMachine::TM(0x32),
    ///             PokemonMachine::HM(0x01),
    ///             PokemonMachine::HM(0x02)
    ///         ]
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x383F2,
    ///         length: 0x07,
    ///         data: vec![0x42, 0x13, 0x37, 0x01, 0x69, 0x42, 0x0F]
    ///     }
    /// );
    /// ```
    pub fn set_pokemon_machines(
        &self,
        pokedex_id: &u8,
        pokemon_machines: &Vec<PokemonMachine>,
    ) -> Result<Patch> {
        let _internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset = {
            if pokedex_id == &151 {
                0x425B
            } else {
                let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;

                (offset_base + 0x03DE) + (((*pokedex_id as usize) - 1) * 0x1C)
            }
        } + 0x14;

        let (_min_id, max_id) = self.tm_id_validate(&0x01)?;
        let mut pokemon_machines = pokemon_machines.to_vec();

        pokemon_machines.sort();

        let pokemon_machines = pokemon_machines
            .iter()
            .map(|pokemon_machine| {
                match pokemon_machine {
                    PokemonMachine::TM(tm_id) => {
                        let (_, _) = self.tm_id_validate(&tm_id)?;
                    }
                    PokemonMachine::HM(hm_id) => {
                        let (_, _) = self.hm_id_validate(&hm_id)?;
                    }
                };

                Ok(pokemon_machine)
            })
            .collect::<Result<Vec<_>>>()?;

        let machine_ids: Vec<u8> = pokemon_machines
            .iter()
            .map(|pokemon_machine| match pokemon_machine {
                PokemonMachine::TM(tm_id) => tm_id - 1,
                PokemonMachine::HM(hm_id) => (hm_id - 1) + (max_id as u8),
            })
            .collect();

        let data: Vec<u8> = (0..(7 * 8))
            .map(|i| {
                if machine_ids.contains(&(i as u8)) {
                    0x01
                } else {
                    0x00
                }
            })
            .collect::<Vec<u8>>()
            .as_slice()
            .chunks(8)
            .map(|chunk| (0..8).map(|i| chunk[7 - i] << 7 - i).fold(0, |a, b| a | b))
            .collect();

        Ok(Patch::new(&offset, &data))
    }

    /// Get Pokémon name by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let pokemon_name = db.get_pokemon_name(&1).unwrap();
    ///
    /// assert_eq!(
    ///     pokemon_name,
    ///     PokemonName {
    ///         name: ROMString::from("BULBASAUR")
    ///     }
    /// );
    /// ```
    pub fn get_pokemon_name(&self, pokedex_id: &u8) -> Result<PokemonName> {
        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x0E;
        let offset = (offset_base + 0x021E) + ((internal_id as usize) * 0x0A);

        let pokemon_name = PokemonName::from(&self.rom[offset..(offset + 0x0A)]);

        Ok(pokemon_name)
    }

    /// Set Pokémon name by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db
    ///     .set_pokemon_name(
    ///         &1,
    ///         &PokemonName {
    ///             name: ROMString::from("ABC"),
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x1C80E,
    ///         length: 0x0A,
    ///         data: vec![0x80, 0x81, 0x82, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50]
    ///     }
    /// );
    /// ```
    pub fn set_pokemon_name(&self, pokedex_id: &u8, pokemon_name: &PokemonName) -> Result<Patch> {
        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x0E;
        let offset = (offset_base + 0x021E) + ((internal_id as usize) * 0x0A);

        let pokemon_name_len = pokemon_name.name.value.len();
        let pokemon_name_raw = pokemon_name.to_raw();

        let data = [pokemon_name_raw, vec![0x50; 0x0A - pokemon_name_len]].concat();

        Ok(Patch::new(&offset, &data))
    }

    /// Get move stats by move ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let move_stats = db.get_move_stats(&1).unwrap();
    ///
    /// assert_eq!(
    ///     move_stats,
    ///     MoveStats {
    ///         move_id: 0x01,
    ///         effect: 0x00,
    ///         power: 0x28,
    ///         type_id: 0x00,
    ///         accuracy: 1.0,
    ///         pp: 0x23
    ///     }
    /// );
    /// ```
    pub fn get_move_stats(&self, move_id: &u8) -> Result<MoveStats> {
        let (_min_id, _max_id) = self.move_id_validate(move_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
        let offset = offset_base + (((*move_id as usize) - 1) * 0x06);

        let move_stats = MoveStats::from(&self.rom[offset..(offset + 6)]);

        Ok(move_stats)
    }

    /// Set move stats by move ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db
    ///     .set_move_stats(
    ///         &1,
    ///         &MoveStats {
    ///             move_id: 0x01,
    ///             effect: 0x00,
    ///             power: 0xFF,
    ///             type_id: 0x01,
    ///             accuracy: 0.0,
    ///             pp: 0xFF,
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x38000,
    ///         length: 0x06,
    ///         data: vec![0x01, 0x00, 0xFF, 0x01, 0x00, 0xFF]
    ///     }
    /// );
    /// ```
    pub fn set_move_stats(&self, move_id: &u8, move_stats: &MoveStats) -> Result<Patch> {
        let (_min_id, _max_id) = self.move_id_validate(move_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
        let offset = offset_base + (((*move_id as usize) - 1) * 0x06);

        let move_stats_raw = move_stats.to_raw();

        Ok(Patch::new(&offset, &move_stats_raw))
    }

    /// Get move name by move ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let move_name = db.get_move_name(&1).unwrap();
    ///
    /// assert_eq!(
    ///     move_name,
    ///     MoveName {
    ///         name: ROMString::from("POUND")
    ///     }
    /// );
    /// ```
    pub fn get_move_name(&self, move_id: &u8) -> Result<MoveName> {
        let (min_id, max_id) = self.move_id_validate(move_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x58;
        let offset = match {
            if move_id == &1 {
                Some(offset_base)
            } else {
                self.rom[offset_base..]
                    .iter()
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if *x == 0x50 {
                            return Some(offset_base + i + 1);
                        }

                        None
                    })
                    .take(max_id)
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if (*move_id as usize) - 2 == i {
                            return Some(x);
                        }

                        None
                    })
                    .next()
            }
        } {
            Some(offset) => offset,
            None => return Err(error::Error::MoveIDInvalid(*move_id, min_id, max_id)),
        };

        let move_name = MoveName::from(&self.rom[offset..(offset + 13)]);

        Ok(move_name)
    }

    /// Set move name by move ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db
    ///     .set_move_name(
    ///         &1,
    ///         &MoveName {
    ///             name: ROMString::from("ABCDE"),
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0xB0000,
    ///         length: 0x05,
    ///         data: vec![0x80, 0x81, 0x82, 0x83, 0x084]
    ///     }
    /// );
    /// ```
    pub fn set_move_name(&self, move_id: &u8, move_name: &MoveName) -> Result<Patch> {
        let (min_id, max_id) = self.move_id_validate(move_id)?;

        let old_move_name = self.get_move_name(move_id)?;
        let old_move_name_len = old_move_name.name.value.len();
        let move_name_raw = move_name.to_raw();
        let move_name_len = move_name_raw.len();

        if old_move_name_len != move_name_len {
            return Err(error::Error::MoveNameWrongSize(
                old_move_name_len,
                move_name_len,
            ));
        }

        let offset_base = PkmnapiDB::ROM_PAGE * 0x58;
        let offset = match {
            if move_id == &1 {
                Some(offset_base)
            } else {
                self.rom[offset_base..]
                    .iter()
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if *x == 0x50 {
                            return Some(offset_base + i + 1);
                        }

                        None
                    })
                    .take(max_id)
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if (*move_id as usize) - 2 == i {
                            return Some(x);
                        }

                        None
                    })
                    .next()
            }
        } {
            Some(offset) => offset,
            None => return Err(error::Error::MoveIDInvalid(*move_id, min_id, max_id)),
        };

        Ok(Patch::new(&offset, &move_name_raw))
    }

    /// Get HM by HM ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let hm = db.get_hm(&1).unwrap();
    ///
    /// assert_eq!(hm, HM { move_id: 0x0F });
    /// ```
    pub fn get_hm(&self, hm_id: &u8) -> Result<HM> {
        let _max_id = self.hm_id_validate(hm_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x01;
        let offset = (offset_base + 0x1052) + ((*hm_id as usize) - 1);

        let hm = HM::from(self.rom[offset]);

        Ok(hm)
    }

    /// Set HM by HM ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db.set_hm(&1, &HM { move_id: 0x42 }).unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x3052,
    ///         length: 0x01,
    ///         data: vec![0x42]
    ///     }
    /// );
    /// ```
    pub fn set_hm(&self, hm_id: &u8, hm: &HM) -> Result<Patch> {
        let _max_id = self.hm_id_validate(hm_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x01;
        let offset = (offset_base + 0x1052) + ((*hm_id as usize) - 1);

        Ok(Patch::new(&offset, &hm.to_raw()))
    }

    /// Get TM by TM ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let tm = db.get_tm(&1).unwrap();
    ///
    /// assert_eq!(tm, TM { move_id: 0x05 });
    /// ```
    pub fn get_tm(&self, tm_id: &u8) -> Result<TM> {
        let _max_id = self.tm_id_validate(tm_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x09;
        let offset = (offset_base + 0x1773) + ((*tm_id as usize) - 1);

        let tm = TM::from(self.rom[offset]);

        Ok(tm)
    }

    /// Set TM by TM ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db.set_tm(&1, &TM { move_id: 0x42 }).unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x13773,
    ///         length: 0x01,
    ///         data: vec![0x42]
    ///     }
    /// );
    /// ```
    pub fn set_tm(&self, tm_id: &u8, tm: &TM) -> Result<Patch> {
        let _max_id = self.tm_id_validate(tm_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x09;
        let offset = (offset_base + 0x1773) + ((*tm_id as usize) - 1);

        Ok(Patch::new(&offset, &tm.to_raw()))
    }

    /// Get TM price by TM ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let tm_price = db.get_tm_price(&1).unwrap();
    ///
    /// assert_eq!(tm_price, TMPrice { value: 3000 });
    /// ```
    pub fn get_tm_price(&self, tm_id: &u8) -> Result<TMPrice> {
        let _max_id = self.tm_id_validate(tm_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x3D;
        let offset = (offset_base + 0x1FA7) + (((*tm_id as usize - 1) as f32 / 2.0) as usize);
        let value = {
            if ((tm_id - 1) % 2) == 0 {
                (self.rom[offset] & 0xF0) >> 4
            } else {
                self.rom[offset] & 0x0F
            }
        };

        let tm_price = TMPrice::from(value);

        Ok(tm_price)
    }

    /// Set TM price by TM ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db.set_tm_price(&1, &TMPrice { value: 9000 }).unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x7BFA7,
    ///         length: 0x01,
    ///         data: vec![0x92]
    ///     }
    /// );
    /// ```
    pub fn set_tm_price(&self, tm_id: &u8, tm_price: &TMPrice) -> Result<Patch> {
        let _max_id = self.tm_id_validate(tm_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x3D;
        let offset = (offset_base + 0x1FA7) + ((((*tm_id as usize) - 1) as f32 / 2.0) as usize);
        let value = {
            if ((tm_id - 1) % 2) == 0 {
                (self.rom[offset] & 0x0F) | (tm_price.to_raw()[0] << 0x04)
            } else {
                (self.rom[offset] & 0xF0) | (tm_price.to_raw()[0])
            }
        };

        Ok(Patch::new(&offset, &vec![value]))
    }

    /// Get Pokédex entry by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let pokedex_entry = db.get_pokedex_entry(&1).unwrap();
    ///
    /// assert_eq!(pokedex_entry, PokedexEntry {
    ///     species: ROMString::from("SEED"),
    ///     height: 28,
    ///     weight: 150
    /// });
    /// ```
    pub fn get_pokedex_entry(&self, pokedex_id: &u8) -> Result<PokedexEntry> {
        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1E;
        let pointer_offset = (offset_base + 0x447E) + ((internal_id as usize) * 2);

        let pointer = offset_base + {
            let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let pokedex_entry = PokedexEntry::from(&self.rom[pointer..(pointer + 15)]);

        Ok(pokedex_entry)
    }

    /// Set Pokédex entry by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db
    ///     .set_pokedex_entry(&1, &PokedexEntry {
    ///         species: ROMString::from("BLAH"),
    ///         height: 100,
    ///         weight: 300
    ///     })
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x40E33,
    ///         length: 0x09,
    ///         data: vec![0x81, 0x8B, 0x80, 0x87, 0x50, 0x08, 0x04, 0x2C, 0x01]
    ///     }
    /// );
    /// ```
    pub fn set_pokedex_entry(
        &self,
        pokedex_id: &u8,
        pokedex_entry: &PokedexEntry,
    ) -> Result<Patch> {
        let old_pokedex_entry_species = self.get_pokedex_entry(pokedex_id)?;
        let old_pokedex_entry_species_len = old_pokedex_entry_species.species.value.len();
        let pokedex_entry_species_len = pokedex_entry.species.value.len();

        if old_pokedex_entry_species_len != pokedex_entry_species_len {
            return Err(error::Error::PokedexEntrySpeciesWrongSize(
                old_pokedex_entry_species_len,
                pokedex_entry_species_len,
            ));
        }

        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1E;
        let pointer_offset = (offset_base + 0x447E) + ((internal_id as usize) * 2);

        let pointer = offset_base + {
            let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        Ok(Patch::new(&pointer, &pokedex_entry.to_raw()))
    }

    /// Get Pokédex entry text by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let pokedex_text = db.get_pokedex_text(&1).unwrap();
    ///
    /// assert_eq!(pokedex_text, PokedexText {
    ///     text: ROMString::from("A strange seed was\nplanted on its\nback at birth.¶The plant sprouts\nand grows with\nthis #MON"),
    /// });
    /// ```
    pub fn get_pokedex_text(&self, pokedex_id: &u8) -> Result<PokedexText> {
        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1E;
        let pointer_offset = (offset_base + 0x447E) + ((internal_id as usize) * 2);

        let pointer = offset_base + {
            let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let pointer_offset =
            pointer + { self.rom[pointer..].iter().position(|&r| r == 0x50).unwrap() } + 0x06;

        let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 3)]);

        let pointer = cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize;
        let pointer_base = (PkmnapiDB::ROM_PAGE * 2) * { cursor.read_u8().unwrap_or(0) as usize };
        let pointer = pointer + pointer_base - (PkmnapiDB::ROM_PAGE * 2);

        let pokedex_text = PokedexText::from(&self.rom[pointer..]);

        Ok(pokedex_text)
    }

    /// Set Pokédex entry text by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db
    ///     .set_pokedex_text(&1, &PokedexText {
    ///         text: ROMString::from("ABCDE"),
    ///     })
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0xAEE81,
    ///         length: 0x07,
    ///         data: vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
    ///     }
    /// );
    /// ```
    pub fn set_pokedex_text(&self, pokedex_id: &u8, pokedex_text: &PokedexText) -> Result<Patch> {
        let old_pokedex_text = self.get_pokedex_text(pokedex_id)?;
        let old_pokedex_text_len = old_pokedex_text.text.value.len();
        let pokedex_text_raw = pokedex_text.text.to_string();
        let pokedex_text_len = pokedex_text_raw.len();

        if pokedex_text_len >= old_pokedex_text_len {
            return Err(error::Error::PokedexTextWrongSize(
                old_pokedex_text_len,
                pokedex_text_len,
            ));
        }

        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1E;
        let pointer_offset = (offset_base + 0x447E) + ((internal_id as usize) * 2);

        let pointer = offset_base + {
            let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let pointer_offset =
            pointer + { self.rom[pointer..].iter().position(|&r| r == 0x50).unwrap() } + 0x06;

        let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 3)]);

        let pointer = cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize;
        let pointer_base = (PkmnapiDB::ROM_PAGE * 2) * { cursor.read_u8().unwrap_or(0) as usize };
        let pointer = pointer + pointer_base - (PkmnapiDB::ROM_PAGE * 2);

        Ok(Patch::new(&pointer, &pokedex_text.to_raw()))
    }

    /// Get Pokémon pic by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let pokemon_pic = db.get_pokemon_pic(&1, &PokemonPicFace::FRONT).unwrap();
    ///
    /// assert_eq!(pokemon_pic.width, 5);
    /// assert_eq!(pokemon_pic.height, 5);
    /// assert_eq!(pokemon_pic.pixels.len(), 1600);
    /// ```
    pub fn get_pokemon_pic(
        &self,
        pokedex_id: &u8,
        pokemon_pic_face: &PokemonPicFace,
    ) -> Result<Pic> {
        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let (offset, bank_offset) = {
            if pokedex_id == &151 {
                let offset_base = PkmnapiDB::ROM_PAGE * 0x02;
                let offset = offset_base + 0x025B;
                let bank_offset = (self.rom[0x163A] - 1) * 0x02;

                (offset, bank_offset as usize)
            } else {
                let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
                let offset = (offset_base + 0x03DE) + (((*pokedex_id as usize) - 1) * 0x1C);

                let bank_offset = match internal_id {
                    _ if internal_id < self.rom[0x1646] - 1 => self.rom[0x1648],
                    _ if internal_id < self.rom[0x164D] - 1 => self.rom[0x164F],
                    _ if internal_id < self.rom[0x1654] - 1 => self.rom[0x1656],
                    _ if internal_id < self.rom[0x165B] - 1 => self.rom[0x165D],
                    _ => self.rom[0x1661],
                };
                let bank_offset = (bank_offset - 1) * 0x02;

                (offset, bank_offset as usize)
            }
        };

        let mut cursor = Cursor::new(&self.rom[(offset + 11)..(offset + 15)]);

        let pointer_front = cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize;
        let pointer_back = cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize;

        let offset_base = PkmnapiDB::ROM_PAGE * bank_offset;
        let offset_front = offset_base + pointer_front;
        let offset_back = offset_base + pointer_back;

        let pointer = match pokemon_pic_face {
            PokemonPicFace::FRONT => offset_front,
            PokemonPicFace::BACK => offset_back,
        };

        let pic = Pic::new(&self.rom[pointer..])?;

        Ok(pic)
    }

    /// Set Pokémon pic by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::pic::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let pokemon_pic = Pic::new(&vec![0x55]).unwrap();
    /// let patch = db.set_pokemon_pic(&1, &PokemonPicFace::FRONT, &pokemon_pic, PicEncodingMethod::THREE(0x01)).unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x34000,
    ///         length: 0x07,
    ///         data: vec![0x55, 0xBF, 0xD2, 0x1D, 0xFE, 0x90, 0x80]
    ///     }
    /// );
    /// ```
    pub fn set_pokemon_pic(
        &self,
        pokedex_id: &u8,
        pokemon_pic_face: &PokemonPicFace,
        pic: &Pic,
        encoding_method: PicEncodingMethod,
    ) -> Result<Patch> {
        let old_pixels = self.get_pokemon_pic(pokedex_id, pokemon_pic_face)?;
        let pixels = pic.encode(encoding_method);

        if pixels.len() > old_pixels.bytes + 1 {
            return Err(error::Error::PicTooLarge);
        }

        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let (offset, bank_offset) = {
            if pokedex_id == &151 {
                let offset_base = PkmnapiDB::ROM_PAGE * 0x02;
                let offset = offset_base + 0x025B;
                let bank_offset = (self.rom[0x163A] - 1) * 0x02;

                (offset, bank_offset as usize)
            } else {
                let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
                let offset = (offset_base + 0x03DE) + (((*pokedex_id as usize) - 1) * 0x1C);

                let bank_offset = match internal_id {
                    _ if internal_id < self.rom[0x1646] - 1 => self.rom[0x1648],
                    _ if internal_id < self.rom[0x164D] - 1 => self.rom[0x164F],
                    _ if internal_id < self.rom[0x1654] - 1 => self.rom[0x1656],
                    _ if internal_id < self.rom[0x165B] - 1 => self.rom[0x165D],
                    _ => self.rom[0x1661],
                };
                let bank_offset = (bank_offset - 1) * 0x02;

                (offset, bank_offset as usize)
            }
        };

        let mut cursor = Cursor::new(&self.rom[(offset + 11)..(offset + 15)]);

        let pointer_front = cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize;
        let pointer_back = cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize;

        let offset_base = PkmnapiDB::ROM_PAGE * bank_offset;
        let offset_front = offset_base + pointer_front;
        let offset_back = offset_base + pointer_back;

        let pointer = match pokemon_pic_face {
            PokemonPicFace::FRONT => offset_front,
            PokemonPicFace::BACK => offset_back,
        };

        Ok(Patch::new(&pointer, &pixels))
    }

    /// Get trainer name by trainer ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let trainer_name = db.get_trainer_name(&1).unwrap();
    ///
    /// assert_eq!(
    ///     trainer_name,
    ///     TrainerName {
    ///         name: ROMString::from("YOUNGSTER")
    ///     }
    /// );
    /// ```
    pub fn get_trainer_name(&self, trainer_id: &u8) -> Result<TrainerName> {
        let offset_base = (PkmnapiDB::ROM_PAGE * 0x1C) + 0x19FF;

        let (min_id, max_id) = self.trainer_id_validate(trainer_id)?;

        let offset = match {
            if trainer_id == &1 {
                Some(offset_base)
            } else {
                self.rom[offset_base..]
                    .iter()
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if *x == 0x50 {
                            return Some(offset_base + i + 1);
                        }

                        None
                    })
                    .take(max_id - 1)
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if (*trainer_id as usize) - 2 == i {
                            return Some(x);
                        }

                        None
                    })
                    .next()
            }
        } {
            Some(offset) => offset,
            None => return Err(error::Error::TrainerIDInvalid(*trainer_id, min_id, max_id)),
        };

        let trainer_name = TrainerName::from(&self.rom[offset..(offset + 13)]);

        Ok(trainer_name)
    }

    /// Set trainer name by trainer ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db
    ///     .set_trainer_name(
    ///         &1,
    ///         &TrainerName {
    ///             name: ROMString::from("ABCDEFGHI"),
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x399FF,
    ///         length: 0x09,
    ///         data: vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]
    ///     }
    /// );
    /// ```
    pub fn set_trainer_name(&self, trainer_id: &u8, trainer_name: &TrainerName) -> Result<Patch> {
        let (min_id, max_id) = self.trainer_id_validate(trainer_id)?;

        let old_trainer_name = self.get_trainer_name(trainer_id)?;
        let old_trainer_name_len = old_trainer_name.name.value.len();
        let trainer_name_raw = trainer_name.to_raw();
        let trainer_name_len = trainer_name_raw.len();

        if old_trainer_name_len != trainer_name_len {
            return Err(error::Error::TrainerNameWrongSize(
                old_trainer_name_len,
                trainer_name_len,
            ));
        }

        let offset_base = (PkmnapiDB::ROM_PAGE * 0x1C) + 0x19FF;

        let offset = match {
            if trainer_id == &1 {
                Some(offset_base)
            } else {
                self.rom[offset_base..]
                    .iter()
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if *x == 0x50 {
                            return Some(offset_base + i + 1);
                        }

                        None
                    })
                    .take(max_id - 1)
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if (*trainer_id as usize) - 2 == i {
                            return Some(x);
                        }

                        None
                    })
                    .next()
            }
        } {
            Some(offset) => offset,
            None => return Err(error::Error::TrainerIDInvalid(*trainer_id, min_id, max_id)),
        };

        Ok(Patch::new(&offset, &trainer_name_raw))
    }

    pub fn get_trainer_pic(&self, trainer_id: &u8) -> Result<Pic> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
        let offset_base = offset_base + 0x1914;

        let (_min_id, _max_id) = self.trainer_id_validate(trainer_id)?;

        let offset = offset_base + (((*trainer_id - 1) as usize) * 0x05);

        let pointer_base = PkmnapiDB::ROM_PAGE * 0x24;
        let pointer = pointer_base + {
            let mut cursor = Cursor::new(&self.rom[offset..(offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let pic = Pic::new(&self.rom[pointer..])?;

        Ok(pic)
    }

    pub fn set_trainer_pic(
        &self,
        trainer_id: &u8,
        pic: &Pic,
        encoding_method: PicEncodingMethod,
    ) -> Result<Patch> {
        let old_pixels = self.get_trainer_pic(trainer_id)?;
        let pixels = pic.encode(encoding_method);

        if pixels.len() > old_pixels.bytes + 1 {
            return Err(error::Error::PicTooLarge);
        }

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
        let offset_base = offset_base + 0x1914;
        let offset = offset_base + (((*trainer_id - 1) as usize) * 0x05);

        let pointer_base = PkmnapiDB::ROM_PAGE * 0x24;
        let pointer = pointer_base + {
            let mut cursor = Cursor::new(&self.rom[offset..(offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        Ok(Patch::new(&pointer, &pixels))
    }

    /// Get item name by item ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let item_name = db.get_item_name(&1).unwrap();
    ///
    /// assert_eq!(
    ///     item_name,
    ///     ItemName {
    ///         name: ROMString::from("MASTER BALL")
    ///     }
    /// );
    /// ```
    pub fn get_item_name(&self, item_id: &u8) -> Result<ItemName> {
        let (min_id, max_id) = self.item_id_validate(item_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x02;
        let offset_base = offset_base + 0x072B;

        let offset = match {
            if item_id == &1 {
                Some(offset_base)
            } else {
                self.rom[offset_base..]
                    .iter()
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if *x == 0x50 {
                            return Some(offset_base + i + 1);
                        }

                        None
                    })
                    .take(max_id - 1)
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if (*item_id as usize) - 2 == i {
                            return Some(x);
                        }

                        None
                    })
                    .next()
            }
        } {
            Some(offset) => offset,
            None => return Err(error::Error::ItemIDInvalid(*item_id, min_id, max_id)),
        };

        let item_name = ItemName::from(&self.rom[offset..(offset + 13)]);

        Ok(item_name)
    }

    /// Set item name by item ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db
    ///     .set_item_name(
    ///         &1,
    ///         &ItemName {
    ///             name: ROMString::from("CHEATERBALL"),
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x472B,
    ///         length: 0x0B,
    ///         data: vec![0x82, 0x87, 0x84, 0x80, 0x93, 0x84, 0x91, 0x81, 0x80, 0x8B, 0x8B]
    ///     }
    /// );
    /// ```
    pub fn set_item_name(&self, item_id: &u8, item_name: &ItemName) -> Result<Patch> {
        let (min_id, max_id) = self.item_id_validate(item_id)?;

        let old_item_name = self.get_item_name(item_id)?;
        let old_item_name_len = old_item_name.name.value.len();
        let item_name_raw = item_name.to_raw();
        let item_name_len = item_name_raw.len();

        if old_item_name_len != item_name_len {
            return Err(error::Error::ItemNameWrongSize(
                old_item_name_len,
                item_name_len,
            ));
        }

        let offset_base = PkmnapiDB::ROM_PAGE * 0x02;
        let offset_base = offset_base + 0x072B;

        let offset = match {
            if item_id == &1 {
                Some(offset_base)
            } else {
                self.rom[offset_base..]
                    .iter()
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if *x == 0x50 {
                            return Some(offset_base + i + 1);
                        }

                        None
                    })
                    .take(max_id - 1)
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if (*item_id as usize) - 2 == i {
                            return Some(x);
                        }

                        None
                    })
                    .next()
            }
        } {
            Some(offset) => offset,
            None => return Err(error::Error::ItemIDInvalid(*item_id, min_id, max_id)),
        };

        Ok(Patch::new(&offset, &item_name_raw))
    }

    pub fn get_map_pic(&self, map_id: &u8) -> Result<Map> {
        let (_min_id, _max_id) = self.map_id_validate(map_id)?;

        let bank_offset_base = PkmnapiDB::ROM_PAGE * 0x06;
        let bank_offset = (bank_offset_base + 0x23D) + (*map_id as usize);
        let bank_id = self.rom[bank_offset];

        let bank = ((bank_id as usize) - 0x01) * (PkmnapiDB::ROM_PAGE * 0x02);

        if bank == 0x00 {
            return Err(error::Error::MapInvalid(*map_id));
        }

        let header_offset = 0x01AE + ((*map_id as usize) * 0x02);
        let header_pointer = bank + {
            let mut cursor = Cursor::new(&self.rom[header_offset..(header_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let tileset = self.rom[header_pointer];

        if tileset == 0x40 || tileset == 0xC7 {
            return Err(error::Error::MapInvalid(*map_id));
        }

        let tileset_bank_pointer = 0xC7BE + ((tileset as usize) * 0x0C);
        let tileset_bank =
            ((self.rom[tileset_bank_pointer] as usize) - 0x01) * (PkmnapiDB::ROM_PAGE * 0x02);
        let tileset_block_pointer = tileset_bank + {
            let mut cursor =
                Cursor::new(&self.rom[(tileset_bank_pointer + 1)..(tileset_bank_pointer + 3)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };
        let tileset_graphics_pointer = tileset_bank + {
            let mut cursor =
                Cursor::new(&self.rom[(tileset_bank_pointer + 3)..(tileset_bank_pointer + 5)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let height = self.rom[header_pointer + 1] as u32;
        let width = self.rom[header_pointer + 2] as u32;

        if width >= 0xF6 || height >= 0xF0 {
            return Err(error::Error::MapInvalid(*map_id));
        }

        let blocks_pointer = bank + {
            let mut cursor = Cursor::new(&self.rom[(header_pointer + 3)..(header_pointer + 5)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let blocks_data =
            self.rom[blocks_pointer..(blocks_pointer + ((width * height) as usize))].to_vec();

        let _text_pointer = bank + {
            let mut cursor = Cursor::new(&self.rom[(header_pointer + 5)..(header_pointer + 7)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let _script_pointer = bank + {
            let mut cursor = Cursor::new(&self.rom[(header_pointer + 7)..(header_pointer + 9)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let _connections = self.rom[header_pointer + 9];

        let object_pointer = bank + {
            let mut cursor = Cursor::new(&self.rom[(header_pointer + 10)..(header_pointer + 12)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let _border_block = self.rom[object_pointer];

        let tiles: Vec<Vec<u8>> = (0..(16 * 6 * 8 * 8))
            .map(|tile_id| {
                let tile_offset = tileset_graphics_pointer + (tile_id * 0x10);

                self.rom[tile_offset..(tile_offset + 0x10)]
                    .to_vec()
                    .chunks(2)
                    .map(|chunk| {
                        let hi_byte =
                            (0..8).map(|bit| (chunk[0] & (0x01 << (7 - bit))) >> (7 - bit));
                        let lo_byte =
                            (0..8).map(|bit| (chunk[1] & (0x01 << (7 - bit))) >> (7 - bit));

                        hi_byte
                            .zip(lo_byte)
                            .map(|(hi_bit, lo_bit)| (hi_bit << 0x01) | lo_bit)
                            .collect::<Vec<u8>>()
                    })
                    .flatten()
                    .collect()
            })
            .collect();

        let map_tiles: Vec<Vec<u8>> = (0..(width * height * 4 * 4))
            .map(|i| {
                let x = i % (width * 4);
                let y = ((i as f32) / (width * 4) as f32) as u32;
                let block_index =
                    (((((y as f32) / 4.0) as u32) * width) + (((x as f32) / 4.0) as u32)) as usize;
                let block = blocks_data[block_index] as usize;
                let block_tile_index = ((i % 4) + ((y % 4) * 4)) as usize;

                let tile_id =
                    (self.rom[tileset_block_pointer + (block * 0x10) + block_tile_index]) as usize;

                tiles[tile_id].to_vec()
            })
            .collect();

        let map = Map::new(&width, &height, &map_tiles)?;

        Ok(map)
    }

    pub fn get_trainer_parties(&self, trainer_id: &u8) -> Result<Vec<Party>> {
        let (min_id, max_id) = self.trainer_id_validate(trainer_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
        let offset = offset_base + 0x1D3B;

        let pointer_min_offset = offset + ((*trainer_id as usize) - 1) * 0x02;
        let pointer_min = (offset_base - (PkmnapiDB::ROM_PAGE * 2)) + {
            let mut cursor = Cursor::new(&self.rom[pointer_min_offset..(pointer_min_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let pointer_max_offset = offset + (*trainer_id as usize) * 0x02;
        let pointer_max = (offset_base - (PkmnapiDB::ROM_PAGE * 2)) + {
            let mut cursor = Cursor::new(&self.rom[pointer_max_offset..(pointer_max_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let data_size = if trainer_id == &(max_id as u8) {
            self.rom[pointer_min..]
                .iter()
                .position(|r| r == &0x00)
                .unwrap()
                + 0x01
        } else {
            pointer_max - pointer_min
        };

        let trainer_party_offsets: Vec<usize> = [
            vec![0x00],
            self.rom[pointer_min..(pointer_min + data_size)]
                .iter()
                .enumerate()
                .filter_map(|(i, x)| {
                    let offset = i + 1;

                    if offset == data_size {
                        return None;
                    }

                    if x == &0x00 {
                        Some(i + 1)
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>(),
        ]
        .concat();

        if data_size == 0x00 {
            return Err(error::Error::TrainerIDInvalid(*trainer_id, min_id, max_id));
        }

        let trainer_parties: Vec<Party> = trainer_party_offsets
            .iter()
            .map(|trainer_party_offset| {
                let mut party = Party::from(
                    &self.rom[(pointer_min + trainer_party_offset)..(pointer_min + data_size)],
                );

                party.pokemon = party
                    .pokemon
                    .iter()
                    .map(|party_pokemon| {
                        PartyPokemon::new(
                            party_pokemon.level,
                            self.internal_id_to_pokedex_id(&party_pokemon.internal_id)
                                .unwrap(),
                        )
                    })
                    .collect();

                party
            })
            .collect();

        Ok(trainer_parties)
    }

    pub fn set_trainer_parties(
        &self,
        trainer_id: &u8,
        trainer_parties: &Vec<Party>,
    ) -> Result<Patch> {
        let old_trainer_parties = self.get_trainer_parties(trainer_id)?;
        let old_trainer_parties_len = old_trainer_parties.len();
        let old_trainer_parties_data: Vec<u8> = old_trainer_parties
            .iter()
            .map(|old_trainer_party| old_trainer_party.to_raw())
            .flatten()
            .collect();
        let old_trainer_parties_data_len = old_trainer_parties_data.len();
        let trainer_parties_len = trainer_parties.len();
        let trainer_parties_data: Vec<u8> = trainer_parties
            .iter()
            .map(|trainer_party| {
                let new_trainer_party = Party {
                    level_type: trainer_party.level_type,
                    pokemon: trainer_party
                        .pokemon
                        .iter()
                        .map(|pokemon| PartyPokemon {
                            level: pokemon.level,
                            pokedex_id: pokemon.pokedex_id,
                            internal_id: self
                                .pokedex_id_to_internal_id(&pokemon.pokedex_id)
                                .unwrap()
                                + 1,
                        })
                        .collect(),
                };

                new_trainer_party.to_raw()
            })
            .flatten()
            .collect();
        let trainer_parties_data_len = trainer_parties_data.len();

        if old_trainer_parties_len != trainer_parties_len {
            return Err(error::Error::TrainerPartiesWrongSize(
                old_trainer_parties_len,
                trainer_parties_len,
            ));
        }

        if old_trainer_parties_data_len != trainer_parties_data_len {
            return Err(error::Error::TrainerPartiesWrongDataSize(
                old_trainer_parties_data_len,
                trainer_parties_data_len,
            ));
        }

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
        let offset = offset_base + 0x1D3B;

        let pointer_offset = offset + ((*trainer_id as usize) - 1) * 0x02;
        let pointer = (offset_base - (PkmnapiDB::ROM_PAGE * 2)) + {
            let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        Ok(Patch::new(&pointer, &trainer_parties_data))
    }

    /// Get title Pokédex IDs
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let pokemon_title = db.get_pokemon_title().unwrap();
    ///
    /// // RED
    /// # #[cfg(feature = "PKMN_RED")]
    /// assert_eq!(
    ///     pokemon_title,
    ///     vec![
    ///         0xB0,
    ///         0xB1,
    ///         0x99,
    ///         0x70,
    ///         0x03,
    ///         0x1A,
    ///         0x54,
    ///         0x04,
    ///         0x01,
    ///         0x94,
    ///         0x19,
    ///         0x4C,
    ///         0x96,
    ///         0x22,
    ///         0xA3,
    ///         0x85,
    ///     ]
    /// );
    ///
    /// // BLUE
    /// # #[cfg(not(feature = "PKMN_RED"))]
    /// assert_eq!(
    ///     pokemon_title,
    ///     vec![
    ///         0xB1,
    ///         0xB0,
    ///         0x99,
    ///         0x39,
    ///         0x2B,
    ///         0x52,
    ///         0x28,
    ///         0xAB,
    ///         0x68,
    ///         0x84,
    ///         0xBA,
    ///         0x47,
    ///         0x46,
    ///         0xAA,
    ///         0x0E,
    ///         0x55
    ///     ]
    /// );
    /// ```
    pub fn get_pokemon_title(&self) -> Result<Vec<u8>> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x02;
        let offset = offset_base + 0x0588;

        let pokemon_title = self.rom[offset..(offset + 0x10)].to_vec();

        Ok(pokemon_title)
    }

    /// Set title Pokédex IDs
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db.set_pokemon_title(&vec![
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    /// ]).unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x4588,
    ///         length: 0x10,
    ///         data: vec![0x85, 0x85, 0x85, 0x85, 0x85, 0x85, 0x85, 0x85, 0x85, 0x85, 0x85, 0x85, 0x85, 0x85, 0x85, 0x85]
    ///     }
    /// );
    /// ```
    pub fn set_pokemon_title(&self, pokemon_title: &Vec<u8>) -> Result<Patch> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x02;
        let offset = offset_base + 0x0588;

        let data = pokemon_title.to_vec();
        let data_len = data.len();
        let max_len = 0x10usize;

        if data_len != max_len {
            return Err(error::Error::PokemonTitleWrongSize(max_len, data_len));
        }

        Ok(Patch::new(&offset, &data))
    }

    /// Get Pokémon evolutions by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::types::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let pokemon_evolutions = db.get_pokemon_evolutions(&1).unwrap();
    ///
    /// assert_eq!(
    ///     pokemon_evolutions,
    ///     vec![
    ///         PokemonEvolutionLevel::new(2, 16)
    ///     ]
    /// );
    /// ```
    pub fn get_pokemon_evolutions(&self, pokedex_id: &u8) -> Result<Vec<PokemonEvolution>> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x1D;
        let offset = offset_base + 0x105C;

        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let pointer_offset = offset + ((internal_id as usize) * 0x02);
        let pointer = (offset_base - (PkmnapiDB::ROM_PAGE * 0x03)) + {
            let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let evolution_data = self.rom[pointer..(pointer + 0x0D)].to_vec();
        let mut pokemon_evolutions = vec![];
        let mut i = 0;

        while i < evolution_data.len() {
            let id = evolution_data[i];

            if id == 0x00 {
                break;
            }

            pokemon_evolutions.push(PokemonEvolution::from(&evolution_data[i..(i + 4)]));

            i = match id {
                0x01 => i + 3,
                0x02 => i + 4,
                0x03 => i + 3,
                _ => unreachable!(),
            };
        }

        let pokemon_evolutions = pokemon_evolutions
            .iter()
            .map(|pokemon_evolution| match pokemon_evolution {
                PokemonEvolution::LEVEL(evolution) => {
                    PokemonEvolution::LEVEL(PokemonEvolutionLevel {
                        pokedex_id: self
                            .internal_id_to_pokedex_id(&evolution.internal_id)
                            .unwrap(),
                        internal_id: 0,
                        ..*evolution
                    })
                }
                PokemonEvolution::ITEM(evolution) => PokemonEvolution::ITEM(PokemonEvolutionItem {
                    pokedex_id: self
                        .internal_id_to_pokedex_id(&evolution.internal_id)
                        .unwrap(),
                    internal_id: 0,
                    ..*evolution
                }),
                PokemonEvolution::TRADE(evolution) => {
                    PokemonEvolution::TRADE(PokemonEvolutionTrade {
                        pokedex_id: self
                            .internal_id_to_pokedex_id(&evolution.internal_id)
                            .unwrap(),
                        internal_id: 0,
                        ..*evolution
                    })
                }
            })
            .collect();

        Ok(pokemon_evolutions)
    }

    /// Set Pokémon evolutions by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db.set_pokemon_evolutions(&1, &vec![
    ///     PokemonEvolutionLevel::new(2, 16)
    /// ]).unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x3B844,
    ///         length: 0x03,
    ///         data: vec![0x01, 0x10, 0x09]
    ///     }
    /// );
    /// ```
    pub fn set_pokemon_evolutions(
        &self,
        pokedex_id: &u8,
        pokemon_evolutions: &Vec<PokemonEvolution>,
    ) -> Result<Patch> {
        let old_pokemon_evolutions = self.get_pokemon_evolutions(pokedex_id)?;
        let old_pokemon_evolutions_data: Vec<u8> = old_pokemon_evolutions
            .iter()
            .map(|pokemon_evolution| pokemon_evolution.to_raw())
            .flatten()
            .collect();
        let old_pokemon_evolutions_data_len = old_pokemon_evolutions_data.len();

        let pokemon_evolutions_data: Vec<u8> = pokemon_evolutions
            .iter()
            .map(|pokemon_evolution| {
                let pokemon_evolution = match pokemon_evolution {
                    PokemonEvolution::LEVEL(evolution) => {
                        PokemonEvolution::LEVEL(PokemonEvolutionLevel {
                            internal_id: self
                                .pokedex_id_to_internal_id(&evolution.pokedex_id)
                                .unwrap(),
                            ..*evolution
                        })
                    }
                    PokemonEvolution::ITEM(evolution) => {
                        PokemonEvolution::ITEM(PokemonEvolutionItem {
                            internal_id: self
                                .pokedex_id_to_internal_id(&evolution.pokedex_id)
                                .unwrap(),
                            ..*evolution
                        })
                    }
                    PokemonEvolution::TRADE(evolution) => {
                        PokemonEvolution::TRADE(PokemonEvolutionTrade {
                            internal_id: self
                                .pokedex_id_to_internal_id(&evolution.pokedex_id)
                                .unwrap(),
                            ..*evolution
                        })
                    }
                };

                pokemon_evolution.to_raw()
            })
            .flatten()
            .collect();
        let pokemon_evolutions_data_len = pokemon_evolutions_data.len();

        if old_pokemon_evolutions_data_len != pokemon_evolutions_data_len {
            return Err(error::Error::PokemonEvolutionWrongSize(
                old_pokemon_evolutions_data_len,
                pokemon_evolutions_data_len,
            ));
        }

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1D;
        let offset = offset_base + 0x105C;

        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let pointer_offset = offset + ((internal_id as usize) * 0x02);
        let pointer = (offset_base - (PkmnapiDB::ROM_PAGE * 0x03)) + {
            let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        Ok(Patch::new(&pointer, &pokemon_evolutions_data))
    }

    pub fn get_pokemon_cry(&self, pokedex_id: &u8) -> Result<Cry> {
        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
        let offset = (offset_base + 0x1446) + ((internal_id as usize) * 0x03);

        let base = self.rom[offset];
        let pitch = self.rom[offset + 1];
        let length = self.rom[offset + 2];

        let offset_base = PkmnapiDB::ROM_PAGE * 0x04;
        let offset = (offset_base + 0x3C) + ((base as usize) * 0x09);

        let cry: Cry = (0..3)
            .map(|i| {
                let cursor_offset = (offset + (i * 3)) + 1;
                let mut cursor = Cursor::new(&self.rom[cursor_offset..(cursor_offset + 2)]);

                cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
            })
            .map(|channel_offset| {
                let offset_base = PkmnapiDB::ROM_PAGE * 0x02;
                let offset = offset_base + channel_offset;

                self.rom[offset..]
                    .iter()
                    .take_while(|&x| *x != 0xFF)
                    .map(|x| *x)
                    .collect::<Vec<u8>>()
            })
            .enumerate()
            .fold(
                Cry {
                    base,
                    pitch,
                    length,
                    ..Default::default()
                },
                |acc, (i, channel_data)| match i {
                    0 => Cry {
                        pulse0: CryChannel::new(&channel_data, false),
                        ..acc
                    },
                    1 => Cry {
                        pulse1: CryChannel::new(&channel_data, false),
                        ..acc
                    },
                    _ => Cry {
                        noise: CryChannel::new(&channel_data, true),
                        ..acc
                    },
                },
            );

        Ok(cry)
    }

    pub fn set_pokemon_cry(&self, pokedex_id: &u8, pokemon_cry: &Cry) -> Result<Patch> {
        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
        let offset = (offset_base + 0x1446) + ((internal_id as usize) * 0x03);

        let pokemon_cry_data = pokemon_cry.to_raw();

        Ok(Patch::new(&offset, &pokemon_cry_data))
    }

    pub fn get_map_pokemon(&self, map_id: &u8) -> Result<MapPokemon> {
        let (_min_id, _max_id) = self.map_id_validate(map_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x06;
        let offset = offset_base + 0x0EEB;
        let pointer_offset = offset + ((*map_id as usize) * 0x02);
        let pointer = offset_base - (PkmnapiDB::ROM_PAGE * 0x02) + {
            let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let map_pokemon = MapPokemon::from(&self.rom[pointer..]);
        let map_pokemon = MapPokemon {
            grass: MapPokemonArea {
                pokemon: map_pokemon
                    .grass
                    .pokemon
                    .iter()
                    .map(|map_pokemon_info| MapPokemonInfo {
                        pokedex_id: self
                            .internal_id_to_pokedex_id(&map_pokemon_info.internal_id)
                            .unwrap(),
                        internal_id: 0,
                        ..*map_pokemon_info
                    })
                    .collect(),
                ..map_pokemon.grass
            },
            water: MapPokemonArea {
                pokemon: map_pokemon
                    .water
                    .pokemon
                    .iter()
                    .map(|map_pokemon_info| MapPokemonInfo {
                        pokedex_id: self
                            .internal_id_to_pokedex_id(&map_pokemon_info.internal_id)
                            .unwrap(),
                        internal_id: 0,
                        ..*map_pokemon_info
                    })
                    .collect(),
                ..map_pokemon.water
            },
        };

        Ok(map_pokemon)
    }

    pub fn set_map_pokemon(&self, map_id: &u8, map_pokemon: &MapPokemon) -> Result<Patch> {
        let old_map_pokemon = self.get_map_pokemon(map_id)?;
        let old_map_pokemon_data = old_map_pokemon.to_raw();
        let old_map_pokemon_data_len = old_map_pokemon_data.len();
        let map_pokemon_data = {
            MapPokemon {
                grass: MapPokemonArea {
                    pokemon: map_pokemon
                        .grass
                        .pokemon
                        .iter()
                        .map(|pokemon| MapPokemonInfo {
                            internal_id: self
                                .pokedex_id_to_internal_id(&pokemon.pokedex_id)
                                .unwrap(),
                            ..*pokemon
                        })
                        .collect(),
                    ..map_pokemon.grass
                },
                water: MapPokemonArea {
                    pokemon: map_pokemon
                        .water
                        .pokemon
                        .iter()
                        .map(|pokemon| MapPokemonInfo {
                            internal_id: self
                                .pokedex_id_to_internal_id(&pokemon.pokedex_id)
                                .unwrap(),
                            ..*pokemon
                        })
                        .collect(),
                    ..map_pokemon.water
                },
            }
        }
        .to_raw();
        let map_pokemon_data_len = map_pokemon_data.len();

        if old_map_pokemon_data_len != map_pokemon_data_len {
            return Err(error::Error::MapPokemonWrongSize(
                old_map_pokemon_data_len,
                map_pokemon_data_len,
            ));
        }

        let offset_base = PkmnapiDB::ROM_PAGE * 0x06;
        let offset = offset_base + 0x0EEB;
        let pointer_offset = offset + ((*map_id as usize) * 0x02);
        let pointer = offset_base - (PkmnapiDB::ROM_PAGE * 0x02) + {
            let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        Ok(Patch::new(&pointer, &map_pokemon_data))
    }

    pub fn get_pokemon_logo_img(&self) -> Result<Img> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x08;
        let offset = offset_base + 0x1380;

        let tiles: Vec<Vec<u8>> = (0..(16 * 7))
            .map(|tile_id| {
                let tile_offset = offset + (tile_id * 0x10);

                self.rom[tile_offset..(tile_offset + 0x10)]
                    .to_vec()
                    .chunks(2)
                    .map(|chunk| {
                        let hi_byte =
                            (0..8).map(|bit| (chunk[1] & (0x01 << (7 - bit))) >> (7 - bit));
                        let lo_byte =
                            (0..8).map(|bit| (chunk[0] & (0x01 << (7 - bit))) >> (7 - bit));

                        hi_byte
                            .zip(lo_byte)
                            .map(|(hi_bit, lo_bit)| (hi_bit << 0x01) | lo_bit)
                            .collect::<Vec<u8>>()
                    })
                    .flatten()
                    .collect()
            })
            .collect();

        let pokemon_logo = Img::new(&16, &7, &tiles)?;

        Ok(pokemon_logo)
    }

    pub fn set_pokemon_logo_img(&self, pokemon_logo: &Img) -> Result<Patch> {
        let old_pokemon_logo = self.get_pokemon_logo_img()?;
        let old_pokemon_logo_data = old_pokemon_logo.to_2bpp()?;
        let old_pokemon_logo_data_len = old_pokemon_logo_data.len();
        let pokemon_logo_data = pokemon_logo.to_2bpp()?;
        let pokemon_logo_data_len = pokemon_logo_data.len();

        if old_pokemon_logo_data_len != pokemon_logo_data_len {
            return Err(error::Error::PokemonLogoWrongSize(
                old_pokemon_logo_data_len,
                pokemon_logo_data_len,
            ));
        }

        let offset_base = PkmnapiDB::ROM_PAGE * 0x08;
        let offset = offset_base + 0x1380;

        Ok(Patch::new(&offset, &pokemon_logo_data))
    }

    pub fn get_town_map_img(&self) -> Result<Img> {
        let graphics_offset_base = PkmnapiDB::ROM_PAGE * 0x09;
        let graphics_offset = graphics_offset_base + 0x05A8;

        let graphics_tiles: Vec<Vec<u8>> = (0..(4 * 4))
            .map(|tile_id| {
                let tile_offset = graphics_offset + (tile_id * 0x10);

                self.rom[tile_offset..(tile_offset + 0x10)]
                    .to_vec()
                    .chunks(2)
                    .map(|chunk| {
                        let hi_byte =
                            (0..8).map(|bit| (chunk[0] & (0x01 << (7 - bit))) >> (7 - bit));
                        let lo_byte =
                            (0..8).map(|bit| (chunk[1] & (0x01 << (7 - bit))) >> (7 - bit));

                        hi_byte
                            .zip(lo_byte)
                            .map(|(hi_bit, lo_bit)| (hi_bit << 0x01) | lo_bit)
                            .collect::<Vec<u8>>()
                    })
                    .flatten()
                    .collect()
            })
            .collect();

        let offset_base = PkmnapiDB::ROM_PAGE * 0x38;
        let offset = offset_base + 0x1100;

        let tiles: Vec<Vec<u8>> = self.rom[offset..]
            .iter()
            .take_while(|&x| *x != 0x00)
            .map(|byte| {
                let byte = *byte as usize;
                let tile_id = (byte & 0xF0) >> 0x04;
                let count = byte & 0x0F;

                vec![tile_id; count]
            })
            .flatten()
            .map(|tile_id| graphics_tiles[tile_id].to_vec())
            .collect::<Vec<Vec<u8>>>();

        let town_map = Img::new(&20, &18, &tiles)?;

        Ok(town_map)
    }

    pub fn get_player_names(&self) -> Result<PlayerNames> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x03;
        let offset = offset_base + 0x0AA8;

        let player_names = PlayerNames::from(&self.rom[offset..]);

        Ok(player_names)
    }

    pub fn set_player_names(&self, player_names: &PlayerNames) -> Result<Patch> {
        let old_player_names = self.get_player_names()?;
        let old_player_names_data = old_player_names.to_raw();
        let old_player_names_data_len = old_player_names_data.len();
        let player_names_data_a = player_names.to_raw();
        let player_names_data_len = player_names_data_a.len();

        if old_player_names_data_len != player_names_data_len {
            return Err(error::Error::PlayerNamesWrongSize(
                old_player_names_data_len,
                player_names_data_len,
            ));
        }

        let offset_base = PkmnapiDB::ROM_PAGE * 0x03;
        let offset_a = offset_base + 0x0AA8;
        let offset_b = offset_base + 0x0AF2;
        let offset_raw_start = offset_a + player_names_data_len;
        let offset_raw_len = offset_b - offset_a - player_names_data_len;

        let player_names_data_b: Vec<u8> = player_names_data_a
            .iter()
            .map(|&x| {
                if x == 0x4E {
                    return 0x50;
                }

                return x;
            })
            .collect();

        let player_names_data = [
            player_names_data_a,
            self.rom[offset_raw_start..(offset_raw_start + offset_raw_len)].to_vec(),
            player_names_data_b,
        ]
        .concat();

        Ok(Patch::new(&offset_a, &player_names_data))
    }

    /// Get Pokémon icon by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let pokemon_icon = db.get_pokemon_icon(&1).unwrap();
    ///
    /// assert_eq!(
    ///     pokemon_icon,
    ///     PokemonIcon {
    ///         icon_id: 0x07
    ///     }
    /// );
    /// ```
    pub fn get_pokemon_icon(&self, pokedex_id: &u8) -> Result<PokemonIcon> {
        let _internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x38;
        let offset = (offset_base + 0x190D) + ((((*pokedex_id - 1) as f32) / 2.0).floor() as usize);

        let icon_id = if pokedex_id % 2 == 0 {
            self.rom[offset] & 0x0F
        } else {
            (self.rom[offset] & 0xF0) >> 0x04
        };

        let pokemon_icon = PokemonIcon::from(&icon_id);

        Ok(pokemon_icon)
    }

    /// Set Pokémon icon by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db
    ///     .set_pokemon_icon(
    ///         &1,
    ///         &PokemonIcon::from(&0x02)
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x7190D,
    ///         length: 0x01,
    ///         data: vec![0x27]
    ///     }
    /// );
    /// ```
    pub fn set_pokemon_icon(&self, pokedex_id: &u8, pokemon_icon: &PokemonIcon) -> Result<Patch> {
        let _internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x38;
        let offset = (offset_base + 0x190D) + ((((*pokedex_id - 1) as f32) / 2.0).floor() as usize);

        let data = if pokedex_id % 2 == 0 {
            vec![(self.rom[offset] & 0xF0) | pokemon_icon.value()]
        } else {
            vec![(self.rom[offset] & 0x0F) | (pokemon_icon.value() << 0x04)]
        };

        Ok(Patch::new(&offset, &data))
    }

    fn get_icon_frame(&self, icon_id: &u8, frame_index: &u8) -> Result<Img> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x38;
        let offset = offset_base + 0x17C0;

        let frame_index = cmp::min(*frame_index as usize, 1);

        let icon_data: Vec<(usize, usize)> = (0..28)
            .map(|i| {
                let data_offset = offset + (i * 0x06);

                let tile_count = self.rom[data_offset + 2] as usize;
                let bank = self.rom[data_offset + 3] as usize;
                let pointer = (bank * (PkmnapiDB::ROM_PAGE * 2)) - (PkmnapiDB::ROM_PAGE * 2) + {
                    let mut cursor = Cursor::new(&self.rom[data_offset..(data_offset + 2)]);

                    cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
                };

                (pointer, tile_count)
            })
            .collect();

        let icon_datum = if *icon_id == 2 {
            vec![((PkmnapiDB::ROM_PAGE * 0x08) + 0x1180, 4)]
        } else if *icon_id < 6 {
            let icon_data_index = if *icon_id >= 3 {
                (*icon_id as usize) - 1
            } else {
                *icon_id as usize
            } + (frame_index * 14);

            vec![icon_data[icon_data_index]]
        } else {
            let icon_data_index = (5 + (((*icon_id as usize) - 6) * 2)) + (frame_index * 14);

            vec![icon_data[icon_data_index], icon_data[icon_data_index + 1]]
        };

        let mut tiles: Vec<Vec<u8>> = icon_datum
            .iter()
            .map(|datum| {
                let (pointer, tile_count) = datum;

                (0..*tile_count)
                    .map(|tile_id| {
                        let tile_offset = pointer + (tile_id * 0x10);

                        self.rom[tile_offset..(tile_offset + 0x10)]
                            .to_vec()
                            .chunks(2)
                            .map(|chunk| {
                                let hi_byte =
                                    (0..8).map(|bit| (chunk[1] & (0x01 << (7 - bit))) >> (7 - bit));
                                let lo_byte =
                                    (0..8).map(|bit| (chunk[0] & (0x01 << (7 - bit))) >> (7 - bit));

                                hi_byte
                                    .zip(lo_byte)
                                    .map(|(hi_bit, lo_bit)| (hi_bit << 0x01) | lo_bit)
                                    .collect::<Vec<u8>>()
                            })
                            .flatten()
                            .collect()
                    })
                    .collect::<Vec<Vec<u8>>>()
            })
            .flatten()
            .collect();

        if *icon_id <= 5 {
            if *icon_id != 2 {
                tiles[1] = tiles[0]
                    .chunks(8)
                    .map(|chunk| {
                        let mut chunk = chunk.to_vec();

                        chunk.reverse();

                        chunk
                    })
                    .flatten()
                    .collect();

                tiles[3] = tiles[2]
                    .chunks(8)
                    .map(|chunk| {
                        let mut chunk = chunk.to_vec();

                        chunk.reverse();

                        chunk
                    })
                    .flatten()
                    .collect();
            }
        } else {
            tiles = vec![
                tiles[0].to_vec(),
                tiles[0]
                    .chunks(8)
                    .map(|chunk| {
                        let mut chunk = chunk.to_vec();

                        chunk.reverse();

                        chunk
                    })
                    .flatten()
                    .collect(),
                tiles[1].to_vec(),
                tiles[1]
                    .chunks(8)
                    .map(|chunk| {
                        let mut chunk = chunk.to_vec();

                        chunk.reverse();

                        chunk
                    })
                    .flatten()
                    .collect(),
            ];
        }

        let mut icon = Img::new(&2, &2, &tiles)?;

        if [1, 2].contains(icon_id) && frame_index == 1 {
            icon.pixels = [icon.pixels[16..].to_vec(), icon.pixels[..16].to_vec()].concat();
        }

        Ok(icon)
    }

    pub fn get_icon(&self, icon_id: &u8) -> Result<Gif> {
        let (_min_id, _max_id) = self.icon_id_validate(icon_id)?;

        let frame_a = self.get_icon_frame(icon_id, &0)?;
        let frame_b = self.get_icon_frame(icon_id, &1)?;

        let gif = Gif::new(&vec![frame_a, frame_b]);

        Ok(gif)
    }
}
