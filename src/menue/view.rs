use log::debug;
use serde_json::Value;
use std::{error::Error,  io::{self, stdout}};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    Frame,
    backend::CrosstermBackend, layout::{Constraint, Direction, Layout, Rect}, prelude::Backend, style::{Color, Modifier, Style}, widgets::{Block, Borders, List, ListDirection, ListItem, ListState, Paragraph}, Terminal
};

use super::logik;


pub fn display_data(json_data: Value) -> Result<(), Box<dyn Error>> {
    todo!("An Änderungen in logik.rs anpassen");
    let uris = super::logik::get_uris(json_data);
    match uris{
        Ok(vector) => {display_uris(vector)},
        Err(e)=> {debug!("Error while parsing JSON!");
    Err(Box::new(e))},
    }

}


pub fn display_uris(uris: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    todo!("An Änderungen in display_data anpassen");
    // Convert URIs to ListItem objects for TUI
    let uri_items: Vec<ListItem> = uris
        .into_iter()
        .map(|uri| ListItem::new(format!("Page: {}", uri)))
        .collect();

    // Initialize the terminal with CrosstermBackend
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create the List widget
    loop {
        
    let list = List::new(uri_items)
        .block(Block::default().title("Password URIs").borders(ratatui::widgets::Borders::ALL))
        .highlight_style(Style::default().bg(Color::Blue))
        .highlight_symbol(">>");

    // Render the List in the terminal
    terminal.draw(|f| {
        let size = f.area();
        f.render_widget(list, size);
    })?;

    //Here belongs some Code for user input handling
    todo!();
    }

    Ok(())
}





pub fn invalid_token() {
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

            let paragraph = Paragraph::new("Session longer than one hour. \n Please sign in again.")
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

pub fn database_error() {
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

            let paragraph = Paragraph::new("Internal Server Error! \n Please sign in again.")
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

pub fn unknown_error() {
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

            let paragraph = Paragraph::new("Unknown Error. \n Please sign in again.")
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