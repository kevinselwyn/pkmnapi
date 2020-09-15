use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    HeaderParseError(String),
    HeaderTooSmall,
    HMIDInvalid(u8, usize, usize),
    InternalIDInvalid(u8),
    ItemIDInvalid(u8),
    ItemNameWrongSize(usize, usize),
    MoveIDInvalid(u8),
    MoveNameWrongSize(usize, usize),
    PicCouldNotRead,
    PicCouldNotWrite,
    PicTooLarge,
    PicWrongSize,
    PokedexEntrySpeciesWrongSize(usize, usize),
    PokedexEntryTextWrongSize(usize, usize),
    PokedexIDInvalid(u8),
    SavBagItemsWrongSize(usize, usize),
    SavBoxItemsWrongSize(usize, usize),
    SavPlayerNameWrongSize(usize, usize),
    SavRivalNameWrongSize(usize, usize),
    SavWrongSize(usize, usize),
    TMIDInvalid(u8, usize, usize),
    TrainerIDInvalid(u8),
    TrainerNameWrongSize(usize, usize),
    TypeEffectIDInvalid(u8, usize, usize),
    TypeIDInvalid(u8, usize, usize),
    TypeNameWrongSize(usize, usize),
}

impl fmt::Display for Error {
    /// Converts the error to a String
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::error;
    ///
    /// let err = error::Error::HeaderParseError("foo".to_owned());
    /// let string = err.to_string();
    ///
    /// assert_eq!(string, "foo");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Error::HeaderParseError(string) => string.to_owned(),
            Error::HeaderTooSmall => "Header too small".to_owned(),
            Error::HMIDInvalid(hm_id, min, max) => {
                format!("Invalid HM ID {}: valid range is {}-{}", hm_id, min, max)
            }
            Error::InternalIDInvalid(internal_id) => {
                format!("Invalid internal ID: {}", internal_id)
            }
            Error::ItemIDInvalid(item_id) => format!("Invalid item ID: {}", item_id),
            Error::MoveIDInvalid(move_id) => format!("Invalid move ID: {}", move_id),
            Error::PokedexIDInvalid(pokedex_id) => format!("Invalid Pokédex ID: {}", pokedex_id),
            Error::TMIDInvalid(tm_id, min, max) => {
                format!("Invalid TM ID {}: valid range is {}-{}", tm_id, min, max)
            }
            Error::TrainerIDInvalid(item_id) => format!("Invalid trainer ID: {}", item_id),
            Error::TypeEffectIDInvalid(type_effect_id, min, max) => format!(
                "Invalid type effect ID {}: valid range is {}-{}",
                type_effect_id, min, max
            ),
            Error::TypeIDInvalid(type_id, min, max) => format!(
                "Invalid type ID {}: valid range is {}-{}",
                type_id, min, max
            ),
            Error::ItemNameWrongSize(expected, actual) => format!(
                "Item name length mismatch: should be exactly {} characters, found {}",
                expected, actual
            ),
            Error::MoveNameWrongSize(expected, actual) => format!(
                "Move name length mismatch: should be exactly {} characters, found {}",
                expected, actual
            ),
            Error::PicCouldNotRead => "Could not read image".to_owned(),
            Error::PicCouldNotWrite => "Could not write image".to_owned(),
            Error::PicTooLarge => "Compressed image is too large".to_owned(),
            Error::PicWrongSize => "Image dimensions must be multiples of 8".to_owned(),
            Error::PokedexEntrySpeciesWrongSize(expected, actual) => format!(
                "Pokédex entry species length mismatch: should be exactly {} characters, found {}",
                expected, actual
            ),
            Error::PokedexEntryTextWrongSize(expected, actual) => format!(
                "Pokédex entry text length mismatch: should be {} characters or fewer, found {}",
                expected, actual
            ),
            Error::SavBagItemsWrongSize(expected, actual) => format!(
                "Sav bag items length mismatch: should be {} items or fewer, found {}",
                expected, actual
            ),
            Error::SavBoxItemsWrongSize(expected, actual) => format!(
                "Sav box items length mismatch: should be {} items or fewer, found {}",
                expected, actual
            ),
            Error::SavPlayerNameWrongSize(expected, actual) => format!(
                "Player name length mismatch: should be {} characters or fewer, found {}",
                expected, actual
            ),
            Error::SavRivalNameWrongSize(expected, actual) => format!(
                "Sav rival name length mismatch: should be {} characters or fewer, found {}",
                expected, actual
            ),
            Error::SavWrongSize(expected, actual) => format!(
                "Sav length mismatch: should be {} bytes, found {}",
                expected, actual
            ),
            Error::TrainerNameWrongSize(expected, actual) => format!(
                "Trainer name length mismatch: should be exactly {} characters, found {}",
                expected, actual
            ),
            Error::TypeNameWrongSize(expected, actual) => format!(
                "Type name length mismatch: should be {} characters or fewer, found {}",
                expected, actual
            ),
        };

        write!(f, "{}", output)
    }
}
