#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

pub mod guards;
pub mod requests;
pub mod responses;
pub mod routes;
pub mod utils;

use pkmnapi_sql::*;
use rocket::fairing::AdHoc;
use rocket::Rocket;

pub struct Pkmnapi {}

impl Pkmnapi {
    pub fn init() -> Rocket {
        let sql = PkmnapiSQL::new();

        rocket::ignite()
            .manage(sql)
            .mount("/", routes![routes::status::status,])
            .mount(
                "/v1",
                routes![
                    routes::access_tokens::post_access_token,
                    routes::roms::post_rom,
                    routes::roms::get_rom,
                    routes::roms::delete_rom,
                    routes::types::get_type,
                    routes::types::post_type,
                    routes::type_effects::get_type_effect,
                    routes::type_effects::post_type_effect,
                    routes::patches::get_patches,
                    routes::patches::get_patches_raw,
                    routes::patches::get_patch
                ],
            )
            .attach(AdHoc::on_response("Update Server Name", |_, res| {
                res.set_raw_header("Server", "pkmnapi");
            }))
    }
}
