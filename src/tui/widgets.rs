use ratatui::{
    layout::Rect,
    Frame,
    widgets::{Block, Borders, Paragraph, Wrap, List, ListItem, Gauge},
    text::{Span, Line},
    style::{Color, Modifier, Style},
};
use super::app::{App, MainView, FileImpact, TokenStats};

pub fn draw_header(f: &mut Frame, area: Rect, branch: &str, status: &str, project_folder: &str) {
    use ratatui::layout::{Alignment};
    let branch_span = Span::styled(
        format!(" {}", branch),
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
    );
    let status_span = Span::styled(
        format!("{}", status),
        Style::default().fg(Color::Yellow),
    );
    let title = Span::styled(
        format!("📄 docgen-rs - AI Developer Docs  |  Project: {}", project_folder),
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    );
    let line = Line::from(vec![title]);
    let branch_line = Line::from(vec![branch_span, Span::raw("  |  "), status_span]);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .title_alignment(Alignment::Center);
    let lines = vec![line, branch_line];
    let para = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Center);
    f.render_widget(para, area);
}

// pub fn draw_input_form(label: &str, input: &str, area: Rect, f: &mut Frame) {
//     let block = Block::default()
//         .title(label)
//         .borders(Borders::ALL);
//     let para = Paragraph::new(input.to_string())
//         .block(block)
//         .wrap(Wrap { trim: true });
//     f.render_widget(para, area);
//     f.set_cursor_position((area.x + 1 + input.len() as u16, area.y + 1)); // dynamic cursor
// }

pub fn draw_flow_diagram(f: &mut Frame, area: Rect, step: usize) {
    let steps = [
        "Git Diff", "Metadata Form", "LLM Analysis", "Markdown Gen", "PDF Export"
    ];
    let mut spans = vec![];
    for (i, s) in steps.iter().enumerate() {
        let style = if i == step {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else {
            Style::default().fg(Color::Gray)
        };
        spans.push(Span::styled(format!("[ {} ]", s), style));
        if i < steps.len() - 1 {
            spans.push(Span::raw(" → "));
        }
    }
    let block = Block::default()
        .title(Span::styled("⚙️  Code Change Flow", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL);
    let para = Paragraph::new(Line::from(spans)).block(block).alignment(ratatui::layout::Alignment::Center);
    f.render_widget(para, area);
}

pub fn draw_heatmap_panel(f: &mut Frame, area: Rect, impacts: &[FileImpact]) {
    let block = Block::default()
        .title(Span::styled("📊 Module Impact Heatmap", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL);
    let max_score = impacts.iter().map(|i| i.impact_score).max().unwrap_or(1) as f64;
    let items: Vec<ListItem> = impacts.iter().map(|i| {
        let color = if i.impact_score as f64 > 0.7 * max_score {
            Color::Red
        } else if i.impact_score as f64 > 0.3 * max_score {
            Color::Yellow
        } else {
            Color::Green
        };
        let bar_len = ((i.impact_score as f64 / max_score) * 20.0).ceil() as usize;
        let bar = "█".repeat(bar_len);
        let text = format!("{}  +{} -{}  fns:{}  {}", i.filename, i.lines_added, i.lines_removed, i.functions_modified, bar);
        ListItem::new(Span::styled(text, Style::default().fg(color)))
    }).collect();
    let list = List::new(items).block(block);
    f.render_widget(list, area);
}

pub fn draw_llm_stats_panel(f: &mut Frame, area: Rect, stats: &TokenStats) {
    let block = Block::default()
        .title(Span::styled("💸 LLM Token Cost Estimator", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL);
    let color = if stats.total_tokens < 2000 {
        Color::Green
    } else if stats.total_tokens < 8000 {
        Color::Yellow
    } else {
        Color::Red
    };
    let gauge = Gauge::default()
        .block(block)
        .gauge_style(Style::default().fg(color).bg(Color::Black).add_modifier(Modifier::BOLD))
        .label(Span::styled(format!("{} tokens", stats.total_tokens), Style::default().fg(color).add_modifier(Modifier::BOLD)))
        .ratio((stats.total_tokens as f64 / 16000.0).min(1.0));
    f.render_widget(gauge, area);
    // Show details below gauge
    let details = vec![
        Line::from(vec![Span::raw("diff_tokens: "), Span::styled(stats.diff_tokens.to_string(), Style::default().fg(Color::Cyan))]),
        Line::from(vec![Span::raw("form_tokens: "), Span::styled(stats.form_tokens.to_string(), Style::default().fg(Color::Cyan))]),
        Line::from(vec![Span::raw("total_tokens: "), Span::styled(stats.total_tokens.to_string(), Style::default().fg(color))]),
    ];
    let para = Paragraph::new(details).alignment(ratatui::layout::Alignment::Left);
    let detail_area = Rect { y: area.y + area.height.saturating_sub(4), height: 4, ..area };
    f.render_widget(para, detail_area);
}

pub fn draw_form_panel(f: &mut Frame, area: Rect, app: &App) {
    let label = match app.current_step {
        super::app::FormStep::CodeFlow => "Describe code flow:",
        super::app::FormStep::DbChanges => "Any DB changes?",
        super::app::FormStep::Extensibility => "How can it be extended?",
        super::app::FormStep::PerfNotes => "Performance or security concerns?",
    };
    let block = Block::default()
        .title(Span::styled(label, Style::default().fg(Color::White)))
        .borders(Borders::ALL);
    let para = Paragraph::new(app.input_buffer.clone())
        .block(block)
        .wrap(Wrap { trim: true });
    f.render_widget(para, area);
    f.set_cursor_position((area.x + 1 + app.input_buffer.len() as u16, area.y + 1));
}

pub fn draw_footer(f: &mut Frame, area: Rect, view: MainView) {
    let tabs = ["Form", "Git Diff Viewer", "LLM Stats"];
    let mut spans = vec![];
    for (i, tab) in tabs.iter().enumerate() {
        let style = if view as usize == i {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };
        spans.push(Span::styled(format!(" {} ", tab), style));
        if i < tabs.len() - 1 {
            spans.push(Span::raw(" | "));
        }
    }
    let para = Paragraph::new(Line::from(spans)).alignment(ratatui::layout::Alignment::Center);
    f.render_widget(para, area);
    // Navigation hint
    let hint = Paragraph::new("Tab: ←/→ • ↑/↓: Next/Prev Question • 0: Change Project Path • Esc: Quit").style(Style::default().fg(Color::LightGreen));
    let hint_area = Rect { y: area.y + area.height.saturating_sub(1), height: 1, ..area };
    f.render_widget(hint, hint_area);
}
