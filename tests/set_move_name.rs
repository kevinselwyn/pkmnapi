macro_rules! set_move_name_test {
    ($test_name: ident, $move_id: expr, $move_name: expr, $patch_offset: expr, $patch_data: expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.set_move_name(
                $move_id,
                PkmnapiDBMoveName {
                    name: PkmnapiDBString::from($move_name),
                },
            ) {
                Ok(patch) => assert_eq!(
                    patch,
                    PkmnapiDBPatch {
                        offset: $patch_offset,
                        length: $patch_data.len(),
                        data: $patch_data
                    },
                    "Searched for move ID: {}",
                    $move_id
                ),
                Err(_) => panic!(format!("Could not find move ID: {}", $move_id)),
            };
        }
    };
}

#[cfg(test)]
#[rustfmt::skip::macros(set_move_name_test)]
mod tests {
    use pkmnapi::db::patch::*;
    use pkmnapi::db::string::*;
    use pkmnapi::db::types::*;

    mod common;

    set_move_name_test!(set_move_name_1, 1, "ABCDE", 0xB0000, vec![0x80, 0x81, 0x82, 0x83, 0x84]);
    set_move_name_test!(set_move_name_2, 2, "ABCDEFGHIJK", 0xB0006, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]);
    set_move_name_test!(set_move_name_3, 3, "ABCDEFGHIJ", 0xB0012, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_4, 4, "ABCDEFGHIJK", 0xB001D, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]);
    set_move_name_test!(set_move_name_5, 5, "ABCDEFGHIJ", 0xB0029, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_6, 6, "ABCDEFG", 0xB0034, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]);
    set_move_name_test!(set_move_name_7, 7, "ABCDEFGHIJ", 0xB003C, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_8, 8, "ABCDEFGHI", 0xB0047, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]);
    set_move_name_test!(set_move_name_9, 9, "ABCDEFGHIJKL", 0xB0051, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B]);
    set_move_name_test!(set_move_name_10, 10, "ABCDEFG", 0xB005E, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]);
    set_move_name_test!(set_move_name_11, 11, "ABCDEFGH", 0xB0066, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87]);
    set_move_name_test!(set_move_name_12, 12, "ABCDEFGHIJ", 0xB006F, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_13, 13, "ABCDEFGHIJ", 0xB007A, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_14, 14, "ABCDEFGHIJKL", 0xB0085, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B]);
    set_move_name_test!(set_move_name_15, 15, "ABC", 0xB0092, vec![0x80, 0x81, 0x82]);
    set_move_name_test!(set_move_name_16, 16, "ABCD", 0xB0096, vec![0x80, 0x81, 0x82, 0x83]);
    set_move_name_test!(set_move_name_17, 17, "ABCDEFGHIJK", 0xB009B, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]);
    set_move_name_test!(set_move_name_18, 18, "ABCDEFGHI", 0xB00A7, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]);
    set_move_name_test!(set_move_name_19, 19, "ABC", 0xB00B1, vec![0x80, 0x81, 0x82]);
    set_move_name_test!(set_move_name_20, 20, "ABCD", 0xB00B5, vec![0x80, 0x81, 0x82, 0x83]);
    set_move_name_test!(set_move_name_21, 21, "ABCD", 0xB00BA, vec![0x80, 0x81, 0x82, 0x83]);
    set_move_name_test!(set_move_name_22, 22, "ABCDEFGHI", 0xB00BF, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]);
    set_move_name_test!(set_move_name_23, 23, "ABCDE", 0xB00C9, vec![0x80, 0x81, 0x82, 0x83, 0x84]);
    set_move_name_test!(set_move_name_24, 24, "ABCDEFGHIJK", 0xB00CF, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]);
    set_move_name_test!(set_move_name_25, 25, "ABCDEFGHI", 0xB00DB, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]);
    set_move_name_test!(set_move_name_26, 26, "ABCDEFGHI", 0xB00E5, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]);
    set_move_name_test!(set_move_name_27, 27, "ABCDEFGHIJKL", 0xB00EF, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B]);
    set_move_name_test!(set_move_name_28, 28, "ABCDEFGHIJK", 0xB00FC, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]);
    set_move_name_test!(set_move_name_29, 29, "ABCDEFGH", 0xB0108, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87]);
    set_move_name_test!(set_move_name_30, 30, "ABCDEFGHIJK", 0xB0111, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]);
    set_move_name_test!(set_move_name_31, 31, "ABCDEFGHIJK", 0xB011D, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]);
    set_move_name_test!(set_move_name_32, 32, "ABCDEFGHIJ", 0xB0129, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_33, 33, "ABCDEF", 0xB0134, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85]);
    set_move_name_test!(set_move_name_34, 34, "ABCDEFGHI", 0xB013B, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]);
    set_move_name_test!(set_move_name_35, 35, "ABCD", 0xB0145, vec![0x80, 0x81, 0x82, 0x83]);
    set_move_name_test!(set_move_name_36, 36, "ABCDEFGHI", 0xB014A, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]);
    set_move_name_test!(set_move_name_37, 37, "ABCDEF", 0xB0154, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85]);
    set_move_name_test!(set_move_name_38, 38, "ABCDEFGHIJK", 0xB015B, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]);
    set_move_name_test!(set_move_name_39, 39, "ABCDEFGHI", 0xB0167, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]);
    set_move_name_test!(set_move_name_40, 40, "ABCDEFGHIJKL", 0xB0171, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B]);
    set_move_name_test!(set_move_name_41, 41, "ABCDEFGHI", 0xB017E, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]);
    set_move_name_test!(set_move_name_42, 42, "ABCDEFGHIJK", 0xB0188, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]);
    set_move_name_test!(set_move_name_43, 43, "ABCD", 0xB0194, vec![0x80, 0x81, 0x82, 0x83]);
    set_move_name_test!(set_move_name_44, 44, "ABCD", 0xB0199, vec![0x80, 0x81, 0x82, 0x83]);
    set_move_name_test!(set_move_name_45, 45, "ABCDE", 0xB019E, vec![0x80, 0x81, 0x82, 0x83, 0x84]);
    set_move_name_test!(set_move_name_46, 46, "ABCD", 0xB01A4, vec![0x80, 0x81, 0x82, 0x83]);
    set_move_name_test!(set_move_name_47, 47, "ABCD", 0xB01A9, vec![0x80, 0x81, 0x82, 0x83]);
    set_move_name_test!(set_move_name_48, 48, "ABCDEFGHIJ", 0xB01AE, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_49, 49, "ABCDEFGHI", 0xB01B9, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]);
    set_move_name_test!(set_move_name_50, 50, "ABCDEFG", 0xB01C3, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]);
    set_move_name_test!(set_move_name_51, 51, "ABCD", 0xB01CB, vec![0x80, 0x81, 0x82, 0x83]);
    set_move_name_test!(set_move_name_52, 52, "ABCDE", 0xB01D0, vec![0x80, 0x81, 0x82, 0x83, 0x84]);
    set_move_name_test!(set_move_name_53, 53, "ABCDEFGHIJKL", 0xB01D6, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B]);
    set_move_name_test!(set_move_name_54, 54, "ABCD", 0xB01E3, vec![0x80, 0x81, 0x82, 0x83]);
    set_move_name_test!(set_move_name_55, 55, "ABCDEFGHI", 0xB01E8, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]);
    set_move_name_test!(set_move_name_56, 56, "ABCDEFGHIJ", 0xB01F2, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_57, 57, "ABCD", 0xB01FD, vec![0x80, 0x81, 0x82, 0x83]);
    set_move_name_test!(set_move_name_58, 58, "ABCDEFGH", 0xB0202, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87]);
    set_move_name_test!(set_move_name_59, 59, "ABCDEFGH", 0xB020B, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87]);
    set_move_name_test!(set_move_name_60, 60, "ABCDEFG", 0xB0214, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]);
    set_move_name_test!(set_move_name_61, 61, "ABCDEFGHIJ", 0xB021C, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_62, 62, "ABCDEFGHIJK", 0xB0227, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]);
    set_move_name_test!(set_move_name_63, 63, "ABCDEFGHIJ", 0xB0233, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_64, 64, "ABCD", 0xB023E, vec![0x80, 0x81, 0x82, 0x83]);
    set_move_name_test!(set_move_name_65, 65, "ABCDEFGHIJ", 0xB0243, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_66, 66, "ABCDEFGHIJ", 0xB024E, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_67, 67, "ABCDEFGH", 0xB0259, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87]);
    set_move_name_test!(set_move_name_68, 68, "ABCDEFG", 0xB0262, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]);
    set_move_name_test!(set_move_name_69, 69, "ABCDEFGHIJKL", 0xB026A, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B]);
    set_move_name_test!(set_move_name_70, 70, "ABCDEFGH", 0xB0277, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87]);
    set_move_name_test!(set_move_name_71, 71, "ABCDEF", 0xB0280, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85]);
    set_move_name_test!(set_move_name_72, 72, "ABCDEFGHIJ", 0xB0287, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_73, 73, "ABCDEFGHIJ", 0xB0292, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_74, 74, "ABCDEF", 0xB029D, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85]);
    set_move_name_test!(set_move_name_75, 75, "ABCDEFGHIJ", 0xB02A4, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_76, 76, "ABCDEFGHI", 0xB02AF, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]);
    set_move_name_test!(set_move_name_77, 77, "ABCDEFGHIJKL", 0xB02B9, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B]);
    set_move_name_test!(set_move_name_78, 78, "ABCDEFGHIJ", 0xB02C6, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_79, 79, "ABCDEFGHIJKL", 0xB02D1, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B]);
    set_move_name_test!(set_move_name_80, 80, "ABCDEFGHIJK", 0xB02DE, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]);
    set_move_name_test!(set_move_name_81, 81, "ABCDEFGHIJK", 0xB02EA, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]);
    set_move_name_test!(set_move_name_82, 82, "ABCDEFGHIJK", 0xB02F6, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]);
    set_move_name_test!(set_move_name_83, 83, "ABCDEFGHI", 0xB0302, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]);
    set_move_name_test!(set_move_name_84, 84, "ABCDEFGHIJKL", 0xB030C, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B]);
    set_move_name_test!(set_move_name_85, 85, "ABCDEFGHIJK", 0xB0319, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]);
    set_move_name_test!(set_move_name_86, 86, "ABCDEFGHIJKL", 0xB0325, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B]);
    set_move_name_test!(set_move_name_87, 87, "ABCDEFG", 0xB0332, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]);
    set_move_name_test!(set_move_name_88, 88, "ABCDEFGHIJ", 0xB033A, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_89, 89, "ABCDEFGHIJ", 0xB0345, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_90, 90, "ABCDEFG", 0xB0350, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]);
    set_move_name_test!(set_move_name_91, 91, "ABC", 0xB0358, vec![0x80, 0x81, 0x82]);
    set_move_name_test!(set_move_name_92, 92, "ABCDE", 0xB035C, vec![0x80, 0x81, 0x82, 0x83, 0x84]);
    set_move_name_test!(set_move_name_93, 93, "ABCDEFGHI", 0xB0362, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]);
    set_move_name_test!(set_move_name_94, 94, "ABCDEFG", 0xB036C, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]);
    set_move_name_test!(set_move_name_95, 95, "ABCDEFGH", 0xB0374, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87]);
    set_move_name_test!(set_move_name_96, 96, "ABCDEFGH", 0xB037D, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87]);
    set_move_name_test!(set_move_name_97, 97, "ABCDEFG", 0xB0386, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]);
    set_move_name_test!(set_move_name_98, 98, "ABCDEFGHIJKL", 0xB038E, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B]);
    set_move_name_test!(set_move_name_99, 99, "ABCD", 0xB039B, vec![0x80, 0x81, 0x82, 0x83]);
    set_move_name_test!(set_move_name_100, 100, "ABCDEFGH", 0xB03A0, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87]);
    set_move_name_test!(set_move_name_101, 101, "ABCDEFGHIJK", 0xB03A9, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]);
    set_move_name_test!(set_move_name_102, 102, "ABCDE", 0xB03B5, vec![0x80, 0x81, 0x82, 0x83, 0x84]);
    set_move_name_test!(set_move_name_103, 103, "ABCDEFG", 0xB03BB, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]);
    set_move_name_test!(set_move_name_104, 104, "ABCDEFGHIJK", 0xB03C3, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]);
    set_move_name_test!(set_move_name_105, 105, "ABCDEFG", 0xB03CF, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]);
    set_move_name_test!(set_move_name_106, 106, "ABCDEF", 0xB03D7, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85]);
    set_move_name_test!(set_move_name_107, 107, "ABCDEFGH", 0xB03DE, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87]);
    set_move_name_test!(set_move_name_108, 108, "ABCDEFGHIJK", 0xB03E7, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]);
    set_move_name_test!(set_move_name_109, 109, "ABCDEFGHIJK", 0xB03F3, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]);
    set_move_name_test!(set_move_name_110, 110, "ABCDEFGH", 0xB03FF, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87]);
    set_move_name_test!(set_move_name_111, 111, "ABCDEFGHIJKL", 0xB0408, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B]);
    set_move_name_test!(set_move_name_112, 112, "ABCDEFG", 0xB0415, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]);
    set_move_name_test!(set_move_name_113, 113, "ABCDEFGHIJKL", 0xB041D, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B]);
    set_move_name_test!(set_move_name_114, 114, "ABCD", 0xB042A, vec![0x80, 0x81, 0x82, 0x83]);
    set_move_name_test!(set_move_name_115, 115, "ABCDEFG", 0xB042F, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]);
    set_move_name_test!(set_move_name_116, 116, "ABCDEFGHIJKL", 0xB0437, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B]);
    set_move_name_test!(set_move_name_117, 117, "ABCD", 0xB0444, vec![0x80, 0x81, 0x82, 0x83]);
    set_move_name_test!(set_move_name_118, 118, "ABCDEFGHI", 0xB0449, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]);
    set_move_name_test!(set_move_name_119, 119, "ABCDEFGHIJK", 0xB0453, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]);
    set_move_name_test!(set_move_name_120, 120, "ABCDEFGHIJKL", 0xB045F, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B]);
    set_move_name_test!(set_move_name_121, 121, "ABCDEFGH", 0xB046C, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87]);
    set_move_name_test!(set_move_name_122, 122, "ABCD", 0xB0475, vec![0x80, 0x81, 0x82, 0x83]);
    set_move_name_test!(set_move_name_123, 123, "ABCD", 0xB047A, vec![0x80, 0x81, 0x82, 0x83]);
    set_move_name_test!(set_move_name_124, 124, "ABCDEF", 0xB047F, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85]);
    set_move_name_test!(set_move_name_125, 125, "ABCDEFGHI", 0xB0486, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]);
    set_move_name_test!(set_move_name_126, 126, "ABCDEFGHIJ", 0xB0490, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_127, 127, "ABCDEFGHI", 0xB049B, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]);
    set_move_name_test!(set_move_name_128, 128, "ABCDE", 0xB04A5, vec![0x80, 0x81, 0x82, 0x83, 0x84]);
    set_move_name_test!(set_move_name_129, 129, "ABCDE", 0xB04AB, vec![0x80, 0x81, 0x82, 0x83, 0x84]);
    set_move_name_test!(set_move_name_130, 130, "ABCDEFGHIJ", 0xB04B1, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_131, 131, "ABCDEFGHIJKL", 0xB04BC, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B]);
    set_move_name_test!(set_move_name_132, 132, "ABCDEFGHI", 0xB04C9, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]);
    set_move_name_test!(set_move_name_133, 133, "ABCDEFG", 0xB04D3, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]);
    set_move_name_test!(set_move_name_134, 134, "ABCDEFG", 0xB04DB, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]);
    set_move_name_test!(set_move_name_135, 135, "ABCDEFGHIJ", 0xB04E3, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_136, 136, "ABCDEFGHIJKL", 0xB04EE, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B]);
    set_move_name_test!(set_move_name_137, 137, "ABCDE", 0xB04FB, vec![0x80, 0x81, 0x82, 0x83, 0x84]);
    set_move_name_test!(set_move_name_138, 138, "ABCDEFGHIJK", 0xB0501, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]);
    set_move_name_test!(set_move_name_139, 139, "ABCDEFGHIJ", 0xB050D, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_140, 140, "ABCDEFG", 0xB0518, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]);
    set_move_name_test!(set_move_name_141, 141, "ABCDEFGHIJ", 0xB0520, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_142, 142, "ABCDEFGHIJK", 0xB052B, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]);
    set_move_name_test!(set_move_name_143, 143, "ABCDEFGHIJ", 0xB0537, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_144, 144, "ABCDEFGHI", 0xB0542, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]);
    set_move_name_test!(set_move_name_145, 145, "ABCDEF", 0xB054C, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85]);
    set_move_name_test!(set_move_name_146, 146, "ABCDEFGHIJK", 0xB0553, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]);
    set_move_name_test!(set_move_name_147, 147, "ABCDE", 0xB055F, vec![0x80, 0x81, 0x82, 0x83, 0x84]);
    set_move_name_test!(set_move_name_148, 148, "ABCDE", 0xB0565, vec![0x80, 0x81, 0x82, 0x83, 0x84]);
    set_move_name_test!(set_move_name_149, 149, "ABCDEFG", 0xB056B, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]);
    set_move_name_test!(set_move_name_150, 150, "ABCDEF", 0xB0573, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85]);
    set_move_name_test!(set_move_name_151, 151, "ABCDEFGHIJ", 0xB057A, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_152, 152, "ABCDEFGHIJ", 0xB0585, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_153, 153, "ABCDEFGHI", 0xB0590, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]);
    set_move_name_test!(set_move_name_154, 154, "ABCDEFGHIJK", 0xB059A, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]);
    set_move_name_test!(set_move_name_155, 155, "ABCDEFGHIJ", 0xB05A6, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_156, 156, "ABCD", 0xB05B1, vec![0x80, 0x81, 0x82, 0x83]);
    set_move_name_test!(set_move_name_157, 157, "ABCDEFGHIJ", 0xB05B6, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_158, 158, "ABCDEFGHIJ", 0xB05C1, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_159, 159, "ABCDEFG", 0xB05CC, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]);
    set_move_name_test!(set_move_name_160, 160, "ABCDEFGHIJ", 0xB05D4, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_161, 161, "ABCDEFGHIJ", 0xB05DF, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_162, 162, "ABCDEFGHIJ", 0xB05EA, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_163, 163, "ABCDE", 0xB05F5, vec![0x80, 0x81, 0x82, 0x83, 0x84]);
    set_move_name_test!(set_move_name_164, 164, "ABCDEFGHIJ", 0xB05FB, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]);
    set_move_name_test!(set_move_name_165, 165, "ABCDEFGH", 0xB0606, vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87]);
}
