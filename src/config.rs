use std::collections::HashMap;
use std::error::Error;
use std::fs;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Config {
    pub presets: HashMap<String, Preset>
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str::<Config>(content.as_str())?)
    }
}

#[derive(Deserialize)]
pub struct Preset {
    pub keys: String,
    pub path: String
}