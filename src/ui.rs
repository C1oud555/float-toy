use ratatui::{
    prelude::{Alignment, Constraint, Direction, Frame, Layout, Line, Span, Style, Stylize},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, FloatFormat, InputField};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Hex input
            Constraint::Length(3), // Float input
            Constraint::Length(3), // Format buttons
            Constraint::Length(3), // Bits display
            Constraint::Length(3), // Custom format inputs
            Constraint::Min(0),    // Spacer
        ])
        .split(frame.area());

    // Hex input
    let hex_input_block = Block::default().title("Hex Input").borders(Borders::ALL);
    let hex_text = format!("0x{}", app.hex_input);
    let hex_input_p = Paragraph::new(hex_text).block(hex_input_block);
    if app.active_input == InputField::Hex {
        frame.render_widget(hex_input_p.style(Style::default().fg(ratatui::style::Color::Cyan)), main_layout[0]);
    } else {
        frame.render_widget(hex_input_p, main_layout[0]);
    }

    // Float input
    let float_input_block = Block::default().title("Float Input").borders(Borders::ALL);
    let float_input_p = Paragraph::new(app.float_input.as_str()).block(float_input_block);
    if app.active_input == InputField::Float {
        frame.render_widget(float_input_p.style(Style::default().fg(ratatui::style::Color::Cyan)), main_layout[1]);
    } else {
        frame.render_widget(float_input_p, main_layout[1]);
    }

    // Format buttons
    let formats = FloatFormat::all();
    let num_formats = formats.len();
    let button_constraints = std::iter::repeat(Constraint::Percentage(100 / num_formats as u16))
        .take(num_formats)
        .collect::<Vec<_>>();

    let buttons_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(button_constraints)
        .split(main_layout[2]);

    app.format_button_areas = buttons_layout.to_vec();

    for (i, &format) in formats.iter().enumerate() {
        let button_text = format.name();
        let mut button_style = Style::default();
        if app.current_format == format {
            button_style = Style::default().bold().yellow();
        }
        let button = Paragraph::new(button_text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .style(button_style);
        frame.render_widget(button, buttons_layout[i]);
    }

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
    frame.render_widget(bits_display, main_layout[3]);

    // Custom format inputs
    let custom_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[4]);

    let exponent_input_block = Block::default().title("Exponent").borders(Borders::ALL);
    let exponent_input_p = Paragraph::new(app.exponent_bits_input.as_str()).block(exponent_input_block);

    let mantissa_input_block = Block::default().title("Mantissa").borders(Borders::ALL);
    let mantissa_input_p = Paragraph::new(app.mantissa_bits_input.as_str()).block(mantissa_input_block);

    if app.active_input == InputField::Exponent {
        frame.render_widget(exponent_input_p.style(Style::default().fg(ratatui::style::Color::Cyan)), custom_layout[0]);
    } else {
        frame.render_widget(exponent_input_p, custom_layout[0]);
    }
    if app.active_input == InputField::Mantissa {
        frame.render_widget(mantissa_input_p.style(Style::default().fg(ratatui::style::Color::Cyan)), custom_layout[1]);
    } else {
        frame.render_widget(mantissa_input_p, custom_layout[1]);
    }
}