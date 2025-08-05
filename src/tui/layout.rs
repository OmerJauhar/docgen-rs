use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use super::app::{App, MainView};
use super::widgets;

pub fn render_ui(f: &mut Frame, app: &mut App) {
    // Top: Header
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(4), // Header
            Constraint::Length(4), // Flow diagram
            Constraint::Min(10),  // Main content
            Constraint::Length(3), // Footer
        ])
        .split(f.area());

    widgets::draw_header(f, main_chunks[0], &app.current_branch, &app.git_status, &app.project_folder);
    widgets::draw_flow_diagram(f, main_chunks[1], app.flow_step);

    // Main content area: split horizontally for main panel and side panel
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(65),
            Constraint::Percentage(35),
        ])
        .split(main_chunks[2]);

    match app.main_view {
        MainView::BranchSelection => {
            widgets::draw_branch_selection_panel(f, content_chunks[0], app);
            widgets::draw_llm_stats_panel(f, content_chunks[1], &app.token_stats);
        },
        MainView::Form => {
            widgets::draw_form_panel(f, content_chunks[0], app);
            widgets::draw_llm_stats_panel(f, content_chunks[1], &app.token_stats);
        },
        MainView::GitDiff => {
            widgets::draw_git_diff_panel(f, content_chunks[0], app);
            widgets::draw_llm_stats_panel(f, content_chunks[1], &app.token_stats);
        },
        MainView::LLMStats => {
            widgets::draw_llm_stats_panel(f, content_chunks[0], &app.token_stats);
            widgets::draw_heatmap_panel(f, content_chunks[1], &app.file_impacts);
        },
        MainView::UserInfo => {
            // Use full content area for user info
            widgets::draw_user_info_panel(f, app, main_chunks[2]);
        },
    }

    // Show user info prompt if needed
    if app.show_user_info_prompt {
        widgets::draw_user_info_prompt(f, f.area());
    }

    widgets::draw_footer(f, main_chunks[3], app.main_view);
}
