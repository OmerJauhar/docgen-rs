use ratatui::{
    layout::Rect,
    Frame,
    widgets::{Block, Borders, Paragraph, Wrap, List, ListItem, Gauge, Clear},
    text::{Span, Line},
    style::{Color, Modifier, Style},
};
use super::app::{App, MainView, FileImpact, TokenStats, UserInfoField};

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
        .title(Span::styled("📊 File Impact Heatmap", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL);
    
    if impacts.is_empty() {
        let para = Paragraph::new("No file changes detected.\n\nMake some changes to your code and press Ctrl+R to refresh.")
            .block(block)
            .style(Style::default().fg(Color::Yellow))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(para, area);
        return;
    }
    
    let max_score = impacts.iter().map(|i| i.impact_score).max().unwrap_or(1) as f64;
    let items: Vec<ListItem> = impacts.iter().map(|i| {
        // Determine color based on impact score
        let color = if i.impact_score as f64 > 0.7 * max_score {
            Color::Red
        } else if i.impact_score as f64 > 0.4 * max_score {
            Color::Yellow
        } else if i.impact_score as f64 > 0.1 * max_score {
            Color::Blue
        } else {
            Color::Green
        };
        
        // Create visual impact bar
        let bar_width = 15;
        let filled_width = ((i.impact_score as f64 / max_score) * bar_width as f64).ceil() as usize;
        let bar = "█".repeat(filled_width) + &"░".repeat(bar_width - filled_width);
        
        // Format the line with file info
        let file_name = if i.filename.len() > 30 {
            format!("{}...", &i.filename[..27])
        } else {
            i.filename.clone()
        };
        
        let text = format!(
            "{:<30} [{:<15}] +{:<3} -{:<3} fn:{:<2}",
            file_name,
            bar,
            i.lines_added,
            i.lines_removed,
            i.functions_modified
        );
        
        ListItem::new(Span::styled(text, Style::default().fg(color)))
    }).collect();
    
    let list = List::new(items).block(block);
    f.render_widget(list, area);
}

