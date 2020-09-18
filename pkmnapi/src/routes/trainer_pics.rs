use pkmnapi_sql::*;
use rocket::http::{ContentType, Header};
use rocket::response::Response;
use rocket::State;
use std::io::Cursor;

use crate::guards::*;
use crate::responses::errors::*;
use crate::utils;

#[get("/trainer/pics/<trainer_id>", format = "image/png", rank = 1)]
pub fn get_trainer_pic_png<'a>(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
    trainer_id: u8,
) -> Result<Response<'a>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pic = match db.get_trainer_pic(&trainer_id) {
        Ok(pic) => pic,
        Err(e) => return Err(TrainerPicResponseError::new(&e.to_string())),
    };

    let trainer_name = match db.get_trainer_name(&trainer_id) {
        Ok(trainer_name) => trainer_name,
        Err(e) => return Err(TrainerPicResponseError::new(&e.to_string())),
    };

    let img = match pic.to_png() {
        Ok(img) => img,
        Err(e) => return Err(TrainerPicResponseError::new(&e.to_string())),
    };

    let response = Response::build()
        .header(ContentType::new("image", "png"))
        .header(Header::new(
            "Content-Disposition",
            format!(r#"attachment; filename="{}.png""#, trainer_name.name),
        ))
        .sized_body(Cursor::new(img))
        .finalize();

    Ok(response)
}

#[get("/trainer/pics/<trainer_id>", format = "image/jpeg", rank = 2)]
pub fn get_trainer_pic_jpeg<'a>(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
    trainer_id: u8,
) -> Result<Response<'a>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pic = match db.get_trainer_pic(&trainer_id) {
        Ok(pic) => pic,
        Err(e) => return Err(TrainerPicResponseError::new(&e.to_string())),
    };

    let trainer_name = match db.get_trainer_name(&trainer_id) {
        Ok(trainer_name) => trainer_name,
        Err(e) => return Err(TrainerPicResponseError::new(&e.to_string())),
    };

    let img = match pic.to_jpeg() {
        Ok(img) => img,
        Err(e) => return Err(TrainerPicResponseError::new(&e.to_string())),
    };

    let response = Response::build()
        .header(ContentType::new("image", "jpeg"))
        .header(Header::new(
            "Content-Disposition",
            format!(r#"attachment; filename="{}.jpg""#, trainer_name.name),
        ))
        .sized_body(Cursor::new(img))
        .finalize();

    Ok(response)
}
