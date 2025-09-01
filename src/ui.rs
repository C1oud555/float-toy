use ratatui::{
    prelude::{Constraint, Direction, Frame, Layout, Line, Span, Style, Stylize},
    widgets::{Block, Borders, Paragraph, Tabs},
};

use crate::app::{App, FloatFormat, InputField};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Inputs
            Constraint::Length(3), // Format buttons
            Constraint::Length(3), // Bits display
            Constraint::Length(3), // Output
            Constraint::Min(0),    // Spacer
        ])
        .split(frame.area());

    // Inputs
    let input_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .split(main_layout[0]);

    let hex_input_block = Block::default().title("Hex Input").borders(Borders::ALL);
    let hex_input_p = Paragraph::new(app.hex_input.as_str()).block(hex_input_block);

    let exponent_input_block = Block::default().title("Exponent").borders(Borders::ALL);
    let exponent_input_p = Paragraph::new(app.exponent_bits_input.as_str()).block(exponent_input_block);

    let mantissa_input_block = Block::default().title("Mantissa").borders(Borders::ALL);
    let mantissa_input_p = Paragraph::new(app.mantissa_bits_input.as_str()).block(mantissa_input_block);

    match app.active_input {
        InputField::Hex => frame.render_widget(hex_input_p.style(Style::default().fg(ratatui::style::Color::Cyan)), input_layout[0]),
        _ => frame.render_widget(hex_input_p, input_layout[0]),
    }
    match app.active_input {
        InputField::Exponent => frame.render_widget(exponent_input_p.style(Style::default().fg(ratatui::style::Color::Cyan)), input_layout[1]),
        _ => frame.render_widget(exponent_input_p, input_layout[1]),
    }
    match app.active_input {
        InputField::Mantissa => frame.render_widget(mantissa_input_p.style(Style::default().fg(ratatui::style::Color::Cyan)), input_layout[2]),
        _ => frame.render_widget(mantissa_input_p, input_layout[2]),
    }


    // Format buttons
    let titles: Vec<&str> = FloatFormat::all().iter().map(|f| f.name()).collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().title("Formats").borders(Borders::ALL))
        .style(Style::default().white())
        .highlight_style(Style::default().bold().yellow());

    let current_format_index = FloatFormat::all()
        .iter()
        .position(|&f| f == app.current_format)
        .unwrap_or(0);
    frame.render_widget(tabs.select(current_format_index), main_layout[1]);

    // Bits display
    let mut bit_spans: Vec<Span> = Vec::new();
    let total_bits = 1 + app.exponent_bits + app.mantissa_bits;
    for (i, &bit) in app.bits.iter().enumerate().take(total_bits as usize) {
        let color = if i == 0 {
            ratatui::style::Color::Red // Sign bit
        } else if i < 1 + app.exponent_bits as usize {
            ratatui::style::Color::Yellow // Exponent
        } else {
            ratatui::style::Color::Blue // Mantissa
        };
        bit_spans.push(Span::styled(
            bit.to_string(),
            Style::default().fg(color),
        ));
    }
    let bits_display = Paragraph::new(Line::from(bit_spans))
        .block(Block::default().title("Bits").borders(Borders::ALL));
    frame.render_widget(bits_display, main_layout[2]);

    // Output
    let output_block = Block::default().title("Float Output").borders(Borders::ALL);
    let output = Paragraph::new(app.float_output.as_str())
        .block(output_block)
        .style(Style::default().fg(ratatui::style::Color::Green));
    frame.render_widget(output, main_layout[3]);
}
