use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    style::{Color, Modifier, Style},
    Terminal,
};
use std::io::{self, stdout};
use super::logik;

pub fn draw_register_screen() -> (String, String) {
    // Terminal initialisieren
    enable_raw_mode().unwrap();
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.clear().unwrap();

    let mut email = String::new();
    let mut password = String::new();
    let mut is_password_field = false;
    let mut error_message = String::new();

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
                            Constraint::Percentage(10),
                            Constraint::Percentage(20),
                        ]
                        .as_ref(),
                    )
                    .split(f.area());

                // Titel
                let title = Paragraph::new("Register to PassHub")
                    .style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    )
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL).title("Register"));

                // E-Mail-Eingabe
                let email_paragraph = Paragraph::new(format!("E-Mail: {}", email))
                    .style(Style::default().fg(Color::White))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(if !is_password_field { "E-Mail" } else { " " }),
                    );

                // Passwort-Eingabe
                let password_masked: String = "*".repeat(password.len());
                let password_paragraph = Paragraph::new(format!("Password: {}", password_masked))
                    .style(Style::default().fg(Color::White))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(if is_password_field { "Password" } else { " " }),
                    );

                // Fehlermeldung
                let error_paragraph = Paragraph::new(error_message.clone())
                    .style(Style::default().fg(Color::Red))
                    .alignment(Alignment::Center);

                f.render_widget(title, chunks[0]);
                f.render_widget(email_paragraph, chunks[1]);
                f.render_widget(password_paragraph, chunks[2]);
                f.render_widget(error_paragraph, chunks[3]);
            })
            .unwrap();

        // Eingaben lesen
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Enter => {
                        if is_password_field {
                            if logik::validate_password(&password) {
                                break;
                            } else {
                                error_message = String::from(
                                    "Invalid password: Must be at least 10 characters long and include uppercase letters, lowercase letters, numbers, and special characters.",
                                );
                                password.clear(); // Passwort zurücksetzen
                            }
                        } else {
                            is_password_field = true;
                        }
                    }
                    KeyCode::Backspace => {
                        if is_password_field {
                            password.pop();
                        } else {
                            email.pop();
                        }
                    }
                    KeyCode::Char(c) => {
                        if is_password_field {
                            password.push(c);
                        } else {
                            email.push(c);
                        }
                    }
                    KeyCode::Up => {
                        is_password_field = false; // Fokus auf E-Mail-Feld
                    }
                    KeyCode::Down => {
                        is_password_field = true; // Fokus auf Passwort-Feld
                    }
                    _ => {}
                }
            }
        }
    }

    // Terminal bereinigen
    terminal.clear().unwrap();
    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen).unwrap();

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

pub fn error_network() {
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

            let paragraph = Paragraph::new("Network Error. \n Press Enter to try again.")
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

pub fn error_bad_request() {
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

            let paragraph = Paragraph::new("Invalid Payload. \n Press Enter to try again.")
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

pub fn error_unknown() {
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

            let paragraph = Paragraph::new("Unknown Error. \n Press Enter to try again.")
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

pub fn error_user_exists() {
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

            let paragraph = Paragraph::new("User already exists. \nPress Enter to try again.\nTo login press any key except 'r' on the welcome screen. Press 'ESC' to exit.")
                .block(block);

            frame.render_widget(paragraph, size);
        }).unwrap();

        // Wait for user input to dismiss the error screen
        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.kind {
                KeyEventKind::Press => match key_event.code {
                    KeyCode::Enter => break, // Verlasse den Fehlerbildschirm
                    KeyCode::Esc => {
                        std::process::exit(0); // Schließe das Programm
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    // Restore terminal
    disable_raw_mode().unwrap();
    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
}