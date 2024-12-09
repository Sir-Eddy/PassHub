use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    backend::{CrosstermBackend},
    layout::{Alignment, Layout, Constraint, Direction},
    style::{Color, Style},
    widgets::{Paragraph},
    Terminal,
};
use std::io::{self, stdout};

pub fn draw_welcome_screen() {
    // Enable raw mode
    enable_raw_mode().unwrap();

    // Initialize the terminal
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.clear().unwrap();

    terminal.draw(|f| {
        // Split the screen layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ]
                .as_ref(),
            )
            .split(f.size());

        // ASCII Art (adjust alignment and ensure proper formatting)
        let ascii_art = r#"
 ____              _   _       _      
|  _ \            | | | |     | |     
| |_) |_ _ ___ ___| |_| |_   _| |__   
|  __/ _` / __/ __|  _  | | | | '_ \  
| | | (_| \__ \__ \ | | | |_| | |_) | 
|_|  \__,_|___/___/_| |_|\__,_|_.__/  
        "#;

        // ASCII Art paragraph
        let art_paragraph = Paragraph::new(ascii_art)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);

        // Welcome message
        let welcome_text = Paragraph::new("Welcome to PassHub. Press Enter to continue.")
            .style(
                Style::default()
                    .fg(Color::Black),
            )
            .alignment(Alignment::Center);

        // Render ASCII art in the top half
        f.render_widget(art_paragraph, chunks[0]);

        // Render welcome text in the bottom half
        f.render_widget(welcome_text, chunks[1]);
    }).unwrap();

    // Wait for user to press 'Enter'
    loop {
        if let Event::Key(key) = event::read().unwrap() {
            if key.code == KeyCode::Enter {
                break;
            }
        }
    }

    // Clear and restore terminal
    terminal.clear().unwrap();
    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen).unwrap();
}