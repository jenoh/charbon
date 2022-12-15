use sysinfo::{System, SystemExt};
use tui::{
    layout::Constraint,
    style::{Color, Style},
    widgets::{Block, Borders, Row, Table},
};

#[derive(Default)]
pub struct SystemInfo {
    pub name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub host_name: String,
}

impl SystemInfo {
    pub fn render() -> Table<'static> {
        let mut sys = System::new_all();
        sys.refresh_all();
        let l = vec![SystemInfo {
            name: sys.name().unwrap(),
            kernel_version: sys.kernel_version().unwrap(),
            os_version: sys.os_version().unwrap(),
            host_name: sys.host_name().unwrap(),
        }];
        let rows = l.iter().map(|s| {
            Row::new(vec![
                s.name.clone(),
                s.kernel_version.clone(),
                s.os_version.clone(),
                s.host_name.clone(),
            ])
        });
        return Table::new(rows)
            .header(
                Row::new(vec!["Name", "Kernel version", "OS version", "Host name"])
                    .style(Style::default().fg(Color::Yellow))
                    .bottom_margin(0),
            )
            .block(Block::default().title("System").borders(Borders::ALL))
            .widths(&[
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]);
    }
}
