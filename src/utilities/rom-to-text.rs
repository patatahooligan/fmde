use std::env;
use std::fs;

use fmde::text::u8_to_char;

fn print_usage() {
    println!("Usage: fmde FORBIDDEN_MEMORIES_ROM");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print_usage();
    }

    let file_path = &args[1];
    let rom_file = fs::read(file_path).expect("Failed to load file");

    for byte in rom_file {
        print!("{}", u8_to_char(byte));
    }
}
