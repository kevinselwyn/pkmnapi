use pkmnapi_sql::models::Patch;
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type PatchResponse = BaseResponse<PatchResponseAttributes>;
pub type PatchesResponse = BaseResponseAll<PatchResponse>;

impl PatchesResponse {
    /// Create a new `PatchesResponse`
    pub fn new(patches: &Vec<Patch>) -> PatchesResponse {
        PatchesResponse {
            data: patches
                .iter()
                .map(|patch| PatchResponse::new(patch))
                .collect(),
            links: Links {
                _self: utils::generate_url("patches", None),
            },
        }
    }
}

impl PatchResponse {
    /// Create a new `PatchResponse`
    pub fn new(patch: &Patch) -> PatchResponse {
        PatchResponse {
            id: patch.id.to_owned(),
            _type: BaseResponseType::patches,
            attributes: PatchResponseAttributes {
                description: match &patch.description {
                    Some(description) => Some(description.to_owned()),
                    None => None,
                },
            },
            links: Links {
                _self: utils::generate_url("patches", Some(&patch.id)),
            },
        }
    }
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum PatchResponseType {
    patches,
}

#[derive(Debug, Serialize)]
pub struct PatchResponseAttributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}
