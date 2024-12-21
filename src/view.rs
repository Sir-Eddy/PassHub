use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Layout, Constraint, Direction},
    style::{Color, Style},
    widgets::Paragraph,
    Terminal,
};
use std::io::stdout;

pub fn draw_welcome_screen() -> Option<char> {
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
            .split(f.area());

        // ASCII Art
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

        // Welcome message with instructions
        let welcome_text = Paragraph::new("Welcome to PassHub.\nPress Enter to continue.\nFirst time here? Press 'r' to register.")
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

    // Wait for user input
    let result = loop {
        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Enter => break None, // Continue without registration
                KeyCode::Char('r') => break Some('r'), // Return 'r' for registration
                _ => {}
            }
        }
    };

    // Clear and restore terminal
    terminal.clear().unwrap();
    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen).unwrap();

    result
}
