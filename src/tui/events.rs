use crossterm::event::{KeyCode, KeyEvent};
use super::app::{App, FormStep};

fn next_step(step: &FormStep) -> FormStep {
    match step {
        FormStep::CodeFlow => FormStep::DbChanges,
        FormStep::DbChanges => FormStep::Extensibility,
        FormStep::Extensibility => FormStep::PerfNotes,
        FormStep::PerfNotes => FormStep::PerfNotes,
    }
}

fn prev_step(step: &FormStep) -> FormStep {
    match step {
        FormStep::CodeFlow => FormStep::CodeFlow,
        FormStep::DbChanges => FormStep::CodeFlow,
        FormStep::Extensibility => FormStep::DbChanges,
        FormStep::PerfNotes => FormStep::Extensibility,
    }
}

pub fn handle_input(key: KeyEvent, app: &mut App) -> Result<bool, Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Esc => return Ok(false),
        KeyCode::Char(c) => app.input_buffer.push(c),
        KeyCode::Backspace => { app.input_buffer.pop(); },
        KeyCode::Left => app.current_step = prev_step(&app.current_step),
        KeyCode::Right => {
            // move to next step and clear buffer
            app.current_step = next_step(&app.current_step);
            app.input_buffer.clear();
        },
        _ => {}
    }
    Ok(true)
}
