use log::info;
use std::error::Error;

mod config;

use config::config::Config;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let mut config = Config::default();
    config.get_config();
    info!("Config tick_rate: {}", config.get_tick_rate());
    Ok(())
}
