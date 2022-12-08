use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame, Terminal,
};

use config::config::Config;
use config::config::CustomLayout;

mod config;

fn main() -> Result<(), Box<dyn Error>> {
    // get config
    let mut config = Config::default();
    config.get_config();
    println!("{:?}", config);
    let c = config.get_custom_layout();
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal, c);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, c: CustomLayout) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, c.clone()))?;
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, c: CustomLayout) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(c.custom_constraints.as_ref())
        .split(f.size());
    for (i, b) in c.app_name.iter().enumerate() {
        let block = Block::default().title(b.clone()).borders(Borders::ALL);
        f.render_widget(block, chunks[i]);
    }
}
