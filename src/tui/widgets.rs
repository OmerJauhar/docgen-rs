use ratatui::{
    layout::Rect,
    Frame,
    widgets::{Block, Borders, Paragraph, Wrap},
    text::Span,
    style::{Color, Modifier, Style},
};

pub fn draw_header(f: &mut Frame, area: Rect)
 {
    let block = Block::default()
        .title(Span::styled("📄 docgen-rs - AI Developer Docs",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL);
    f.render_widget(block, area);
}

pub fn draw_input_form(label: &str, input: &str, area: Rect, f: &mut Frame) {
    let block = Block::default()
        .title(label)
        .borders(Borders::ALL);
    let para = Paragraph::new(input.to_string())
        .block(block)
        .wrap(Wrap { trim: true });
    f.render_widget(para, area);
    f.set_cursor_position((area.x + 1 + input.len() as u16, area.y + 1)); // dynamic cursor
}
pub fn draw_footer(f: &mut Frame, area: Rect) {
    let text = Paragraph::new("← → to Navigate • Esc to Quit")
        .style(Style::default().fg(Color::LightGreen));
    f.render_widget(text, area);
}
