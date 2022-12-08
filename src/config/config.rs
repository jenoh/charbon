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
#[derive(Default, Clone)]
pub struct CustomLayout {
    pub custom_constraints: Vec<Constraint>,
    pub app_name: Vec<String>,
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
     * Converte layout size to CustomLayout
     */
    pub fn get_custom_layout(&mut self) -> CustomLayout {
        let mut vec_constraint: Vec<Constraint> = Vec::new();
        let mut vec_app_name: Vec<String> = Vec::new();
        for l in &self.layout {
            vec_app_name.push(l.app_name.clone());
            vec_constraint.push(Constraint::Percentage(l.size))
        }
        return CustomLayout {
            app_name: vec_app_name,
            custom_constraints: vec_constraint,
        };
    }
    /*
     * Setters
     */
    fn set_config(&mut self, config: Config) {
        self.tick_rate = config.tick_rate;
        self.layout = config.layout;
    }
}
