use pkmnapi_sql::*;
use rocket::http::{ContentType, Header};
use rocket::response::status;
use rocket::response::Response;
use rocket::State;
use rocket_contrib::json::Json;
use std::io::Cursor;

use crate::guards::*;
use crate::responses::errors::*;
use crate::responses::rom_patches::*;

#[get("/roms/patches", format = "application/json", rank = 1)]
pub fn get_rom_patches(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<RomPatchesResponse>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let connection = sql.get_connection().unwrap();
    let patches = match sql.select_rom_patches_by_access_token(&connection, &access_token) {
        Ok(patches) => patches,
        Err(_) => return Err(RomResponseErrorNoRom::new()),
    };

    let response = RomPatchesResponse::new(&patches);

    Ok(Json(response))
}

#[get("/roms/patches", format = "application/patch", rank = 2)]
pub fn get_rom_patches_raw<'a>(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Response<'a>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let connection = sql.get_connection().unwrap();
    let patches = match sql.select_rom_patches_by_access_token(&connection, &access_token) {
        Ok(patches) => patches,
        Err(_) => return Err(RomResponseErrorNoRom::new()),
    };

    let patch: Vec<u8> = [
        "PATCH".chars().map(|c| c as u8).collect::<Vec<u8>>(),
        patches
            .iter()
            .map(|patch| patch.data.to_vec())
            .flatten()
            .collect(),
        "EOF".chars().map(|c| c as u8).collect::<Vec<u8>>(),
    ]
    .concat();

    let response = Response::build()
        .header(ContentType::new("application", "patch"))
        .header(Header::new(
            "Content-Disposition",
            r#"attachment; filename="patch.ips""#,
        ))
        .sized_body(Cursor::new(patch))
        .finalize();

    Ok(response)
}

#[get("/roms/patches/<patch_id>")]
pub fn get_rom_patch<'a>(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_id: String,
) -> Result<Response<'a>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let connection = sql.get_connection().unwrap();
    let patch = match sql.select_rom_patch_by_id(&connection, &access_token, &patch_id) {
        Ok(Some(patch)) => patch,
        Ok(None) => return Err(RomPatchResponseError::new()),
        Err(_) => return Err(RomResponseErrorNoRom::new()),
    };

    let response = RomPatchResponse::new(&patch);
    let body = serde_json::to_string(&response).unwrap();

    let response = Response::build()
        .header(ContentType::JSON)
        .header(Header::new("ETag", patch.etag))
        .sized_body(Cursor::new(body))
        .finalize();

    Ok(response)
}

#[delete("/roms/patches/<patch_id>")]
pub fn delete_rom_patch(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
    if_match: Result<IfMatch, IfMatchError>,
    patch_id: String,
) -> Result<status::NoContent, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let etag = match if_match {
        Ok(if_match) => if_match.into_inner(),
        Err(_) => return Err(ETagErrorMissing::new()),
    };

    let connection = sql.get_connection().unwrap();
    match sql.delete_rom_patch_by_id(&connection, &access_token, &patch_id, &etag) {
        Ok(_) => {}
        Err(pkmnapi_sql::error::Error::ETagError) => return Err(ETagErrorMismatch::new()),
        Err(_) => return Err(RomResponseErrorNoRom::new()),
    }

    Ok(status::NoContent)
}
