use config::config::Config;
use std::{error::Error, io};
mod config;

fn main() -> Result<(), Box<dyn Error>> {
    let mut config = Config::default();
    config.get_config();
    println!("{:?}", config);
    Ok(())
}
