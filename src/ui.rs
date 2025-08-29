use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3), // Input
                Constraint::Length(3), // Output
                Constraint::Min(0),    // Spacer
            ]
            .as_ref(),
        )
        .split(frame.area());

    let input_block = Block::default().title("Binary Input").borders(Borders::ALL);
    let input = Paragraph::new(app.binary_input.as_str())
        .block(input_block)
        .style(Style::default().fg(Color::White));
    frame.render_widget(input, chunks[0]);

    let output_block = Block::default().title("Float Output").borders(Borders::ALL);
    let output = Paragraph::new(app.float_output.as_str())
        .block(output_block)
        .style(Style::default().fg(Color::Green));
    frame.render_widget(output, chunks[1]);
}
