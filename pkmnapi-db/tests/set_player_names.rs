use pkmnapi_db::patch::*;
use pkmnapi_db::string::*;
use pkmnapi_db::types::*;

mod common;

#[test]
#[ignore]
#[allow(non_snake_case)]
fn get_player_names() {
    let db = common::load_rom();

    let player_names = PlayerNames {
        player: vec![
            ROMString::from("RED"),
            ROMString::from("ASH"),
            ROMString::from("JACK"),
        ],
        rival: vec![
            ROMString::from("BLUE"),
            ROMString::from("GARY"),
            ROMString::from("JOHN"),
        ],
    };

    let patch = db.set_player_names(&player_names).unwrap();

    assert_eq!(
        patch,
        Patch {
            offset: 0x6AA8,
            length: 120,
            data: vec![
                0x8D, 0x84, 0x96, 0x7F, 0x8D, 0x80, 0x8C, 0x84, 0x4E, 0x91, 0x84, 0x83, 0x4E, 0x80,
                0x92, 0x87, 0x4E, 0x89, 0x80, 0x82, 0x8A, 0x50, 0x8D, 0x84, 0x96, 0x7F, 0x8D, 0x80,
                0x8C, 0x84, 0x4E, 0x81, 0x8B, 0x94, 0x84, 0x4E, 0x86, 0x80, 0x91, 0x98, 0x4E, 0x89,
                0x8E, 0x87, 0x8D, 0x50, 0x47, 0x0E, 0x00, 0x54, 0x5D, 0x2A, 0xFE, 0x50, 0x20, 0xFB,
                0x78, 0xB9, 0x28, 0x03, 0x0C, 0x18, 0xF2, 0x62, 0x6B, 0x11, 0x6D, 0xCD, 0x01, 0x14,
                0x00, 0xC3, 0xB5, 0x00, 0x8D, 0x84, 0x96, 0x7F, 0x8D, 0x80, 0x8C, 0x84, 0x50, 0x91,
                0x84, 0x83, 0x50, 0x80, 0x92, 0x87, 0x50, 0x89, 0x80, 0x82, 0x8A, 0x50, 0x8D, 0x84,
                0x96, 0x7F, 0x8D, 0x80, 0x8C, 0x84, 0x50, 0x81, 0x8B, 0x94, 0x84, 0x50, 0x86, 0x80,
                0x91, 0x98, 0x50, 0x89, 0x8E, 0x87, 0x8D, 0x50,
            ],
        }
    );
}
