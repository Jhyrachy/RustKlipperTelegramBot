use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SettingsStruct{
    pub telegram_token : String,
    pub telegram_chat_id : String,
    pub moonraker_port : String,
    pub host_system : String,
    pub camera_enabled : bool,
}

#[derive(Serialize, Deserialize)]
pub struct MoonrakeStructure{
    pub result : MoonrakeResult ,
}

#[derive(Serialize, Deserialize)]
pub struct MoonrakeResult{
    pub status : MoonrakeStatus ,
    pub eventtime : f64,
}

#[derive(Serialize, Deserialize)]
pub struct MoonrakeDisplayStatus{
    pub progress : f64,
    pub message : Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MoonrakePrintStats{
    pub print_duration : f64,
    pub total_duration : f64,
    pub filament_used : f64,
    pub filename : String,
    pub state : String,
    pub message : String,
}

#[derive(Serialize, Deserialize)]
pub struct MoonrakeHeaterBed{
    pub target : f64,
    pub temperature : f64,
}

#[derive(Serialize, Deserialize)]
pub struct MoonrakeExtruder{
    pub target : f64,
    pub temperature : f64,
}

#[derive(Serialize, Deserialize)]
pub struct MoonrakeStatus{
    pub display_status: MoonrakeDisplayStatus,
    pub print_stats: MoonrakePrintStats,
    pub heater_bed: MoonrakeHeaterBed,
    pub extruder: MoonrakeExtruder,
}