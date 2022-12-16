use crate::blocks::system_info;
use tui::widgets::Widget;

extern crate strum;

#[derive(EnumString)]
pub enum Block {
    System,
}

pub fn select_feature(selected: Block) -> impl Widget {
    match selected {
        Block::System => system_info::SystemInfo::render(),
    }
}
