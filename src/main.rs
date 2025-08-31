use std::fs;
use std::io::Write;

use clap::{Parser, Subcommand};

use fmde::*;

/// CLI utility to mod a .bin/.cue image of the PSX game Yu-Gi-Oh!
/// Forbidden Memories. The ROM must be the US version: SLUS-01411.
#[derive(Parser)]
#[command(version)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Read the ROM and dump all of its data in csv files.
    Dump {
        /// Path of the ROM file.
        rom_path: String,

        /// Directory in which to dump the data. If it doesn't
        /// exist it will be created.
        dump_dir: std::path::PathBuf,
    },

    /// Extract the data from the ROM and rewrite them in. Useful only
    /// for debugging read/write functionality.
    Passthrough {
        /// Path of the ROM file.
        rom_path: String,

        /// Path to save the output.
        output_path: std::path::PathBuf,
    },
}

fn dump_data(rom_path: &String, dump_dir: &std::path::Path) {
    // TODO: this loads the entire file in memory. Try accessing it
    // directly from the disc.
    let rom_file = fs::read(rom_path).expect("Failed to load file");

    let slus = image::read_slus_from_bin(&rom_file);
    let wa_mrg = image::read_wa_mrg_from_bin(&rom_file);

    let duelist_info = duelist::read_all_duelists(&slus, &wa_mrg);
    let card_names = duelist::get_card_names(&slus);

    duelist::dump_all_duelists_csv(&dump_dir, &duelist_info, &card_names);
}

fn passthrough_test(rom_path: &String, output_path: &std::path::Path) {
    let mut rom_file = fs::read(rom_path).expect("Failed to load file");

    testing::passthrough_test(&mut rom_file);
    let mut output_file = fs::File::create_new(output_path).unwrap();
    output_file.write_all(&rom_file).unwrap();
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Dump { rom_path, dump_dir } => {
            dump_data(&rom_path, &dump_dir);
        }
        Command::Passthrough {
            rom_path,
            output_path,
        } => {
            passthrough_test(&rom_path, &output_path);
        }
    }
}
