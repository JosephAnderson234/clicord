use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(f.area());

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .map(|m| {
            ListItem::new(Line::from(vec![
                Span::styled(format!("{}: ", m.from), Style::default().fg(Color::Cyan)),
                Span::raw(m.text.clone()),
            ]))
        })
        .collect();

    let messages_list =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("clicord"));
    f.render_widget(messages_list, chunks[0]);

    let input = Paragraph::new(app.input.as_str()).block(
        Block::default()
            .borders(Borders::ALL)
            .title(app.username.as_str()),
    );
    f.render_widget(input, chunks[1]);

    let help = Paragraph::new("Enter: enviar   Esc / Ctrl+C: salir")
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(help, chunks[2]);
}
