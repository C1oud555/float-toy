use std::{io, time::Duration};

use app::{App, FloatFormat, InputField};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use ui::ui;

mod app;
mod ui;

fn main() -> io::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    run_app(&mut terminal, &mut app)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Tab => {
                        app.active_input = match app.active_input {
                            InputField::Hex => InputField::Exponent,
                            InputField::Exponent => InputField::Mantissa,
                            InputField::Mantissa => InputField::Hex,
                        };
                    }
                    KeyCode::Right => {
                        let formats = FloatFormat::all();
                        let current_index = formats.iter().position(|&f| f == app.current_format).unwrap_or(0);
                        let next_index = (current_index + 1) % formats.len();
                        app.set_format(formats[next_index]);
                    }
                    KeyCode::Left => {
                        let formats = FloatFormat::all();
                        let current_index = formats.iter().position(|&f| f == app.current_format).unwrap_or(0);
                        let next_index = (current_index + formats.len() - 1) % formats.len();
                        app.set_format(formats[next_index]);
                    }
                    KeyCode::Char(c) => match app.active_input {
                        InputField::Hex => {
                            app.hex_input.push(c);
                            app.convert();
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
                    KeyCode::Backspace => match app.active_input {
                        InputField::Hex => {
                            app.hex_input.pop();
                            app.convert();
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
        }
    }
}