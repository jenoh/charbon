use crate::config::config;
use crate::features::features;

use tui::{
    backend::Backend,
    layout::{Direction, Layout},
    Frame,
};

pub fn ui<B: Backend>(f: &mut Frame<B>, c: config::CustomLayout) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(c.custom_constraints.as_ref())
        .split(f.size());

    for (i, b) in c.app_name.iter().enumerate() {
        let feature: features::Features = b.parse().unwrap();
        f.render_widget(features::select_feature(feature), chunks[i]);
    }
}
