use crate::config::config;
use crate::features::system_info;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Row, Table},
    Frame,
};

pub fn ui<B: Backend>(f: &mut Frame<B>, c: config::CustomLayout) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(c.custom_constraints.as_ref())
        .split(f.size());

    for (i, b) in c.app_name.iter().enumerate() {
        if b == "system" {
            f.render_widget(system_info::SystemInfo::get_system_info(), chunks[i]);
        } else {
            let block = Block::default().title(b.clone()).borders(Borders::ALL);
            f.render_widget(block, chunks[i]);
        }
    }
}
