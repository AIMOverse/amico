//! TUI rendering with ratatui.

use crate::app::{App, Focus};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    // Overall layout: sidebar (sessions) | main (chat)
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(75),
        ])
        .split(f.area());

    draw_sessions_panel(f, app, chunks[0]);
    draw_chat_panel(f, app, chunks[1]);
}

fn draw_sessions_panel(f: &mut Frame, app: &App, area: Rect) {
    let border_style = if app.focus == Focus::Sessions {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let items: Vec<ListItem> = app
        .sessions
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let label = format!(
                " {} ({} msgs)",
                &s.id[..8.min(s.id.len())],
                s.message_count
            );
            let style = if Some(&s.id) == app.current_session_id.as_ref() {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else if i == app.selected_session {
                Style::default().fg(Color::White)
            } else {
                Style::default().fg(Color::Gray)
            };
            ListItem::new(label).style(style)
        })
        .collect();

    let sessions_block = Block::default()
        .title(" Sessions ")
        .borders(Borders::ALL)
        .border_style(border_style);

    let list = List::new(items).block(sessions_block);
    f.render_widget(list, area);
}

fn draw_chat_panel(f: &mut Frame, app: &App, area: Rect) {
    let border_style = if app.focus == Focus::Chat {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    // Split chat area: messages | input | status
    let chat_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(area);

    // -- Messages area --
    let mut lines: Vec<Line> = Vec::new();

    for msg in &app.messages {
        let (prefix, color) = match msg.role.as_str() {
            "user" => ("You: ", Color::Green),
            "assistant" => ("Agent: ", Color::Blue),
            "system" => ("System: ", Color::DarkGray),
            _ => ("", Color::White),
        };
        lines.push(Line::from(vec![
            Span::styled(prefix, Style::default().fg(color).add_modifier(Modifier::BOLD)),
            Span::styled(&msg.content, Style::default().fg(Color::White)),
        ]));
        lines.push(Line::from(""));
    }

    // Show streaming text
    if app.is_streaming && !app.streaming_text.is_empty() {
        lines.push(Line::from(vec![
            Span::styled(
                "Agent: ",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(&app.streaming_text, Style::default().fg(Color::White)),
            Span::styled("▊", Style::default().fg(Color::Cyan)),
        ]));
    } else if app.is_streaming {
        lines.push(Line::from(Span::styled(
            "Agent is thinking...",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let messages_block = Block::default()
        .title(" Chat ")
        .borders(Borders::ALL)
        .border_style(border_style);

    let messages_widget = Paragraph::new(Text::from(lines))
        .block(messages_block)
        .wrap(Wrap { trim: false });
    f.render_widget(messages_widget, chat_chunks[0]);

    // -- Input area --
    let input_block = Block::default()
        .title(" Input (Enter to send) ")
        .borders(Borders::ALL)
        .border_style(if app.focus == Focus::Chat {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::DarkGray)
        });

    let input_widget = Paragraph::new(app.input.as_str()).block(input_block);
    f.render_widget(input_widget, chat_chunks[1]);

    // -- Status bar --
    let status = Paragraph::new(Span::styled(
        format!(" {} ", app.status),
        Style::default().fg(Color::DarkGray),
    ));
    f.render_widget(status, chat_chunks[2]);
}
