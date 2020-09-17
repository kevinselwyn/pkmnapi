use serde::Serialize;

use crate::responses::links::Links;

#[derive(Debug, Serialize)]
pub struct BaseResponse<T> {
    pub data: BaseResponseData<T>,
    pub links: Links,
}

#[derive(Debug, Serialize)]
pub struct BaseResponseData<T> {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: BaseResponseType,
    pub attributes: T,
    pub links: Links,
}

#[derive(Debug, Serialize)]
pub struct BaseResponseAll<T> {
    pub data: Vec<T>,
    pub links: Links,
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum BaseResponseType {
    moves,
    pokemon_names,
    rom_patches,
    roms,
    savs,
    stats,
    tms,
    trainer_names,
    type_effects,
    types,
}

#[derive(Debug, Serialize)]
pub struct BaseErrorResponse<T> {
    pub data: BaseErrorResponseData<T>,
}

#[derive(Debug, Serialize)]
pub struct BaseErrorResponseData<T> {
    pub id: BaseErrorResponseId,
    #[serde(rename = "type")]
    pub _type: BaseErrorResponseType,
    pub attributes: T,
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum BaseErrorResponseId {
    error_access_tokens_email,
    error_access_tokens_forbidden,
    error_access_tokens_invalid,
    error_access_tokens_timeout,
    error_access_tokens_unauthorized,
    error_map_pics,
    error_moves_invalid,
    error_moves,
    error_pokemon_names_invalid,
    error_pokemon_names,
    error_pokemon_pics,
    error_rom_patches,
    error_roms_invalid_rom,
    error_roms_no_rom,
    error_roms_rom_exists,
    error_savs_invalid_sav,
    error_savs_no_sav,
    error_savs_sav_exists,
    error_stats_invalid,
    error_stats,
    error_tms_invalid,
    error_tms,
    error_trainer_names_invalid,
    error_trainer_names,
    error_trainer_pics,
    error_type_effects_invalid,
    error_type_effects,
    error_types_invalid,
    error_types,
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum BaseErrorResponseType {
    errors,
}
