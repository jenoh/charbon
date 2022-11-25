use serde::Deserialize;
use std::fs;
use std::process::exit;

#[derive(Default, Deserialize)]
pub struct Config {
    tick_rate: i32,
}

impl Config {
    /*
     * Parse the config file to get parameters
     */
    pub fn get_config(&mut self) {
        let path_config_file: &str = "/etc/charbon/config.toml";

        let contents: String = match fs::read_to_string(path_config_file) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Could not read file `{}`", path_config_file);
                exit(1);
            }
        };

        let data: Config = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(_) => {
                eprintln!("Unable to read the config file `{}`", path_config_file);
                exit(1);
            }
        };

        self.set_tick_rate(data.tick_rate)
    }
    /*
     * Getters and setters
     */
    fn set_tick_rate(&mut self, tick_rate: i32) {
        self.tick_rate = tick_rate;
    }
    pub fn get_tick_rate(self) -> i32 {
        return self.tick_rate;
    }
}
