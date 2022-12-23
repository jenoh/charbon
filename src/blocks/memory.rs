use tui::{
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Widget},
};
extern crate sys_info;

use sys_info::*;

#[derive(Default)]
pub struct Memory {
    pub total_memory: String,
    pub used_memory: String,
    pub total_swap: String,
    pub used_swap: String,
}

impl Memory {
    pub fn render() -> impl Widget {
        let mem = mem_info().unwrap();
        let u: f64 = ((mem.total as f64 - mem.avail as f64) / mem.total as f64) * 100.00;
        return Gauge::default()
            .block(Block::default().title("Memory usage").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Blue))
            .percent(u as u16);
    }
}
