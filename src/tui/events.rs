use crossterm::event::{KeyCode, KeyEvent};
use super::app::{App, FormStep, MainView};

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

fn next_view(view: MainView) -> MainView {
    match view {
        MainView::Form => MainView::GitDiff,
        MainView::GitDiff => MainView::LLMStats,
        MainView::LLMStats => MainView::Form,
    }
}
fn prev_view(view: MainView) -> MainView {
    match view {
        MainView::Form => MainView::LLMStats,
        MainView::GitDiff => MainView::Form,
        MainView::LLMStats => MainView::GitDiff,
    }
}

pub fn handle_input(key: KeyEvent, app: &mut App) -> Result<bool, Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Esc => return Ok(false),
        KeyCode::Char(c) => app.input_buffer.push(c),
        KeyCode::Backspace => { app.input_buffer.pop(); },
        KeyCode::Left => app.main_view = prev_view(app.main_view),
        KeyCode::Right => app.main_view = next_view(app.main_view),
        KeyCode::Up => app.current_step = prev_step(&app.current_step),
        KeyCode::Down => {
            app.current_step = next_step(&app.current_step);
            app.input_buffer.clear();
        },
        _ => {}
    }
    Ok(true)
}
