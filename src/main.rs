use std::env;
use std::fs;

use fmde::*;

fn print_usage() {
    println!("Usage: fmde FORBIDDEN_MEMORIES_ROM");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print_usage();
    }

    let file_path = &args[1];
    // TODO: this loads the entire file in memory. Try seeking accessing it
    // directly from the disc.
    let rom_file = fs::read(file_path).expect("Failed to load file");

    print_card_names(&rom_file);
}
