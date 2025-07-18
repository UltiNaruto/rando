use crate::config::PatchConfig;

pub mod config;
mod io;
mod logic;
mod map;
mod writing;

type Asset<T> = unreal_asset::Asset<std::io::Cursor<T>>;

pub fn patch_from_config(json: &String, game_path: String) -> Result<(), String> {
    let config = PatchConfig::from_json((*json).as_str(), game_path.into())?;

    writing::write_from_config(config)
        .map_err(|e| e.to_string())
}
