//! Functions to test the rest of the modules. These are not unit tests
//! because I'm not sure what meaningful unit tests I could create at
//! this point in development. Right now I need to do some sort of e2e
//! test so I can create something that will run in an emulator. I don't
//! have any other way to verify correctness. After I have a working
//! mod, I can be sure of what the expected output of each function
//! should be and I can revisit unit testing.

use crate::*;

/// Does a simple passthrough of the game data to verify that we can
/// read & write the image without errors and without accidentally
/// changing something. The output file should be the identical to the
/// input file.
pub fn passthrough_test(rom_file: &mut Vec<u8>) {
    let slus = image::read_slus_from_bin(rom_file);
    let mut wa_mrg = image::read_wa_mrg_from_bin(rom_file);

    let duelist_info = duelist::read_all_duelists(&slus, &wa_mrg);

    duelist::write_all_duelists(&mut wa_mrg, &duelist_info);

    image::write_slus_to_bin(rom_file, &slus);
    image::write_wa_mrg_to_bin(rom_file, &wa_mrg);
}
