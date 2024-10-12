use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Config {
    presets: Vec<Preset>
}

#[derive(Deserialize)]
pub struct Preset {
    pub keys: String,
    pub path: String
}