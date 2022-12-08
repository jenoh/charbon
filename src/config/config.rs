use serde::Deserialize;
use tui::layout::Constraint;

#[derive(Default, Debug, Deserialize)]
pub struct Config {
    tick_rate: u32,
    layout: Vec<Layout>,
}

#[derive(Default, Debug, Deserialize)]
pub struct Layout {
    app_name: String,
    size: u16,
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
    /*
     * Converte layout size to Vec<Constraint>
     */
    pub fn get_contraints(&mut self) -> Vec<Constraint> {
        let mut vec: Vec<Constraint> = Vec::new();
        for l in &self.layout {
            vec.push(Constraint::Percentage(l.size))
        }
        return vec;
    }
    /*
     * Setters
     */
    fn set_config(&mut self, config: Config) {
        self.tick_rate = config.tick_rate;
        self.layout = config.layout;
    }
}
