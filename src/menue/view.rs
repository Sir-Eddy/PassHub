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



use super::logik::{self, get_uris, serialize_json, Entry, Uri, Login};

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


pub fn display_data(json_data: Value) -> Result<Vec<Entry>, Box<dyn Error>> {
    let entries = super::logik::deserialize_json(json_data);
    let entries = match entries{
        Ok(e) => e,
        Err(..) => {debug!("There was an error while parsing JSON");
        panic!()},
    };
    let entries_2 = entries.clone();
    let uris = get_uris(entries);
    match uris{
        Ok(vector) => {display_uris( entries_2)},
        Err(e)=> {debug!("Error while parsing JSON!");
    Err(Box::new(e))},
    }
    

}

pub fn add_entry() -> Entry {
    enable_raw_mode().unwrap();
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();

    let mut entry = Entry {
        id: String::new(),
        name: String::new(),
        notes: None,
        login: Login {
            uris: vec![Uri { uri: String::new() }],
            username: None,
            password: String::new(),
            totp: None,
        },
    };

    let mut edit_mode = EditMode::Uri; // Start editing URIs
    loop {
        terminal.draw(|f| {
            let size = f.area();
            let block = Block::default().title("New Entry").borders(Borders::ALL);

            let content = match edit_mode {
                EditMode::Uri => format!("URI: {}", entry.login.uris[0].uri),
                EditMode::Password => format!("Password: {}", entry.login.password),
                EditMode::Note => format!("Notes: {}", entry.notes.as_deref().unwrap_or("")),
                EditMode::Username => format!("Username: {}", entry.login.username.as_deref().unwrap_or("")),
                EditMode::Name => format!("Name: {}", entry.name),
                _ => String::new(),
            };

            let paragraph = Paragraph::new(content).block(block);
            f.render_widget(paragraph, size);
        }).unwrap();

        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Enter => break, // Finish editing
                KeyCode::Tab => {
                    // Switch editing modes
                    edit_mode = match edit_mode {
                        EditMode::Uri => EditMode::Password,
                        EditMode::Password => EditMode::Note,
                        EditMode::Note => EditMode::Username,
                        EditMode::Username => EditMode::Name,
                        EditMode::Name => EditMode::Uri,
                        _ => EditMode::Uri,
                    };
                }
                KeyCode::Char(c) => match edit_mode {
                    EditMode::Uri => entry.login.uris[0].uri.push(c),
                    EditMode::Password => entry.login.password.push(c),
                    EditMode::Note => {
                        if entry.notes.is_none() {
                            entry.notes = Some(String::new());
                        }
                        entry.notes.as_mut().unwrap().push(c);
                    }
                    EditMode::Username => {
                        if entry.login.username.is_none() {
                            entry.login.username = Some(String::new());
                        }
                        entry.login.username.as_mut().unwrap().push(c);
                    }
                    EditMode::Name => entry.name.push(c),
                    _ => {}
                },
                KeyCode::Backspace => match edit_mode {
                    EditMode::Uri => { entry.login.uris[0].uri.pop(); }
                    EditMode::Password => { entry.login.password.pop(); }
                    EditMode::Note => {
                        if let Some(notes) = &mut entry.notes {
                            notes.pop();
                        }
                    }
                    EditMode::Username => {
                        if let Some(username) = &mut entry.login.username {
                            username.pop();
                        }
                    }
                    EditMode::Name => {entry.name.pop();}
                    _ => {}
                },
                KeyCode::Esc => break, // Exit without saving
                _ => {}
            }
        }
    }

    terminal.clear().unwrap();

    entry
}



