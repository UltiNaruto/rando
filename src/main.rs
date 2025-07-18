#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use clap::Parser;
use pseudoregalia_rando::patch_from_config;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to config
    #[arg(short, long)]
    config_path: String,

    /// Path to game
    #[arg(short, long)]
    game_path: String,
}

fn usage() -> Result<(), String>
{
    panic!("Usage: \
         \tpseudoregalia-rando.exe --config-path <path to config file> --game-path <path to pseudoregalia.exe>")
}

fn main() -> Result<(), String> {
    let args = Args::try_parse();
    if args.is_err() {
        usage()
    } else {
        let args_ = args.unwrap();
        let config_path: PathBuf = args_.config_path.into();
        let mut json_file = File::open(config_path).unwrap();
        let mut json = String::new();
        if json_file.read_to_string(&mut json).is_err() {
            panic!("Could not read json file");
        }
        patch_from_config(&json, args_.game_path.into())
    }
}
