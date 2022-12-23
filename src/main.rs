use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{backend::CrosstermBackend, Terminal};
// Strum contains all the trait definitions
extern crate strum;
#[macro_use]
extern crate strum_macros;
extern crate sys_info;
use config::config::Config;
mod app;
mod blocks;
mod config;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    // get config
    let mut config = Config::default();
    config.get_config();
    let c = config.get_custom_layout();
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = app::run_app(&mut terminal, c);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }
    Ok(())
}