pub fn display_uris(mut entries: Vec<Entry>) -> Result<Vec<Entry>, Box<dyn Error>> {
    let mut name_list = entries.iter().map(|item| item.name.clone()).collect::<Vec<_>>();
    let mut stateful_list = StatefulList::new(name_list);

    // Initializing the terminal with CrosstermBackend
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear().unwrap();
    enable_raw_mode().unwrap();

    let mut show_popup = false; // Track whether the popup is displayed
    let mut selected_index = 0;
    let mut popup: Option<PasswordPopup> = None; // Track selected entry index

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
            } else if let Some(popup) = &popup{
                // Render the popup
                let area = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![
                        Constraint::Percentage(30),
                        Constraint::Percentage(70),
                    ])
                    .split(f.area());

                //f.render_widget(Clear, area[1]); // Clear the background
                popup.render(area[1], f.buffer_mut()); // Render the popup
            }
        })?;

        // Code for user input handling
        if let event::Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                if show_popup {
                    if let Some(popup) = popup.as_mut() {
                    match key.code {
                        KeyCode::Esc => show_popup = false,
                        KeyCode::Tab => {
                            popup.edit_mode = match popup.edit_mode{
                                EditMode::None => EditMode::Uri,
                                EditMode::Uri => EditMode::Password,
                                EditMode::Password => EditMode::Note,
                                EditMode::Note => EditMode::Uri,
                                EditMode::Username => EditMode::Username,
                                EditMode::Name => EditMode::None,
                            };
                        }
                        _ => popup.handle_input(key.code),
                    }}
                } else {
                    
                    match key.code {
                        KeyCode::Down => stateful_list.next(),
                        KeyCode::Up => stateful_list.previous(),
                        KeyCode::Enter => {
                            selected_index = stateful_list.state.selected().unwrap_or(0);
                            popup = Some(PasswordPopup::from_entry(&mut entries[selected_index]));
                            show_popup = true; // Show the popup
                        }
                        KeyCode::Char('+') => {
                            let new_entry: Entry = add_entry();
                            let new_entry_name = new_entry.name.clone();
                            popup = None;
                            entries.push(new_entry);
                            stateful_list.items.push(new_entry_name)
                        },
                        KeyCode::Esc => break, // Exit the loop
                        _ => {}
                    }
                }
            }
        }
    }
    
    disable_raw_mode().unwrap();
    terminal.clear()?;
    
    Ok(entries)
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

pub fn update_error(code: i16) {
    
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        terminal.draw(|frame|{
            let size = frame.area();
            let block = Block::default()
                .borders(Borders::ALL)
                .title("Error");
            let paragraph = match code {
                401 => Paragraph::new("JWT Token is invalid!")
                .block(block),
                500 => Paragraph::new("Database Error or JWT Extraction Error!")
                .block(block),
                _ => {Paragraph::new("Unknown Error!")
                    .block(block)},
            };
            frame.render_widget(paragraph, size);

        }).unwrap();
        if let Event::Key(key_event) = event::read().unwrap() {
            if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Enter {
                break;
            }
        }

    }
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
    disable_raw_mode().unwrap();
    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
}  

pub fn serialization_error() {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        terminal.draw(|frame|{
            let size = frame.area();
            let block = Block::default()
                .borders(Borders::ALL)
                .title("Error");
            let paragraph = Paragraph::new("Serialization Error!")
                    .block(block);
            
            frame.render_widget(paragraph, size);

        }).unwrap();
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

#[derive(Debug)]
struct PasswordPopup<'a> {
    title: Line<'a>,
    border_style: Style,
    title_style: Style,
    style: Style,
    edit_mode: EditMode,
    entry: &'a mut Entry,
}


#[derive(Debug)]
enum EditMode {
    None,
    Uri,
    Password,
    Note,
    Username,
    Name,
}

