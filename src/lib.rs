pub mod text;

const CARD_NAME_OFFSET: usize = 2194441;

pub fn read_card_data(rom_file: &Vec<u8>) {
    //TODO: Assert that rom_file size is correct.

    // Placeholder implementation. Just reads the part where the name of the
    // card "Blue-eyes White Dragon" is stored. Currently, this exists just
    // to verify that our tests searching for the data in the ROM are
    // working.
    for n in 0..23 {
        println!(
            "{}    {}",
            text::u8_to_char(rom_file[CARD_NAME_OFFSET + n]),
            rom_file[CARD_NAME_OFFSET + n]
        );
    }
}
