#![cfg_attr(
    all(
        target_os = "windows",
        not(feature = "console"),
    ),
    windows_subsystem = "windows"
)]

mod config;
mod wallpapers_switcher;

use device_query::{DeviceState};
use log::error;
use crate::wallpapers_switcher::WallpapersSwitcher;


fn main() {
    env_logger::init();
    log_panics::init();

    let config = config::Config::load("./config.toml").unwrap();
    if config.do_check {
        let mut has_errors = false;
        for preset in &config.presets {
            let valid = match imghdr::from_file(preset.1.path.as_str()) {
                Err(_) => false,
                Ok(option) => option.is_some()
            };

            if !valid {
                error!("Preset ({}) wallpaper file doesn't exist or isn't image.", preset.0);
                has_errors = true;
            }
        }

        if has_errors {
            return;
        }
    }

    let mut switcher = WallpapersSwitcher::from_config(&config).unwrap();

    let device_state = DeviceState::new();
    loop {
        switcher.check(&device_state);
    }
}