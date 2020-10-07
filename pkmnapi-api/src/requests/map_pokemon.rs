use pkmnapi_db::types::{MapPokemonArea, MapPokemonInfo};
use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type MapPokemonRequest = BaseRequest<MapPokemonRequestType, MapPokemonRequestAttributes>;

impl MapPokemonRequest {
    pub fn get_grass(&self) -> MapPokemonArea {
        MapPokemonArea {
            encounter_rate: self.data.attributes.grass.encounter_rate,
            pokemon: self
                .data
                .attributes
                .grass
                .pokemon
                .iter()
                .map(|pokemon| MapPokemonInfo::new(pokemon.level, pokemon.pokemon.id))
                .collect(),
        }
    }

    pub fn get_water(&self) -> MapPokemonArea {
        MapPokemonArea {
            encounter_rate: self.data.attributes.water.encounter_rate,
            pokemon: self
                .data
                .attributes
                .water
                .pokemon
                .iter()
                .map(|pokemon| MapPokemonInfo::new(pokemon.level, pokemon.pokemon.id))
                .collect(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum MapPokemonRequestType {
    map_pokemon,
}

#[derive(Debug, Deserialize)]
pub struct MapPokemonRequestAttributes {
    pub grass: MapPokemonRequestAttributesArea,
    pub water: MapPokemonRequestAttributesArea,
}

#[derive(Debug, Deserialize)]
pub struct MapPokemonRequestAttributesArea {
    pub encounter_rate: u8,
    pub pokemon: Vec<MapPokemonRequestAttributesPokemon>,
}

#[derive(Debug, Deserialize)]
pub struct MapPokemonRequestAttributesPokemon {
    pub level: u8,
    pub pokemon: MapPokemonRequestAttributesPokemonInfo,
}

#[derive(Debug, Deserialize)]
pub struct MapPokemonRequestAttributesPokemonInfo {
    #[serde(deserialize_with = "crate::utils::from_numeric_str")]
    pub id: u8,
}
