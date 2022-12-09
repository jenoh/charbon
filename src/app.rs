use crate::config::config;
use crate::ui;
use crossterm::event::{self, Event, KeyCode};
use std::io;
use tui::{backend::Backend, Terminal};

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, c: config::CustomLayout) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, c.clone()))?;
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}
