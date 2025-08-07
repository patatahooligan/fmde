mod image;
pub mod text;

const NUMBER_OF_CARDS: usize = 722;

const CARD_NAME_INDICES_OFFSET: usize = 0x1C6002;
// I've noticed that the card names do not start from here because their
// relative offsets don't start at `0`. So this might be a misnomer and
// it might actually by the start of a data section with more stuff than
// just the names.
const CARD_NAME_OFFSET: usize = 0x1C0800;

/// Find all card names in the ROM file and print them. This is a
/// placeholder function which is used to verify that our manipulation
/// of the image file is logically sound. It will eventually be removed
/// and replaced with functions that implement a cleaner interface, eg
/// returning stuff instead of printing, and operating only on the
/// relevant segment, ie the SLUS file instead of the entire bin.
pub fn print_card_names(rom_file: &Vec<u8>) {
    //TODO: Assert that rom_file size is correct.

    let slus = image::get_slus_from_bin(rom_file);

    for i in 0..NUMBER_OF_CARDS {
        // The game stores a relative offset starting from CARD_NAME_OFFSET
        let low_byte: usize = slus[CARD_NAME_INDICES_OFFSET + 2 * i].into();
        let high_byte: usize =
            slus[CARD_NAME_INDICES_OFFSET + 2 * i + 1].into();
        let name_relative_offset: usize = (high_byte << 8) + low_byte;

        let name_absolute_offset = CARD_NAME_OFFSET + name_relative_offset;
        let card_name =
            text::read_terminated_string(&slus[name_absolute_offset..]);
        println!("{}", card_name);
    }
}
