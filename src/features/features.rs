use crate::features::system_info;
use tui::widgets::Widget;

extern crate strum;

#[derive(EnumString)]
pub enum Features {
    System,
}

pub fn select_feature(selected: Features) -> impl Widget {
    match selected {
        Features::System => system_info::SystemInfo::render(),
    }
}
