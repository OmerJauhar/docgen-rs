use crossterm::event::{KeyCode, KeyEvent};
use super::app::{App, FormStep, MainView};
use rfd::FileDialog;

fn next_step(step: &FormStep) -> FormStep {
    match step {
        FormStep::ProblemStatement => FormStep::HighLevelOverview,
        FormStep::HighLevelOverview => FormStep::CodeStructure,
        FormStep::CodeStructure => FormStep::KeyChanges,
        FormStep::KeyChanges => FormStep::FutureConsiderations,
        FormStep::FutureConsiderations => FormStep::FutureConsiderations,
    }
}

fn prev_step(step: &FormStep) -> FormStep {
    match step {
        FormStep::ProblemStatement => FormStep::ProblemStatement,
        FormStep::HighLevelOverview => FormStep::ProblemStatement,
        FormStep::CodeStructure => FormStep::HighLevelOverview,
        FormStep::KeyChanges => FormStep::CodeStructure,
        FormStep::FutureConsiderations => FormStep::KeyChanges,
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
    let idx = app.current_step as usize;
    match key.code {
        KeyCode::Esc => return Ok(false),
        KeyCode::Char('0') => {
            if let Some(folder) = FileDialog::new().pick_folder() {
                app.project_folder = folder.display().to_string();
                app.git_status = format!("Opened folder: {}", folder.display());
            }
        },
        KeyCode::Char(c) => app.input_buffers[idx].push(c),
        KeyCode::Backspace => { app.input_buffers[idx].pop(); },
        KeyCode::Left => app.main_view = prev_view(app.main_view),
        KeyCode::Right => app.main_view = next_view(app.main_view),
        KeyCode::Up => app.current_step = prev_step(&app.current_step),
        KeyCode::Down => {
            app.current_step = next_step(&app.current_step);
            // app.input_buffers[idx].clear(); // Don't clear on next
        },
        _ => {}
    }
    Ok(true)
}
