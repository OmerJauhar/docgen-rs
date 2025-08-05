use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
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
        MainView::BranchSelection => MainView::Form,
        MainView::Form => MainView::GitDiff,
        MainView::GitDiff => MainView::LLMStats,
        MainView::LLMStats => MainView::BranchSelection,
        MainView::UserInfo => MainView::BranchSelection, // UserInfo returns to BranchSelection
    }
}
fn prev_view(view: MainView) -> MainView {
    match view {
        MainView::BranchSelection => MainView::LLMStats,
        MainView::Form => MainView::BranchSelection,
        MainView::GitDiff => MainView::Form,
        MainView::LLMStats => MainView::GitDiff,
        MainView::UserInfo => MainView::BranchSelection, // UserInfo returns to BranchSelection
    }
}

pub fn handle_input(key: KeyEvent, app: &mut App) -> Result<bool, Box<dyn std::error::Error>> {
    // Handle user info prompt first (if shown)
    if app.show_user_info_prompt {
        match key.code {
            KeyCode::Char('e') | KeyCode::Char('E') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                app.enter_user_info_mode();
                return Ok(true);
            },
            _ => {
                app.show_user_info_prompt = false;
                return Ok(true);
            }
        }
    }

    match key.code {
        KeyCode::Esc => {
            // Exit user info mode if in that mode
            if app.main_view == MainView::UserInfo {
                app.main_view = MainView::BranchSelection;
                return Ok(true);
            }
            return Ok(false);
        },
        KeyCode::Char('e') | KeyCode::Char('E') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            // Open user info screen
            app.enter_user_info_mode();
        },
        KeyCode::Char('0') => {
            if let Some(folder) = FileDialog::new().pick_folder() {
                let folder_path = folder.display().to_string();
                app.project_folder = folder_path.clone();
                app.git_status = format!("Opened folder: {}", folder.display());
                // Load branches for the selected project
                if let Err(e) = app.load_branches(&folder_path) {
                    app.git_status = format!("Error loading branches: {}", e);
                    app.available_branches.clear();
                } else {
                    app.git_status = format!("Loaded {} branches from {}", app.available_branches.len(), folder.display());
                }
                // Switch to branch selection view
                app.main_view = MainView::BranchSelection;
            }
        },
        KeyCode::Char('r') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            // Refresh git diff
            if let Err(e) = app.load_git_diff() {
                app.git_status = format!("Error loading git diff: {}", e);
            } else {
                app.git_status = format!("Git diff refreshed: {}", app.git_diff_data.summary());
            }
        },
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            // Submit form or save user info
            if app.main_view == MainView::UserInfo {
                match app.save_user_info() {
                    Ok(()) => {
                        app.main_view = MainView::BranchSelection;
                    },
                    Err(e) => {
                        app.git_status = format!("Save error: {}", e);
                    }
                }
            } else {
                // Submit form
                match app.attempt_submit() {
                    Ok(true) => {
                        app.git_status = "Successfully submitted to LLM!".to_string();
                    },
                    Ok(false) => {
                        app.git_status = "Cannot submit: Please fix validation errors".to_string();
                    },
                    Err(e) => {
                        app.git_status = format!("Submit error: {}", e);
                    }
                }
            }
        },
        KeyCode::Left => app.main_view = prev_view(app.main_view),
        KeyCode::Right => app.main_view = next_view(app.main_view),
        _ => {
            // Handle input based on current view
            match app.main_view {
                MainView::BranchSelection => handle_branch_selection_input(key, app)?,
                MainView::Form => handle_form_input(key, app)?,
                MainView::UserInfo => handle_user_info_input(key, app)?,
                _ => {} // Other views don't have specific input handling yet
            }
        }
    }
    Ok(true)
}

fn handle_branch_selection_input(key: KeyEvent, app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Up => {
            if app.selected_branch_index > 0 {
                app.selected_branch_index -= 1;
            }
        },
        KeyCode::Down => {
            if app.selected_branch_index < app.available_branches.len().saturating_sub(1) {
                app.selected_branch_index += 1;
            }
        },
        KeyCode::Enter => {
            app.select_current_branch();
            app.git_status = format!("Selected branch: {}", app.current_branch);
            // Load git diff when branch is selected
            if let Err(e) = app.load_git_diff() {
                app.git_status = format!("Error loading git diff: {}", e);
            }
            // Move to the next view (Form)
            app.main_view = MainView::Form;
        },
        _ => {}
    }
    Ok(())
}

fn handle_form_input(key: KeyEvent, app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let idx = app.current_step as usize;
    match key.code {
        KeyCode::Char(c) => {
            app.input_buffers[idx].push(c);
            app.validate_form(); // Validate on every change
        },
        KeyCode::Backspace => { 
            app.input_buffers[idx].pop(); 
            app.validate_form(); // Validate on every change
        },
        KeyCode::Up => app.current_step = prev_step(&app.current_step),
        KeyCode::Down => app.current_step = next_step(&app.current_step),
        _ => {}
    }
    Ok(())
}

fn handle_user_info_input(key: KeyEvent, app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Char(c) => {
            app.user_info_buffer.push(c);
        },
        KeyCode::Backspace => {
            app.user_info_buffer.pop();
        },
        KeyCode::Tab => {
            app.next_user_info_field();
        },
        KeyCode::BackTab => {
            app.previous_user_info_field();
        },
        KeyCode::Up => {
            app.previous_user_info_field();
        },
        KeyCode::Down => {
            app.next_user_info_field();
        },
        _ => {}
    }
    Ok(())
}
