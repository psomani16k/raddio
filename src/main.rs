mod args;
mod config;
mod ui;

use crossterm::{
    event::{
        KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
    },
    execute,
    terminal::{
        EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
        supports_keyboard_enhancement,
    },
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

use crate::args::{Args, Command};
use crate::config::CONFIG;
use crate::ui::App;

fn main() -> anyhow::Result<()> {
    let args = Args::parse_args();

    match args.command {
        Command::Init => config::init(),
        Command::Run { station } => tune(&station),
    }
}

fn tune(station_name: &str) -> anyhow::Result<()> {
    let station = CONFIG
        .stations
        .iter()
        .find(|s| s.name == station_name)
        .ok_or_else(|| anyhow::anyhow!("no station named '{}'", station_name))?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    // Enable the kitty keyboard protocol where supported so the editor can
    // distinguish Ctrl+Enter (insert newline) from a plain Enter (submit).
    let kb_enhanced = supports_keyboard_enhancement().unwrap_or(false);
    if kb_enhanced {
        execute!(
            stdout,
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
        )?;
    }

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new(station);

    app.run(&mut terminal)?;

    disable_raw_mode()?;
    if kb_enhanced {
        execute!(terminal.backend_mut(), PopKeyboardEnhancementFlags)?;
    }
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    app.execute()?;
    Ok(())
}
