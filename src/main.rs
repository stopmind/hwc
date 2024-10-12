mod config;
mod wallpapers_switcher;

use device_query::{DeviceQuery, DeviceState};
use crate::wallpapers_switcher::WallpapersSwitcher;

fn main() {
    env_logger::init();
    log_panics::init();

    let device_state = DeviceState::new();
    let config = config::Config::load("./config.toml").unwrap();
    let mut switcher = WallpapersSwitcher::from_config(&config).unwrap();

    loop {
        switcher.check(&device_state);
    }
}
