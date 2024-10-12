use std::error::Error;
use std::str::FromStr;
use device_query::{DeviceQuery, DeviceState, Keycode};
use log::{error, info};
use crate::config::Config;

pub struct WallpapersSwitcher {
    presets: Vec<Preset>
}

impl WallpapersSwitcher {
    pub fn new(presets: Vec<Preset>) -> Self {
        WallpapersSwitcher {
            presets
        }
    }

    pub fn from_config(config: &Config) -> Result<Self, Box<dyn Error>> {
        let presets = config.presets.iter()
            .map(|data| {Preset::from_config(data.1, data.0.clone())})
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::new(presets))
    }

    pub fn check(&mut self, state: &DeviceState) {
        let keys = state.get_keys();
        for preset in &mut self.presets {
            let mut pressed = true;

            for key in &preset.keys {
                if !keys.contains(key) {
                    pressed = false;
                    break
                }
            }

            if !preset.pressed && pressed {
                info!("Preset selected {}", preset.name);
                if let Err(err) = wallpaper::set_from_path(preset.path.as_str()) {
                    error!("Error while try set wallpaper: {}", err)
                }
            }

            preset.pressed = pressed;
        }
    }
}

pub struct Preset {
    keys: Vec<Keycode>,
    path: String,
    pressed: bool,
    name: String
}

impl Preset {
    pub fn new(keys: Vec<Keycode>, path: String, name: String) -> Self {
        Preset {
            keys, path, name,
            pressed: false
        }
    }
    fn from_config(value: &crate::config::Preset, name: String) -> Result<Self, Box<dyn Error>> {
        let keys = value.keys.split("+")
            .map(Keycode::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::new(keys, value.path.clone(), name))
    }
}