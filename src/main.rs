use std::{io, time::Duration};

use app::{App, FloatFormat, InputField};
use crossterm::event::{
    self, EnableMouseCapture, Event, KeyCode, KeyModifiers, MouseEventKind, DisableMouseCapture,
};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::Position,
    Terminal,
};
use ui::ui;

mod app;
mod ui;

fn main() -> io::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    run_app(&mut terminal, &mut app)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    match (key.code, key.modifiers) {
                        (KeyCode::Char('a'), KeyModifiers::CONTROL) => match app.active_input {
                            InputField::Hex => app.hex_input.clear(),
                            InputField::Float => app.float_input.clear(),
                            InputField::Exponent => app.exponent_bits_input.clear(),
                            InputField::Mantissa => app.mantissa_bits_input.clear(),
                        },
                        (KeyCode::Esc, _) => return Ok(()),
                        (KeyCode::Tab, _) => {
                            app.active_input = match app.active_input {
                                InputField::Hex => InputField::Float,
                                InputField::Float => InputField::Exponent,
                                InputField::Exponent => InputField::Mantissa,
                                InputField::Mantissa => InputField::Hex,
                            };
                        }
                        (KeyCode::Char(c), _) => match app.active_input {
                            InputField::Hex => {
                                app.hex_input.push(c);
                                app.convert_hex_to_float();
                            }
                            InputField::Float => {
                                app.float_input.push(c);
                                app.convert_float_to_hex();
                            }
                            InputField::Exponent => {
                                app.exponent_bits_input.push(c);
                                app.update_custom_format();
                            }
                            InputField::Mantissa => {
                                app.mantissa_bits_input.push(c);
                                app.update_custom_format();
                            }
                        },
                        (KeyCode::Backspace, _) => match app.active_input {
                            InputField::Hex => {
                                app.hex_input.pop();
                                app.convert_hex_to_float();
                            }
                            InputField::Float => {
                                app.float_input.pop();
                                app.convert_float_to_hex();
                            }
                            InputField::Exponent => {
                                app.exponent_bits_input.pop();
                                app.update_custom_format();
                            }
                            InputField::Mantissa => {
                                app.mantissa_bits_input.pop();
                                app.update_custom_format();
                            }
                        },
                        _ => {}
                    }
                }
                Event::Mouse(mouse) => {
                    if matches!(mouse.kind, MouseEventKind::Down(_)) {
                        let (x, y) = (mouse.column, mouse.row);
                        for (i, &area) in app.format_button_areas.iter().enumerate() {
                            if area.contains(Position { x, y }) {
                                let formats = FloatFormat::all();
                                app.set_format(formats[i]);
                                break;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
