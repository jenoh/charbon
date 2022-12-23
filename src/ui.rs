use crate::blocks;
use crate::config::config;

use tui::{
    backend::Backend,
    layout::{Direction, Layout},
    Frame,
};

pub fn ui<B: Backend>(f: &mut Frame<B>, cl: config::CustomLayout) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(cl.custom_constraints.as_ref())
        .split(f.size());

    for (i, b) in cl.app_name.iter().enumerate() {
        match b.parse().unwrap() {
            blocks::blocks::Block::System => {
                f.render_widget(blocks::system_info::SystemInfo::render(), chunks[i])
            }
            blocks::blocks::Block::Memory => {
                f.render_widget(blocks::memory::Memory::render(), chunks[i])
            }
        }
    }
}
