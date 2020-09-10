use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::base::*;

#[derive(Debug, Responder)]
pub enum ResponseError {
    AccessTokenErrorForbidden(status::Forbidden<Json<AccessTokenErrorForbidden>>),
    AccessTokenErrorInvalid(status::BadRequest<Json<AccessTokenErrorInvalid>>),
    AccessTokenErrorUnauthorized(status::Unauthorized<Json<AccessTokenErrorUnauthorized>>),
    MoveResponseError(status::NotFound<Json<MoveResponseError>>),
    MoveResponseErrorInvalid(status::BadRequest<Json<MoveResponseErrorInvalid>>),
    PatchResponseError(status::NotFound<Json<PatchResponseError>>),
    PokemonPicResponseError(status::NotFound<Json<PokemonPicResponseError>>),
    RomResponseErrorInvalidRom(status::BadRequest<Json<RomResponseErrorInvalidRom>>),
    RomResponseErrorNoRom(status::Forbidden<Json<RomResponseErrorNoRom>>),
    RomResponseErrorRomExists(status::Forbidden<Json<RomResponseErrorRomExists>>),
    StatsResponseError(status::NotFound<Json<StatsResponseError>>),
    StatsResponseErrorInvalid(status::BadRequest<Json<StatsResponseErrorInvalid>>),
    TMResponseError(status::NotFound<Json<TMResponseError>>),
    TMResponseErrorInvalid(status::BadRequest<Json<TMResponseErrorInvalid>>),
    TypeEffectResponseError(status::NotFound<Json<TypeEffectResponseError>>),
    TypeEffectResponseErrorInvalid(status::BadRequest<Json<TypeEffectResponseErrorInvalid>>),
    TypeResponseError(status::NotFound<Json<TypeResponseError>>),
    TypeResponseErrorInvalid(status::BadRequest<Json<TypeResponseErrorInvalid>>),
}

pub type AccessTokenErrorForbidden = BaseErrorResponse<AccessTokenErrorForbiddenAttributes>;

impl AccessTokenErrorForbidden {
    pub fn new() -> ResponseError {
        let response = AccessTokenErrorForbidden {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_access_tokens_forbidden,
                _type: BaseErrorResponseType::errors,
                attributes: AccessTokenErrorForbiddenAttributes {
                    message: "Authorization header must not be set".to_owned(),
                },
            },
        };

