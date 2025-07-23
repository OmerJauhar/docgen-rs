use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use super::app::{App, FormStep};
use super::widgets;

pub fn render_ui(f: &mut Frame, app: &mut App)
 {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(2),
            Constraint::Length(3),
        ])
        .split(f.area()); // instead of f.size()

    widgets::draw_header(f, chunks[0]);
    
    match app.current_step {
        FormStep::CodeFlow => widgets::draw_input_form("Describe code flow", &app.input_buffer, chunks[1], f),
        FormStep::DbChanges => widgets::draw_input_form("Any DB changes?", &app.input_buffer, chunks[1], f),
        FormStep::Extensibility => widgets::draw_input_form("How can it be extended?", &app.input_buffer, chunks[1], f),
        FormStep::PerfNotes => widgets::draw_input_form("Performance or security concerns?", &app.input_buffer, chunks[1], f),
    }

    widgets::draw_footer(f, chunks[2]);
}