pub fn draw_llm_stats_panel(f: &mut Frame, area: Rect, stats: &TokenStats) {
    // Determine color based on token count and cost
    let (color, warning_text) = if stats.total_tokens < 1000 {
        (Color::Green, "Low cost")
    } else if stats.total_tokens < 4000 {
        (Color::Yellow, "Moderate cost")
    } else if stats.total_tokens < 8000 {
        (Color::Rgb(255, 165, 0), "High cost") // Orange
    } else {
        (Color::Red, "Very high cost!")
    };
    
    // Create main content area for the gauge
    let main_area = Rect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: area.height.saturating_sub(8), // Leave space for details (increased from 6 to 8)
    };
    
    let gauge = Gauge::default()
        .block(Block::default()
            .borders(Borders::ALL)
            .title(Span::styled("💸 LLM Token Cost Estimator", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)))
        )
        .gauge_style(Style::default().fg(color).bg(Color::Black).add_modifier(Modifier::BOLD))
        .label(Span::styled(
            format!("{} tokens ({})", stats.total_tokens, warning_text), 
            Style::default().fg(color).add_modifier(Modifier::BOLD)
        ))
        .ratio((stats.total_tokens as f64 / 16000.0).min(1.0)); // Max scale of 16k tokens
    
    f.render_widget(gauge, main_area);
    
    // Show detailed breakdown below gauge
    let detail_area = Rect {
        x: area.x + 1,
        y: area.y + main_area.height + 1,
        width: area.width.saturating_sub(2),
        height: 6, // Reduced from 7 to 6 since we removed the exchange rate line
    };
    
    let details = vec![
        Line::from(vec![
            Span::styled("Form: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(format!("{} tokens", stats.form_tokens), Style::default().fg(Color::White))
        ]),
        Line::from(vec![
            Span::styled("Diff: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(format!("{} tokens", stats.diff_tokens), Style::default().fg(Color::White))
        ]),
        Line::from(vec![
            Span::styled("Total: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(format!("{} tokens", stats.total_tokens), Style::default().fg(color))
        ]),
        Line::from(vec![
            Span::styled("Cost (USD): ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(format!("${:.4}", stats.estimated_cost_usd), Style::default().fg(Color::Green))
        ]),
        Line::from(vec![
            Span::styled("Cost (PKR): ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(format!("₨{:.2}", stats.estimated_cost_pkr), Style::default().fg(Color::Green))
        ]),
    ];
    
    let para = Paragraph::new(details)
        .alignment(ratatui::layout::Alignment::Left);
    f.render_widget(para, detail_area);
}

pub fn draw_form_panel(f: &mut Frame, area: Rect, app: &App) {
    let questions = [
        ("Problem Statement", "Briefly explain the need or motivation behind the feature. What issue, gap, or requirement does it address?"),
        ("High-Level Overview", "Summarize the main logic and flow. Describe how the feature fits into the existing system and how it operates."),
        ("Code Structure", "List all significant files, directories, or modules affected or created. Highlight any structural changes."),
        ("Key Changes", "Mention any new or updated functions, endpoints, or DB schema changes. Include table/field names if applicable."),
        ("Future Considerations", "Note any design decisions, edge cases, or potential pitfalls that future developers should be aware of."),
    ];
    let idx = app.current_step as usize;
    let (title, desc) = &questions[idx];
    
    let header = Span::styled(format!("[{}]", title), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED));
    let mut lines = vec![
        Line::from(vec![header]),
        Line::from(vec![Span::styled(desc.to_string(), Style::default().fg(Color::DarkGray))]),
        Line::from(vec![Span::raw("")]), // Empty line
    ];
    
    // Add input buffer content
    let input_text = if app.input_buffers[idx].is_empty() {
        Span::styled("(Type your answer here...)", Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC))
    } else {
        Span::styled(&app.input_buffers[idx], Style::default().fg(Color::White))
    };
    lines.push(Line::from(vec![input_text]));
    
    // Add validation errors if submit was attempted
    if app.submit_attempted {
        let errors = app.get_form_errors();
        if !errors.is_empty() {
            lines.push(Line::from(vec![Span::raw("")])); // Empty line
            lines.push(Line::from(vec![Span::styled("❌ Validation Errors:", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))]));
            for error in errors {
                lines.push(Line::from(vec![Span::styled(format!("  • {}", error), Style::default().fg(Color::Red))]));
            }
        }
    }
    
    // Add submit button status
    lines.push(Line::from(vec![Span::raw("")])); // Empty line
    let submit_style = if app.can_submit {
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };
    let submit_text = if app.can_submit {
        "✅ Press Ctrl+S to Submit to LLM"
    } else {
        "⏳ Complete all fields and make git changes to submit"
    };
    lines.push(Line::from(vec![Span::styled(submit_text, submit_style)]));
    
    let block = Block::default()
        .title(Span::styled("Feature Documentation Form", Style::default().fg(Color::White)))
        .borders(Borders::ALL);
    let para = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });
    f.render_widget(para, area);
    
    // Set cursor for current question (on the input line)
    if !app.input_buffers[idx].is_empty() {
        let y = area.y + 4; // 0: header, 1: desc, 2: empty, 3: input
        let x = area.x + 1 + app.input_buffers[idx].len() as u16;
        f.set_cursor_position((x, y));
    }
}