        ResponseError::AccessTokenErrorForbidden(status::Forbidden(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorForbiddenAttributes {
    pub message: String,
}

pub type AccessTokenErrorInvalid = BaseErrorResponse<AccessTokenErrorInvalidAttributes>;

impl AccessTokenErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = AccessTokenErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_access_tokens_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: AccessTokenErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::AccessTokenErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorInvalidAttributes {
    pub message: String,
}

pub type AccessTokenErrorUnauthorized = BaseErrorResponse<AccessTokenErrorUnauthorizedAttributes>;

impl AccessTokenErrorUnauthorized {
    pub fn new() -> ResponseError {
        let response = AccessTokenErrorUnauthorized {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_access_tokens_unauthorized,
                _type: BaseErrorResponseType::errors,
                attributes: AccessTokenErrorUnauthorizedAttributes {
                    message: "Authorization header must be set".to_owned(),
                },
            },
        };

        ResponseError::AccessTokenErrorUnauthorized(status::Unauthorized(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorUnauthorizedAttributes {
    pub message: String,
}

pub type MoveResponseError = BaseErrorResponse<MoveResponseErrorAttributes>;

impl MoveResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = MoveResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_moves,
                _type: BaseErrorResponseType::errors,
                attributes: MoveResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::MoveResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct MoveResponseErrorAttributes {
    pub message: String,
}

pub type MoveResponseErrorInvalid = BaseErrorResponse<MoveResponseErrorInvalidAttributes>;

impl MoveResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = MoveResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_moves_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: MoveResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::MoveResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct MoveResponseErrorInvalidAttributes {
    pub message: String,
}

pub type PatchResponseError = BaseErrorResponse<PatchResponseErrorAttributes>;

impl PatchResponseError {
    pub fn new() -> ResponseError {
        let response = PatchResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_patches,
                _type: BaseErrorResponseType::errors,
                attributes: PatchResponseErrorAttributes {
                    message: "No patch found".to_owned(),
                },
            },
        };

        ResponseError::PatchResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct PatchResponseErrorAttributes {
    pub message: String,
}

pub type PokemonPicResponseError = BaseErrorResponse<PokemonPicResponseErrorAttributes>;

impl PokemonPicResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = PokemonPicResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_pokemon_pics,
                _type: BaseErrorResponseType::errors,
                attributes: PokemonPicResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::PokemonPicResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct PokemonPicResponseErrorAttributes {
    pub message: String,
}

pub type RomResponseErrorNoRom = BaseErrorResponse<RomResponseErrorNoRomAttributes>;

impl RomResponseErrorNoRom {
    pub fn new() -> ResponseError {
        let response = RomResponseErrorNoRom {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_roms_no_rom,
                _type: BaseErrorResponseType::errors,
                attributes: RomResponseErrorNoRomAttributes {
                    message: "No ROM uploaded".to_owned(),
                },
            },
        };

        ResponseError::RomResponseErrorNoRom(status::Forbidden(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct RomResponseErrorNoRomAttributes {
    pub message: String,
}

pub type RomResponseErrorInvalidRom = BaseErrorResponse<RomResponseErrorInvalidRomAttributes>;

impl RomResponseErrorInvalidRom {
    pub fn new() -> ResponseError {
        let response = RomResponseErrorInvalidRom {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_roms_invalid_rom,
                _type: BaseErrorResponseType::errors,
                attributes: RomResponseErrorInvalidRomAttributes {
                    message: "Invalid ROM provided".to_owned(),
                },
            },
        };

        ResponseError::RomResponseErrorInvalidRom(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct RomResponseErrorInvalidRomAttributes {
    pub message: String,
}

pub type RomResponseErrorRomExists = BaseErrorResponse<RomResponseErrorRomExistsAttributes>;

impl RomResponseErrorRomExists {
    pub fn new() -> ResponseError {
        let response = RomResponseErrorRomExists {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_roms_rom_exists,
                _type: BaseErrorResponseType::errors,
                attributes: RomResponseErrorRomExistsAttributes {
                    message: "ROM already exists".to_owned(),
                },
            },
        };

        ResponseError::RomResponseErrorRomExists(status::Forbidden(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct RomResponseErrorRomExistsAttributes {
    pub message: String,
}

pub type StatsResponseError = BaseErrorResponse<StatsResponseErrorAttributes>;

impl StatsResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = StatsResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_stats,
                _type: BaseErrorResponseType::errors,
                attributes: StatsResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::StatsResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct StatsResponseErrorAttributes {
    pub message: String,
}

pub type StatsResponseErrorInvalid = BaseErrorResponse<StatsResponseErrorInvalidAttributes>;

impl StatsResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = StatsResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_stats_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: StatsResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::StatsResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct StatsResponseErrorInvalidAttributes {
    pub message: String,
}

pub type TMResponseError = BaseErrorResponse<TMResponseErrorAttributes>;

impl TMResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = TMResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_tms,
                _type: BaseErrorResponseType::errors,
                attributes: TMResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TMResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct TMResponseErrorAttributes {
    pub message: String,
}

pub type TMResponseErrorInvalid = BaseErrorResponse<TMResponseErrorInvalidAttributes>;

impl TMResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = TMResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_tms_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: TMResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TMResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct TMResponseErrorInvalidAttributes {
    pub message: String,
}

pub type TypeResponseError = BaseErrorResponse<TypeResponseErrorAttributes>;

impl TypeResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = TypeResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_types,
                _type: BaseErrorResponseType::errors,
                attributes: TypeResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TypeResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct TypeResponseErrorAttributes {
    pub message: String,
}

pub type TypeResponseErrorInvalid = BaseErrorResponse<TypeResponseErrorInvalidAttributes>;

impl TypeResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = TypeResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_types_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: TypeResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TypeResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct TypeResponseErrorInvalidAttributes {
    pub message: String,
}

pub type TypeEffectResponseError = BaseErrorResponse<TypeEffectResponseErrorAttributes>;

impl TypeEffectResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = TypeEffectResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_type_effects,
                _type: BaseErrorResponseType::errors,
                attributes: TypeEffectResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TypeEffectResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct TypeEffectResponseErrorAttributes {
    pub message: String,
}

pub type TypeEffectResponseErrorInvalid =
    BaseErrorResponse<TypeEffectResponseErrorInvalidAttributes>;

impl TypeEffectResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = TypeEffectResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_type_effects_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: TypeEffectResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TypeEffectResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct TypeEffectResponseErrorInvalidAttributes {
    pub message: String,
}
