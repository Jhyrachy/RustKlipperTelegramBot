use crate::structures::SettingsStruct;
use std::fs;
use serde_json::Result;

pub fn read_settings_file() -> Result<SettingsStruct> {
    // Lettura file di settaggi
    let settings_json = fs::read_to_string("settings.config").expect("Unable to read file");

    // Parsing del json
    let settings_structure: SettingsStruct = serde_json::from_str(&settings_json).expect("Unable to parse .json file");

    Ok(settings_structure)
}