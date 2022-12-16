use crate::blocks::blocks;
use crate::config::config;

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
        let feature: blocks::Block = b.parse().unwrap();
        f.render_widget(blocks::select_feature(feature), chunks[i]);
    }
}
