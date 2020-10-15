use pkmnapi_db::PokemonLearnset;
use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type PokemonLearnsetRequest =
    BaseRequest<PokemonLearnsetRequestType, PokemonLearnsetRequestAttributes>;

impl PokemonLearnsetRequest {
    pub fn get_learnset(&self) -> Vec<PokemonLearnset> {
        self.data
            .attributes
            .learnset
            .iter()
            .map(|learnset| PokemonLearnset::new(learnset.level, learnset._move.id))
            .collect()
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum PokemonLearnsetRequestType {
    pokemon_learnsets,
}

#[derive(Debug, Deserialize)]
pub struct PokemonLearnsetRequestAttributes {
    pub learnset: Vec<PokemonLearnsetRequestAttributesLearnset>,
}

#[derive(Debug, Deserialize)]
pub struct PokemonLearnsetRequestAttributesLearnset {
    pub level: u8,

    #[serde(rename = "move")]
    pub _move: PokemonLearnsetRequestAttributesLearnsetMove,
}

#[derive(Debug, Deserialize)]
pub struct PokemonLearnsetRequestAttributesLearnsetMove {
    #[serde(deserialize_with = "crate::utils::from_numeric_str")]
    pub id: u8,
}
