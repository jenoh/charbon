use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
pub struct Config {
    tick_rate: i32,
    layout: Vec<Layout>,
}

#[derive(Default, Debug, Deserialize)]
pub struct Layout {
    app_name: String,
    size: i8,
}

impl Config {
    /*
     * Parse the config file to get parameters
     */
    pub fn get_config(&mut self) {
        let path_config_file: &str = "/etc/charbon/config.yaml";
        let f = std::fs::File::open(path_config_file).expect("Could not open file.");
        self.set_config(serde_yaml::from_reader(f).expect("Could not read values."));
    }
    fn set_config(&mut self, config: Config) {
        self.tick_rate = config.tick_rate;
        self.layout = config.layout;
    }
}
