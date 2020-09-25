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
use rocket_cors::AllowedHeaders;

pub struct Pkmnapi {}

impl Pkmnapi {
    pub fn init() -> Rocket {
        let sql = PkmnapiSQL::new();
        let cors = rocket_cors::CorsOptions {
            allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
            allow_credentials: true,
            ..Default::default()
        }
        .to_cors()
        .unwrap();

        rocket::ignite()
            .manage(sql)
            .mount("/", routes![routes::status::status,])
            .mount(
                "/v1",
                routes![
                    routes::access_tokens::post_access_token,
                    routes::hms::get_hm,
                    routes::hms::post_hm,
                    routes::map_pics::get_map_pic_jpeg,
                    routes::map_pics::get_map_pic_png,
                    routes::move_names::get_move_name,
                    routes::move_names::post_move_name,
                    routes::pokemon_names::get_pokemon_name,
                    routes::pokemon_names::post_pokemon_name,
                    routes::pokemon_pics::get_pokemon_pic_jpeg,
                    routes::pokemon_pics::get_pokemon_pic_png,
                    routes::pokemon_pics::post_pokemon_pic_jpeg,
                    routes::pokemon_pics::post_pokemon_pic_png,
                    routes::pokemon_stats::get_pokemon_stats,
                    routes::pokemon_stats::post_pokemon_stats,
                    routes::rom_patches::delete_rom_patch,
                    routes::rom_patches::get_rom_patch,
                    routes::rom_patches::get_rom_patches_raw,
                    routes::rom_patches::get_rom_patches,
                    routes::roms::delete_rom,
                    routes::roms::get_rom,
                    routes::roms::post_rom,
                    routes::sav_player_names::get_sav_player_name,
                    routes::sav_player_names::post_sav_player_name,
                    routes::savs::delete_sav,
                    routes::savs::get_sav,
                    routes::savs::post_sav,
                    routes::tms::get_tm,
                    routes::tms::post_tm,
                    routes::trainer_names::get_trainer_name,
                    routes::trainer_names::post_trainer_name,
                    routes::trainer_parties::get_trainer_parties,
                    routes::trainer_parties::post_trainer_parties,
                    routes::trainer_pics::get_trainer_pic_jpeg,
                    routes::trainer_pics::get_trainer_pic_png,
                    routes::trainer_pics::post_trainer_pic_jpeg,
                    routes::trainer_pics::post_trainer_pic_png,
                    routes::type_effects::get_type_effect,
                    routes::type_effects::post_type_effect,
                    routes::type_names::get_type_name,
                    routes::type_names::post_type_name,
                ],
            )
            .register(catchers![
                routes::errors::not_found,
                routes::errors::internal_server_error
            ])
            .attach(AdHoc::on_response("Update Server Name", |_, res| {
                res.set_raw_header(
                    "Server",
                    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                );
            }))
            .attach(cors)
    }
}
