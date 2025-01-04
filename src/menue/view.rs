use log::debug;
use serde_json::Value;
use std::{error::Error,  io::{self, stdout,Error as ioError, ErrorKind, Stdout}, vec};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyEvent},
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,buffer::Buffer, layout::{Constraint, Direction, Layout, Rect}, prelude::Backend, style::{Color, Modifier, Style, Stylize}, text::{Line, Text, Span}, widgets::{Block, Borders, Clear, List, ListDirection, ListItem, ListState, Paragraph, Widget, Wrap}, Frame, Terminal
};
use derive_setters::Setters;



use super::logik::{self, get_uris, Entry};

pub fn display_data_empty() {
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
                .title("Main Menue");

            let paragraph = Paragraph::new("No data stored. \nPlease press p to add a new password.")
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


pub fn display_data(json_data: Value) -> Result<(), Box<dyn Error>> {
    let entries = super::logik::deserialize_json(json_data);
    let entries = match entries{
        Ok(e) => e,
        Err(..) => {debug!("There was an error while parsing JSON");
        panic!()},
    };
    let entries_2 = entries.clone();
    let uris = get_uris(entries);
    match uris{
        Ok(vector) => {display_uris(vector, entries_2)},
        Err(e)=> {debug!("Error while parsing JSON!");
    Err(Box::new(e))},
    }

}


pub fn display_uris(uris: Vec<String>, mut entries: Vec<Entry>) -> Result<(), Box<dyn Error>> {
    let mut stateful_list = StatefulList::new(uris);

    // Initializing the terminal with CrosstermBackend
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear().unwrap();
    enable_raw_mode().unwrap();

    let mut show_popup = false; // Track whether the popup is displayed
    let mut selected_index = 0; // Track selected entry index

    loop {
        terminal.draw(|f| {
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            // Render the list if no popup is active
            if !show_popup {
                let list_items: Vec<ListItem> = stateful_list
                    .items
                    .iter()
                    .map(|i| ListItem::new(i.as_str()))
                    .collect();

                let list = List::new(list_items)
                    .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL))
                    .highlight_style(Style::default().fg(Color::Yellow))
                    .highlight_symbol(">> ");

                f.render_stateful_widget(list, chunks[0], &mut stateful_list.state);
            } else {
                // Render the popup
                let popup = PasswordPopup::from_entry(&entries[selected_index]);
                let area = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![
                        Constraint::Percentage(20),
                        Constraint::Percentage(80),
                    ])
                    .split(f.area());

                f.render_widget(Clear, area[1]); // Clear the background
                f.render_widget(popup, area[1]); // Render the popup
            }
        })?;

        // Code for user input handling
        if let event::Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                if show_popup {
                    // Handle popup input
                    match key.code {
                        KeyCode::Esc => show_popup = false, // Close the popup
                        _ => {}
                    }
                } else {
                    // Handle list input
                    match key.code {
                        KeyCode::Down => stateful_list.next(),
                        KeyCode::Up => stateful_list.previous(),
                        KeyCode::Enter => {
                            selected_index = stateful_list.state.selected().unwrap_or(0);
                            show_popup = true; // Show the popup
                        }
                        KeyCode::Esc => break, // Exit the loop
                        _ => {}
                    }
                }
            }
        }
    }

    disable_raw_mode().unwrap();
    terminal.clear()?;
    Ok(())
}


//Noch Problem mit dem Frame


pub fn edit_password_popup(entry: &mut Entry, key_event: KeyEvent) -> bool {
    match key_event.code {
        KeyCode::Char('e') => {
            todo!();
            // Edit the first URI
            if let Some(uri) = entry.login.uris.get_mut(0) {
                uri.uri = "https://edited-uri.com".to_string();
            }
            true
        }
        KeyCode::Char('p') => {
            todo!();
            // Edit the password
            entry.login.password = "newpassword".to_string();
            true
        }
        _ => false,
    }
}


struct StatefulList {
    state: ListState,
    items: Vec<String>,
}

impl StatefulList {
    fn new(items: Vec<String>) -> StatefulList {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
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

            let paragraph = Paragraph::new("Session longer than one hour. \nPlease sign in again.")
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

            let paragraph = Paragraph::new("Internal Server Error! \nPlease sign in again.")
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

            let paragraph = Paragraph::new("Unknown Error. \nPlease sign in again.")
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

#[derive(Debug, Default, Setters)]
struct PasswordPopup<'a> {
    #[setters(into)]
    title: Line<'a>,
    #[setters(into)]
    content: Text<'a>,
    border_style: Style,
    title_style: Style,
    style: Style,
}

impl Widget for PasswordPopup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        let block = Block::new()
        .title(self.title)
        .title_style(self.title_style)
        .borders(Borders::ALL)
        .border_style(self.border_style);

        Paragraph::new(self.content)
        .wrap(Wrap {trim:true})
        .style(self.style)
        .block(block)
        .render(area, buf);
    }
}

impl<'a> PasswordPopup<'a> {
    pub fn from_entry(entry: &'a Entry) -> Self {
        // Build the popup content from the `Entry`
        let mut content = Text::default();
        content.lines.push(Line::from(vec![
            Span::raw("ID: "),
            Span::styled(&entry.id, Style::default().add_modifier(Modifier::BOLD)),
        ]));
        content.lines.push(Line::from(vec![
            Span::raw("Name: "),
            Span::styled(&entry.name, Style::default().add_modifier(Modifier::BOLD)),
        ]));
        if let Some(notes) = &entry.notes {
            content.lines.push(Line::from(vec![
                Span::raw("Notes: "),
                Span::styled(notes, Style::default().add_modifier(Modifier::ITALIC)),
            ]));
        }
        for uri in &entry.login.uris {
            content.lines.push(Line::from(vec![
                Span::raw("URI: "),
                Span::styled(&uri.uri, Style::default()),
            ]));
        }
        content.lines.push(Line::from(vec![
            Span::raw("Username: "),
            Span::styled(
                entry.login.username.as_deref().unwrap_or("(none)"),
                Style::default(),
            ),
        ]));
        content.lines.push(Line::from(vec![
            Span::raw("Password: "),
            Span::styled("********", Style::default().add_modifier(Modifier::DIM)),
        ]));
        if let Some(totp) = &entry.login.totp {
            content.lines.push(Line::from(vec![
                Span::raw("TOTP: "),
                Span::styled(totp, Style::default()),
            ]));
        }

        PasswordPopup {
            title: Line::from("Password Entry"),
            content,
            border_style: Style::default(),
            title_style: Style::default().add_modifier(Modifier::BOLD),
            style: Style::default(),
        }
    }
}