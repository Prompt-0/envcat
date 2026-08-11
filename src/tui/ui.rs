use crate::tui::app::App;
use crate::types::ValueType;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Main content (List + Details)
            Constraint::Length(3), // Search bar / Status
        ])
        .split(f.size());

    draw_header(f, app, chunks[0]);
    draw_main_body(f, app, chunks[1]);
    draw_footer(f, app, chunks[2]);
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let total = app.all_entries.len();
    let secret_cnt = app.all_entries.iter().filter(|e| e.is_secret).count();
    let mask_status = if app.global_reveal {
        Span::styled(" [REVEALED] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
    } else {
        Span::styled(" [MASKED] ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
    };

    let title_line = Line::from(vec![
        Span::styled(" ENVCAT TUI ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw("│ Total: "),
        Span::styled(format!("{}", total), Style::default().fg(Color::Green)),
        Span::raw(" │ Secrets: "),
        Span::styled(format!("{}", secret_cnt), Style::default().fg(Color::Red)),
        Span::raw(" │ Mode: "),
        mask_status,
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title_line)
        .style(Style::default().fg(Color::White));
    f.render_widget(block, area);
}

fn draw_main_body(f: &mut Frame, app: &mut App, area: Rect) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(area);

    // Left List
    let items: Vec<ListItem> = app
        .filtered_indices
        .iter()
        .enumerate()
        .map(|(idx, &entry_idx)| {
            let entry = &app.all_entries[entry_idx];
            let is_sel = idx == app.selected_index;

            let icon = if entry.is_secret { "🔒 " } else { "   " };
            let type_str = match entry.value_type {
                ValueType::Secret => "[secret]",
                ValueType::Url => "[url]",
                ValueType::Json => "[json]",
                ValueType::Boolean => "[bool]",
                ValueType::Number => "[num]",
                ValueType::String => "[str]",
                ValueType::Empty => "[empty]",
            };

            let style = if is_sel {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else if entry.is_secret {
                Style::default().fg(Color::LightRed)
            } else {
                Style::default().fg(Color::White)
            };

            let line = Line::from(vec![
                Span::raw(icon),
                Span::styled(format!("{:<25}", entry.key), style),
                Span::styled(format!(" {:<8}", type_str), Style::default().fg(Color::DarkGray)),
            ]);

            ListItem::new(line)
        })
        .collect();

    let list_block = Block::default()
        .borders(Borders::ALL)
        .title(" Variables ")
        .style(Style::default().fg(Color::Cyan));

    let mut list_state = ListState::default();
    if !app.filtered_indices.is_empty() {
        list_state.select(Some(app.selected_index));
    }

    let list_widget = List::new(items)
        .block(list_block)
        .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD));
    f.render_stateful_widget(list_widget, main_chunks[0], &mut list_state);

    // Right Detail View
    let detail_block = Block::default()
        .borders(Borders::ALL)
        .title(" Variable Details ")
        .style(Style::default().fg(Color::Green));

    if let Some(entry) = app.selected_entry() {
        let val_display = if app.global_reveal {
            &entry.raw_value
        } else {
            &entry.masked_value
        };

        let entropy = app.get_entropy_score();

        let detail_text = vec![
            Line::from(vec![
                Span::styled("Key:         ", Style::default().fg(Color::DarkGray)),
                Span::styled(&entry.key, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("Value:       ", Style::default().fg(Color::DarkGray)),
                Span::styled(val_display, if entry.is_secret && !app.global_reveal { Style::default().fg(Color::Red) } else { Style::default().fg(Color::White) }),
            ]),
            Line::from(vec![
                Span::styled("Raw Value:   ", Style::default().fg(Color::DarkGray)),
                Span::styled(&entry.raw_value, Style::default().fg(Color::Gray)),
            ]),
            Line::from(vec![
                Span::styled("Type:        ", Style::default().fg(Color::DarkGray)),
                Span::styled(entry.value_type.to_string(), Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("Is Secret:   ", Style::default().fg(Color::DarkGray)),
                Span::styled(if entry.is_secret { "YES (🔒)" } else { "NO" }, if entry.is_secret { Style::default().fg(Color::Red) } else { Style::default().fg(Color::Green) }),
            ]),
            Line::from(vec![
                Span::styled("Entropy:     ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{:.2} bits/char", entropy), Style::default().fg(Color::Magenta)),
            ]),
            Line::from(vec![
                Span::styled("Line Number: ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{}", entry.line_number), Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::styled("Section:     ", Style::default().fg(Color::DarkGray)),
                Span::styled(entry.section.as_deref().unwrap_or("General"), Style::default().fg(Color::Blue)),
            ]),
            Line::from(vec![
                Span::styled("Comment:     ", Style::default().fg(Color::DarkGray)),
                Span::styled(entry.comment.as_deref().unwrap_or("None"), Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)),
            ]),
        ];

        let p = Paragraph::new(detail_text)
            .block(detail_block)
            .wrap(Wrap { trim: false });
        f.render_widget(p, main_chunks[1]);
    } else {
        let empty_p = Paragraph::new("No entries match search query.")
            .block(detail_block)
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(empty_p, main_chunks[1]);
    }
}

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default().borders(Borders::ALL);

    if app.is_searching {
        let input_text = Paragraph::new(format!("Search: {}_", app.search_query))
            .block(block.title(" Filter Mode (Press Enter/Esc to exit search) "))
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(input_text, area);
    } else {
        let msg = app.status_message.as_deref().unwrap_or("Controls: ↑/↓ Move | / Filter | Space Mask Toggle | r Refresh | q Quit");
        let status_p = Paragraph::new(msg)
            .block(block.title(" Controls & Navigation "))
            .style(Style::default().fg(Color::Cyan));
        f.render_widget(status_p, area);
    }
}