impl<'a> PasswordPopup<'a> {
    pub fn from_entry(entry: &'a mut Entry) -> Self {
        PasswordPopup {
            title: Line::from("Password Entry"),
            border_style: Style::default(),
            title_style: Style::default().add_modifier(Modifier::BOLD),
            style: Style::default(),
            entry,
            edit_mode: EditMode::None,
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        let mut content = Text::default();
        Clear.render(area,buf);
        content.lines.push(Line::from(vec![
            Span::raw("URI: "),
            if matches!(self.edit_mode, EditMode::Uri) {
                Span::styled(&self.entry.login.uris[0].uri, Style::default().fg(Color::Cyan))
            } else {
                Span::raw(&self.entry.login.uris[0].uri)
            },
        ]));
        content.lines.push(Line::from(vec![
            Span::raw("Password: "),
            if matches!(self.edit_mode, EditMode::Password) {
                Span::styled(&self.entry.login.password, Style::default().fg(Color::Cyan))
            } else {
                Span::raw("********")
            },
        ]));
        content.lines.push(Line::from(vec![
            Span::raw("Username: "),
            if matches!(self.edit_mode, EditMode::Note){
                if self.entry.notes.is_some(){
                Span::styled(self.entry.notes.as_ref().unwrap(), Style::default().fg(Color::Cyan))}
                else {
                    Span::raw("(none)").style(Style::default().fg(Color::Cyan))
                }
            } else {
                Span::raw("(none)")
            },
        ]));
        
        content.lines.push(Line::from(vec![
            Span::raw("totp: "),
            Span::styled(
                self.entry.login.totp.as_deref().unwrap_or("(none)"),
                Style::default(),
            )
        ]));

        content.lines.push(Line::from(vec![
            Span::raw("Notes: "),
            if matches!(self.edit_mode, EditMode::Note){
                if self.entry.notes.is_some(){
                Span::styled(self.entry.notes.as_ref().unwrap(), Style::default().fg(Color::Cyan))}
                else {
                    Span::raw("(none)").style(Style::default().fg(Color::Cyan))
                }
            } else {
                Span::raw("(none)")
            },
        ]));

        // Render the block and paragraph
        let block = Block::default()
            .title(self.title.clone())
            .borders(Borders::ALL)
            .border_style(self.border_style);

        Paragraph::new(content)
            .block(block)
            .style(self.style)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }

    pub fn handle_input(&mut self, key: KeyCode) {
        match self.edit_mode {
            EditMode::Uri => match key {
                KeyCode::Char(c) => self.entry.login.uris[0].uri.push(c),
                KeyCode::Backspace => {
                    self.entry.login.uris[0].uri.pop();
                }
                KeyCode::Tab => self.edit_mode = EditMode::Password,
                _ => {}
            },
            EditMode::Password => match key {
                KeyCode::Char(c) => self.entry.login.password.push(c),
                KeyCode::Backspace => {
                    self.entry.login.password.pop();
                }
                KeyCode::Tab => self.edit_mode = EditMode::Note,
                _ => {}
            },
            EditMode::Note => match key {
                KeyCode::Char(c) => {
                    if self.entry.notes.is_some(){
                        self.entry.notes.as_mut().unwrap().push(c)
                    }
                    else {
                        self.entry.notes = Some(String::new());
                        self.entry.notes.as_mut().unwrap().push(c)
                    }
                }
                KeyCode::Backspace => {
                    if self.entry.notes.is_some() {
                        self.entry.notes.as_mut().unwrap().pop();
                    }
                    else{}
                }
                KeyCode::Tab => self.edit_mode = EditMode::Uri,
                _ => {}
            },
            EditMode::Username => match key {
                KeyCode::Char(c) => {
                    if self.entry.notes.is_some(){
                        self.entry.notes.as_mut().unwrap().push(c)
                    }
                    else {
                        self.entry.notes = Some(String::new());
                        self.entry.notes.as_mut().unwrap().push(c)
                    }
                }
                KeyCode::Backspace => {
                    if self.entry.notes.is_some() {
                        self.entry.notes.as_mut().unwrap().pop();
                    }
                    else{}
                }
                KeyCode::Tab => self.edit_mode = EditMode::Note,
                _ => {}
            },
            EditMode::None => {},
            EditMode::Name => {},
        }
    }
}
