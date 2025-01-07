use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io::{self};

//Screen that asks User for Backend URL
pub fn ask_for_url() -> String {
    // Setup terminal
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut input = String::new(); // Stores user input

    loop {
        terminal
            .draw(|frame| {
                // Layout: Split the screen vertically
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Percentage(80), // Main message area
                            Constraint::Percentage(20), // Input area
                        ]
                        .as_ref(),
                    )
                    .split(frame.area());

                // Message to the user
                let text = Paragraph::new("Please enter the Backend URL and press Enter:")
                    .block(Block::default().borders(Borders::ALL).title("Input"));

                // Display the user input
                let input_box = Paragraph::new(input.as_str())
                    .block(Block::default().borders(Borders::ALL).title("Your Input"));

                // Render both areas
                frame.render_widget(text, chunks[0]);
                frame.render_widget(input_box, chunks[1]);
            })
            .unwrap();

        // Handle user input
        if let Event::Key(key_event) = event::read().unwrap() {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Char(c) => {
                        // Append character to input
                        input.push(c);
                    }
                    KeyCode::Backspace => {
                        // Remove last character from input
                        input.pop();
                    }
                    KeyCode::Enter => {
                        // Return the input when Enter is pressed
                        disable_raw_mode().unwrap();
                        execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture).unwrap();
                        if input.ends_with('/') {
                            input = input.trim_end_matches('/').to_string();
                        }
                        return input;
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn error_url_unavailable() {
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
                    Paragraph::new("Invalid URL. Please press Enter and insert a valid URL.")
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

pub fn error_url_unreachable(backend_url: &Option<String>) {
    // Setup terminal for error screen
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    // Get the backend URL or use a placeholder if None
    let url_display = backend_url.as_deref().unwrap_or("Unknown");

    loop {
        terminal
            .draw(|frame| {
                let size = frame.area();
                let block = Block::default().borders(Borders::ALL).title("Error");

                let message = format!(
                    "Backend on {} not reachable. Please press Enter and insert a valid URL.",
                    url_display
                );
                let paragraph = Paragraph::new(message).block(block);

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
