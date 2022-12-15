use tui::widgets::Table;
extern crate strum;
use crate::features::system_info;

#[derive(EnumString)]
pub enum Features {
    System,
}

pub fn select_feature(choice: Features) -> Table<'static> {
    match choice {
        Features::System => system_info::SystemInfo::render(),
    }
}
