use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use std::io::{self, stdout};

pub fn draw_login_screen(stored_email: String) -> (String, String) {
    // Enable raw mode
    enable_raw_mode().unwrap();

    // Initialize the terminal
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.clear().unwrap();

    // Variables to store user input
    let mut email = stored_email.clone();
    let mut password = String::new();
    let mut is_password_field = !stored_email.is_empty();

    loop {
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints(
                        [
                            Constraint::Percentage(30),
                            Constraint::Percentage(20),
                            Constraint::Percentage(20),
                            Constraint::Percentage(30),
                        ]
                        .as_ref(),
                    )
                    .split(f.area());

                // Title
                let title = Paragraph::new("PassHub Login")
                    .style(
                        Style::default()
                            .fg(Color::Rgb(255, 163, 26))
                            .add_modifier(Modifier::BOLD),
                    )
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL).title("Login"));

                // Email Input
                let email_paragraph = Paragraph::new(format!("E-Mail: {}", email))
                    .style(Style::default().fg(Color::White))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(if !is_password_field { "E-Mail" } else { " " }),
                    );

                // Password Input
                let password_masked: String = "*".repeat(password.len());
                let password_paragraph = Paragraph::new(format!("Password: {}", password_masked))
                    .style(Style::default().fg(Color::White))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(if is_password_field { "Password" } else { " " }),
                    );

                // Render Widgets
                f.render_widget(title, chunks[0]);
                f.render_widget(email_paragraph, chunks[1]);
                f.render_widget(password_paragraph, chunks[2]);
            })
            .unwrap();

        // Handle user input
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Enter => {
                        if is_password_field {
                            break; // Beende die Schleife bei "Enter" im Passwortfeld
                        } else {
                            is_password_field = true; // Wechsel zum Passwortfeld
                        }
                    }
                    KeyCode::Backspace => {
                        if is_password_field {
                            password.pop(); // Entfernt das letzte Zeichen im Passwort
                        } else {
                            email.pop(); // Entfernt das letzte Zeichen in der E-Mail
                        }
                    }
                    KeyCode::Char(c) => {
                        if is_password_field {
                            password.push(c); // Fügt ein Zeichen zum Passwort hinzu
                        } else {
                            email.push(c); // Fügt ein Zeichen zur E-Mail hinzu
                        }
                    }
                    KeyCode::Up => {
                        is_password_field = false; // Wechsel zum E-Mail-Feld
                    }
                    KeyCode::Down => {
                        is_password_field = true; // Wechsel zum Passwort-Feld
                    }
                    _ => {}
                }
            }
        }
    }

    // Clear and restore terminal
    terminal.clear().unwrap();
    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    )
    .unwrap();

    (email, password)
}

pub fn error_argon2_fail() {
    // Setup terminal for error screen
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        terminal.draw(|frame| {
            let size = frame.area();
            let block = Block::default()
                .borders(Borders::ALL)
                .title("Error");

            let paragraph = Paragraph::new("FATAL ERROR. Argon 2 password hashing failed. Please press Enter to exit programm.")
                .block(block);

            frame.render_widget(paragraph, size);
        }).unwrap();

        // Wait for user input to dismiss the error screen
        if let Event::Key(key_event) = event::read().unwrap() {
            if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Enter {
                break;
            }
        }
    }

    // Restore terminal
    disable_raw_mode().unwrap();
    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
}

pub fn error_unauthorized() {
    // Setup terminal for error screen
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        terminal
            .draw(|frame| {
                let size = frame.area();
                let block = Block::default().borders(Borders::ALL).title("Error");

                let paragraph = Paragraph::new(
                    "Login failed. Please check your credentials. \nPress Enter to try again.",
                )
                .block(block);

                frame.render_widget(paragraph, size);
            })
            .unwrap();

        // Wait for user input to dismiss the error screen
        if let Event::Key(key_event) = event::read().unwrap() {
            if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Enter {
                break;
            }
        }
    }

    // Restore terminal
    disable_raw_mode().unwrap();
    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
}

pub fn error_network() {
    // Setup terminal for error screen
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        terminal
            .draw(|frame| {
                let size = frame.area();
                let block = Block::default().borders(Borders::ALL).title("Error");

                let paragraph =
                    Paragraph::new("Network Error. \nPress Enter to try again.").block(block);

                frame.render_widget(paragraph, size);
            })
            .unwrap();

        // Wait for user input to dismiss the error screen
        if let Event::Key(key_event) = event::read().unwrap() {
            if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Enter {
                break;
            }
        }
    }

    // Restore terminal
    disable_raw_mode().unwrap();
    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
}

pub fn error_user_not_found() {
    // Setup terminal for error screen
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        terminal.draw(|frame| {
            let size = frame.area();
            let block = Block::default()
                .borders(Borders::ALL)
                .title("Error");

            let paragraph = Paragraph::new("User not found. \nPress Enter to try again.\nTo register, press 'r' on the welcome screen. Press 'ESC' to exit.")
                .block(block);

            frame.render_widget(paragraph, size);
        }).unwrap();

        // Wait for user input to dismiss the error screen
        if let Event::Key(key_event) = event::read().unwrap() {
                if key_event.kind == KeyEventKind::Press { match key_event.code {
                    KeyCode::Enter => break,
                    KeyCode::Esc => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode().unwrap();
    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
}

pub fn error_bad_request() {
    // Setup terminal for error screen
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        terminal
            .draw(|frame| {
                let size = frame.area();
                let block = Block::default().borders(Borders::ALL).title("Error");

                let paragraph =
                    Paragraph::new("Invalid Payload. \nPress Enter to try again.").block(block);

                frame.render_widget(paragraph, size);
            })
            .unwrap();

        // Wait for user input to dismiss the error screen
        if let Event::Key(key_event) = event::read().unwrap() {
            if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Enter {
                break;
            }
        }
    }

    // Restore terminal
    disable_raw_mode().unwrap();
    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
}

pub fn error_unknown() {
    // Setup terminal for error screen
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        terminal
            .draw(|frame| {
                let size = frame.area();
                let block = Block::default().borders(Borders::ALL).title("Error");

                let paragraph = Paragraph::new("Unknown. \nPress Enter to try again.").block(block);

                frame.render_widget(paragraph, size);
            })
            .unwrap();

        // Wait for user input to dismiss the error screen
        if let Event::Key(key_event) = event::read().unwrap() {
            if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Enter {
                break;
            }
        }
    }

    // Restore terminal
    disable_raw_mode().unwrap();
    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
}
