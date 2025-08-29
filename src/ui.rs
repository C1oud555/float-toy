use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
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
                Constraint::Length(3), // Bit display
                Constraint::Length(3), // Output
                Constraint::Min(0),    // Spacer
            ]
            .as_ref(),
        )
        .split(frame.area());

    let input_block = Block::default().title("Hex Input").borders(Borders::ALL);
    let input = Paragraph::new(app.hex_input.as_str())
        .block(input_block)
        .style(Style::default().fg(Color::White));
    frame.render_widget(input, chunks[0]);

    let mut bit_spans: Vec<Span> = Vec::with_capacity(32);
    for (i, &bit) in app.bits.iter().enumerate() {
        let color = if i == 0 {
            Color::Red // Sign bit
        } else if i < 9 {
            Color::Yellow // Exponent
        } else {
            Color::Blue // Mantissa
        };
        bit_spans.push(Span::styled(
            bit.to_string(),
            Style::default().fg(color),
        ));
    }
    let bits_display = Paragraph::new(Line::from(bit_spans))
        .block(Block::default().title("Bits").borders(Borders::ALL));
    frame.render_widget(bits_display, chunks[1]);


    let output_block = Block::default().title("Float Output").borders(Borders::ALL);
    let output = Paragraph::new(app.float_output.as_str())
        .block(output_block)
        .style(Style::default().fg(Color::Green));
    frame.render_widget(output, chunks[2]);
}