mod tui;

use std::io;
use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::Event,
};
use ratatui::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Terminal Setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = tui::app::App::new();

    loop {
        terminal.draw(|f| tui::layout::render_ui(f, &mut app))?;
    
        if let Event::Key(key) = crossterm::event::read()? {
            if !tui::events::handle_input(key, &mut app)? {
                break;
            }
        }
    }
    

    // Cleanup
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
// Test comment for real-time token update
