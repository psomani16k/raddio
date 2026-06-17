mod args;
mod config;
mod ui;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

use crate::args::Args;
use crate::config::CONFIG;
use crate::ui::App;

fn main() -> anyhow::Result<()> {
    let args = Args::parse_args();

    let station = CONFIG
        .stations
        .iter()
        .find(|s| s.name == args.station)
        .ok_or_else(|| anyhow::anyhow!("no station named '{}'", args.station))?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new(station);

    app.run(&mut terminal)?;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    app.execute()?;
    Ok(())
}