pub fn draw_footer(f: &mut Frame, area: Rect, view: MainView) {
    let tabs = ["Branch Selection", "Form", "Git Diff Viewer", "LLM Stats", "User Info"];
    let mut spans = vec![];
    for (i, tab) in tabs.iter().enumerate() {
        let style = if (view as usize == i) || (matches!(view, MainView::UserInfo) && i == 4) {
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
    let hint = Paragraph::new("Tab: ←/→ • ↑/↓: Navigate • Ctrl+R: Refresh Git Diff • Ctrl+S: Submit • 0: Change Project • Ctrl+E: User Info • Esc: Quit").style(Style::default().fg(Color::LightGreen));
    let hint_area = Rect { y: area.y + area.height.saturating_sub(1), height: 1, ..area };
    f.render_widget(hint, hint_area);
}

pub fn draw_git_diff_panel(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(Span::styled("📋 Git Diff Viewer", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL);
    
    if app.git_diff_data.is_empty() {
        let text = "No git changes found.\n\nMake some changes to your code and press Ctrl+R to refresh.";
        let para = Paragraph::new(text)
            .block(block)
            .style(Style::default().fg(Color::Yellow))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(para, area);
        return;
    }

    let mut lines = Vec::new();
    
    // Add summary
    lines.push(Line::from(vec![
        Span::styled("Summary: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(app.git_diff_data.summary(), Style::default().fg(Color::White))
    ]));
    lines.push(Line::from(vec![Span::raw("")])); // Empty line
    
    // Add file changes
    for file_change in &app.git_diff_data.files {
        // File header
        let status_color = match file_change.status.as_str() {
            "added" => Color::Green,
            "deleted" => Color::Red,
            "modified" => Color::Yellow,
            "renamed" => Color::Blue,
            "untracked" => Color::Magenta,
            _ => Color::White,
        };
        
        lines.push(Line::from(vec![
            Span::styled(format!("📄 {} ", file_change.file_path), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled(format!("({})", file_change.status), Style::default().fg(status_color)),
            Span::styled(format!(" +{} -{}", file_change.additions, file_change.deletions), Style::default().fg(Color::Gray))
        ]));
        
        // Show first few lines of diff
        let max_lines_per_file = 5;
        for diff_line in file_change.diff_lines.iter().take(max_lines_per_file) {
            let line_style = if diff_line.starts_with('+') {
                Style::default().fg(Color::Green)
            } else if diff_line.starts_with('-') {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::Gray)
            };
            
            lines.push(Line::from(vec![Span::styled(format!("  {}", diff_line), line_style)]));
        }
        
        if file_change.diff_lines.len() > max_lines_per_file {
            lines.push(Line::from(vec![Span::styled(
                format!("  ... ({} more lines)", file_change.diff_lines.len() - max_lines_per_file),
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)
            )]));
        }
        
        lines.push(Line::from(vec![Span::raw("")])); // Empty line between files
    }
    
    let para = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });
    f.render_widget(para, area);
}

pub fn draw_branch_selection_panel(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(Span::styled("🌿 Select Branch", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL);
    
    if app.available_branches.is_empty() {
        let text = if app.project_folder == "." {
            "No project selected. Press '0' to select a project folder."
        } else {
            "No branches found in the selected project or not a git repository."
        };
        let para = Paragraph::new(text)
            .block(block)
            .style(Style::default().fg(Color::Yellow))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(para, area);
        return;
    }

    let items: Vec<ListItem> = app.available_branches.iter().enumerate().map(|(i, branch)| {
        let style = if i == app.selected_branch_index {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::REVERSED)
        } else {
            Style::default().fg(Color::White)
        };
        let text = if branch == &app.current_branch {
            format!("● {} (current)", branch)
        } else {
            format!("  {}", branch)
        };
        ListItem::new(Span::styled(text, style))
    }).collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    
    f.render_widget(list, area);
    
    // Add instructions
    let instruction_area = Rect {
        x: area.x,
        y: area.y + area.height.saturating_sub(3),
        width: area.width,
        height: 3,
    };
    
    let instructions = vec![
        Line::from(Span::styled("Instructions:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
        Line::from(Span::styled("↑/↓: Navigate branches", Style::default().fg(Color::Gray))),
        Line::from(Span::styled("Enter: Select branch and continue", Style::default().fg(Color::Gray))),
    ];
    
    let instruction_para = Paragraph::new(instructions)
        .alignment(ratatui::layout::Alignment::Left);
    f.render_widget(instruction_para, instruction_area);
}

pub fn draw_user_info_panel(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title("👤 User Information Setup")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Split area for form fields
    let chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .margin(1)
        .constraints([
            ratatui::layout::Constraint::Length(3), // Name
            ratatui::layout::Constraint::Length(3), // Employee Number
            ratatui::layout::Constraint::Length(3), // Designation
            ratatui::layout::Constraint::Length(4), // Instructions
            ratatui::layout::Constraint::Min(1),    // Error messages
        ])
        .split(inner);

    // Draw name field
    let name_style = if matches!(app.current_user_field, UserInfoField::Name) {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let name_value = if matches!(app.current_user_field, UserInfoField::Name) {
        &app.user_info_buffer
    } else {
        &app.user_info.name
    };
    let name_block = Block::default()
        .title("Name")
        .borders(Borders::ALL)
        .border_style(name_style);
    let name_para = Paragraph::new(name_value.as_str())
        .block(name_block)
        .wrap(Wrap { trim: true });
    f.render_widget(name_para, chunks[0]);

    // Draw employee number field
    let emp_style = if matches!(app.current_user_field, UserInfoField::EmployeeNumber) {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let emp_value = if matches!(app.current_user_field, UserInfoField::EmployeeNumber) {
        &app.user_info_buffer
    } else {
        &app.user_info.employee_number
    };
    let emp_block = Block::default()
        .title("Employee Number")
        .borders(Borders::ALL)
        .border_style(emp_style);
    let emp_para = Paragraph::new(emp_value.as_str())
        .block(emp_block)
        .wrap(Wrap { trim: true });
    f.render_widget(emp_para, chunks[1]);

    // Draw designation field
    let des_style = if matches!(app.current_user_field, UserInfoField::Designation) {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let des_value = if matches!(app.current_user_field, UserInfoField::Designation) {
        &app.user_info_buffer
    } else {
        &app.user_info.designation
    };
    let des_block = Block::default()
        .title("Designation")
        .borders(Borders::ALL)
        .border_style(des_style);
    let des_para = Paragraph::new(des_value.as_str())
        .block(des_block)
        .wrap(Wrap { trim: true });
    f.render_widget(des_para, chunks[2]);

    // Draw instructions
    let instructions = vec![
        Line::from(Span::styled("Instructions:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
        Line::from(Span::styled("Tab/Shift+Tab: Navigate fields", Style::default().fg(Color::Gray))),
        Line::from(Span::styled("Type: Edit current field", Style::default().fg(Color::Gray))),
        Line::from(Span::styled("Ctrl+S: Save user info", Style::default().fg(Color::Gray))),
        Line::from(Span::styled("Esc: Back to main", Style::default().fg(Color::Gray))),
    ];
    let instruction_para = Paragraph::new(instructions);
    f.render_widget(instruction_para, chunks[3]);

    // Draw errors if any
    let errors = app.get_user_info_errors();
    if !errors.is_empty() {
        let error_lines: Vec<Line> = errors.iter()
            .map(|err| Line::from(Span::styled(format!("⚠ {}", err), Style::default().fg(Color::Red))))
            .collect();
        let error_para = Paragraph::new(error_lines);
        f.render_widget(error_para, chunks[4]);
    }

    // Set cursor for active field
    if matches!(app.current_user_field, UserInfoField::Name) {
        let cursor_x = chunks[0].x + 1 + app.user_info_buffer.len() as u16;
        let cursor_y = chunks[0].y + 1;
        f.set_cursor_position((cursor_x, cursor_y));
    } else if matches!(app.current_user_field, UserInfoField::EmployeeNumber) {
        let cursor_x = chunks[1].x + 1 + app.user_info_buffer.len() as u16;
        let cursor_y = chunks[1].y + 1;
        f.set_cursor_position((cursor_x, cursor_y));
    } else if matches!(app.current_user_field, UserInfoField::Designation) {
        let cursor_x = chunks[2].x + 1 + app.user_info_buffer.len() as u16;
        let cursor_y = chunks[2].y + 1;
        f.set_cursor_position((cursor_x, cursor_y));
    }
}

pub fn draw_user_info_prompt(f: &mut Frame, area: Rect) {
    // Draw a centered popup
    let popup_area = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Percentage(30),
            ratatui::layout::Constraint::Length(8),
            ratatui::layout::Constraint::Percentage(30),
        ])
        .split(area)[1];
    
    let popup_area = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            ratatui::layout::Constraint::Percentage(20),
            ratatui::layout::Constraint::Length(60),
            ratatui::layout::Constraint::Percentage(20),
        ])
        .split(popup_area)[1];

    // Clear the area behind the popup
    f.render_widget(Clear, popup_area);

    let block = Block::default()
        .title("⚠ User Information Required")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::default().fg(Color::Yellow));

    let text = vec![
        Line::from(Span::styled("User information is not configured!", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from(Span::raw("")),
        Line::from(Span::styled("Please set up your user information before using the application.", Style::default().fg(Color::White))),
        Line::from(Span::raw("")),
        Line::from(Span::styled("Press 'Ctrl+E' to configure your user information", Style::default().fg(Color::Green))),
        Line::from(Span::styled("Press any other key to continue without setup", Style::default().fg(Color::Gray))),
    ];

    let para = Paragraph::new(text)
        .block(block)
        .alignment(ratatui::layout::Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(para, popup_area);
}
