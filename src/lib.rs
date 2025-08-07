mod image;
pub mod text;

const CARD_NAME_INDICES_OFFSET: usize = 0x1C6002;
const CARD_NAME_OFFSET: usize = 0x1C6800;

pub fn read_card_data(rom_file: &Vec<u8>) {
    //TODO: Assert that rom_file size is correct.

    let slus = image::get_slus_from_bin(rom_file);

    // Placeholder implementation. Just reads the part where the name of the
    // card "Blue-eyes White Dragon" is stored. Currently, this exists just
    // to verify that our tests searching for the data in the ROM are
    // working.

    // Read Blue-eyes White Dragon's name index
    let name_index: usize =
        (slus[CARD_NAME_INDICES_OFFSET] << 8 +
         slus[CARD_NAME_INDICES_OFFSET + 1])
        .into();

    for n in 0..22 {
        print!( "{}", text::u8_to_char(slus[CARD_NAME_OFFSET + name_index + n]));
    }
    println!();
}
