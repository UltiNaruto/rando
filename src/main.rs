#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use clap::Parser;
use pseudoregalia_rando::config::PatchConfig;
use pseudoregalia_rando::{writing, Rando};

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

pub fn patch_from_config(json: &String, game_path: PathBuf) -> Result<(), String> {
    let config = PatchConfig::from_json((*json).as_str())?;

    writing::write_from_config(config, game_path)
            .map_err(|e| e.to_string())
}

fn main() -> Result<(), String> {
    let args = Args::try_parse();
    if args.is_err() {
        eframe::run_native(
            "",
            eframe::NativeOptions {
                // initial_window_size: Some(eframe::epaint::Vec2::new(420.0, 315.0)),
                viewport: eframe::egui::ViewportBuilder::default()
                    .with_icon(eframe::egui::IconData {
                        rgba: include_bytes!("assets/sybil.rgba").to_vec(),
                        width: 32,
                        height: 32,
                    })
                    .with_app_id("pseudoregalia-rando"),
                ..Default::default()
            },
            Box::new(|ctx| Box::new(Rando::new(ctx))),
        ).map_err(|e| e.to_string())
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
