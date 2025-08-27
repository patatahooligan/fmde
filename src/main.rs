use std::env;
use std::fs;
use std::io::Write;

use fmde::*;

fn print_usage() {
    println!("Usage: fmde FORBIDDEN_MEMORIES_ROM PATH_TO_SAVE_MOD");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        print_usage();
        return;
    }

    let rom_path = &args[1];
    let output_path = &args[2];

    // TODO: this loads the entire file in memory. Try accessing it
    // directly from the disc.
    let mut rom_file = fs::read(rom_path).expect("Failed to load file");

    print_game_data(&rom_file);
    testing::passthrough_test(&mut rom_file);

    let mut output_file = fs::File::create_new(output_path).unwrap();
    output_file.write_all(&rom_file).unwrap();
}
