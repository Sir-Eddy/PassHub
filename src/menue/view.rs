use log::debug;
use serde_json::Value;
use std::{error::Error,  io::{self, stdout}, vec};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,buffer::Buffer, layout::{Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style}, text::{Line, Text, Span}, widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Widget, Wrap}, Frame, Terminal
};




use super::logik::{ self, get_uris, Entry, Login, Uri};

pub fn display_data_empty() {
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
                let block = Block::default().borders(Borders::ALL).title("Main Menue");

                let paragraph =
                    Paragraph::new("No data stored. \nPlease press p to add a new password.")
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

pub fn display_data(json_data: Value) -> Result<Vec<Entry>, Box<dyn Error>> {
    let entries = super::logik::deserialize_json(json_data);
    let entries = match entries {
        Ok(e) => e,
        Err(..) => {
            debug!("There was an error while parsing JSON");
            panic!()
        }
    };
    let entries_2 = entries.clone();
    let uris = get_uris(entries);
    match uris{
        Ok(_) => {display_uris( entries_2)},
        Err(e)=> {debug!("Error while parsing JSON!");
    Err(Box::new(e))},
    }
    

}

pub fn add_entry() -> Entry {

    let mut new_entry = Entry {
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

    let mut popup_fields = StatefulList::new(vec![
        "Name".to_string(),
        "URI".to_string(),
        "Username".to_string(),
        "Password".to_string(),
        "Notes".to_string(),
    ]);

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();

    loop {
        // Render the popup
        terminal.draw(|f| {
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(size);

            // Render the popup fields
            let list_items: Vec<ListItem> = popup_fields
                .items
                .iter()
                .map(|field| ListItem::new(field.clone()))
                .collect();

            let list = List::new(list_items)
                .block(Block::default().borders(Borders::ALL).title("Add New Entry"))
                .highlight_style(Style::default().fg(Color::Yellow))
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, chunks[0], &mut popup_fields.state);

            // Render the content of the selected field for editing
            let field_content = match popup_fields.state.selected() {
                Some(0) => &new_entry.name,
                Some(1) => &new_entry.login.uris[0].uri,
                Some(2) => new_entry.login.username.as_deref().unwrap_or("(none)"),
                Some(3) => &new_entry.login.password,
                Some(4) => new_entry.notes.as_deref().unwrap_or("(none)"),
                _ => "",
            };

            let paragraph = Paragraph::new(field_content)
                .block(Block::default().borders(Borders::ALL).title("Field Content"));

            f.render_widget(paragraph, chunks[1]);
        }).unwrap();

        // Handle user input in the popup
        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Esc => break, // Exit the popup
                KeyCode::Up => popup_fields.previous(),
                KeyCode::Down => popup_fields.next(),
                KeyCode::Enter => break, // Finalize entry
                KeyCode::Char(c) => {
                    // Edit the selected field
                    match popup_fields.state.selected() {
                        Some(0) => {
                            if logik::validate_string_length(&new_entry.name){
                                new_entry.name.push(c)}},
                        Some(1) => {
                            if logik::validate_string_length(&new_entry.login.uris[0].uri){
                                new_entry.login.uris[0].uri.push(c)}},
                        Some(2) => {
                            if new_entry.login.username.is_none() {
                                new_entry.login.username = Some(String::new());
                            }
                            if logik::validate_string_length(&new_entry.login.username.as_ref().unwrap()) {
                                new_entry.login.username.as_mut().unwrap().push(c);
                        }}
                        Some(3) => {
                            if logik::validate_string_length(&new_entry.login.password){
                            new_entry.login.password.push(c)}},
                        Some(4) => {
                            if new_entry.notes.is_none() {
                                new_entry.notes = Some(String::new());
                            }
                            if logik::validate_string_length(&new_entry.notes.as_ref().unwrap()){
                            new_entry.notes.as_mut().unwrap().push(c);
                        }}
                        _ => {}
                    }
                }
                KeyCode::Backspace => {
                    // Handle deletion of characters in the selected field
                    match popup_fields.state.selected() {
                        Some(0) => { new_entry.name.pop(); }
                        Some(1) => { new_entry.login.uris[0].uri.pop(); }
                        Some(2) => {
                            if let Some(username) = new_entry.login.username.as_mut() {
                                username.pop();
                            }
                        }
                        Some(3) => { new_entry.login.password.pop(); }
                        Some(4) => {
                            if let Some(notes) = new_entry.notes.as_mut() {
                                notes.pop();
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    terminal.clear().unwrap();

    new_entry
}                         




pub fn display_uris(mut entries: Vec<Entry>) -> Result<Vec<Entry>, Box<dyn Error>> {
    let name_list = entries.iter().map(|item| item.name.clone()).collect::<Vec<_>>();
    let mut stateful_list = StatefulList::new(name_list);

    // Initializing the terminal with CrosstermBackend
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear().unwrap();
    enable_raw_mode().unwrap();

    let mut show_popup = false;
    let mut selected_index = 0; // Track whether the popup is displayed
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
                    .block(
                        ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL),
                    )
                    .highlight_style(Style::default().fg(Color::Yellow))
                    .highlight_symbol(">> ");

                f.render_stateful_widget(list, chunks[0], &mut stateful_list.state);
            } else if let Some(popup) = &popup {
                // Render the popup
                let area = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
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
                            popup.edit_mode = match popup.edit_mode {
                                EditMode::None => EditMode::Uri,
                                EditMode::Uri => EditMode::Password,
                                EditMode::Password => EditMode::Username,
                                EditMode::Note => EditMode::Name,
                                EditMode::Username => EditMode::Note,
                                EditMode::Name => EditMode::Uri,
                            };
                        }
                    }
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
                            terminal.clear()?;
                            let new_entry: Entry = add_entry();
                            let new_entry_name = new_entry.name.clone();
                            popup = None;
                            entries.push(new_entry);
                            stateful_list.items.push(new_entry_name)
                        },
                        KeyCode::Delete => {
                            match stateful_list.get_selected_index() {
                                Some(index) => {
                                    popup = None;
                                    stateful_list.delete_selected();
                                    entries.remove(index);
                                },
                                None => {}
                            };
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
    fn get_selected_index(& self) -> Option<usize> {
        let index = match self.state.selected() {
            Some(index) => {Some(index)},
            None => {None}
        };
        index
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
        terminal
            .draw(|frame| {
                let size = frame.area();
                let block = Block::default().borders(Borders::ALL).title("Error");

                let paragraph =
                    Paragraph::new("Session longer than one hour. \nPlease sign in again.")
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

pub fn database_error() {
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
                    Paragraph::new("Internal Server Error! \nPlease sign in again.").block(block);

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

pub fn update_error(code: i16) {
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
                let paragraph = match code {
                    401 => Paragraph::new("JWT Token is invalid!").block(block),
                    500 => Paragraph::new("Database Error or JWT Extraction Error!").block(block),
                    _ => Paragraph::new("Unknown Error!").block(block),
                };
                frame.render_widget(paragraph, size);
            })
            .unwrap();
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
        terminal
            .draw(|frame| {
                let size = frame.area();
                let block = Block::default().borders(Borders::ALL).title("Error");

                let paragraph =
                    Paragraph::new("Unknown Error. \nPlease sign in again.").block(block);

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
        terminal
            .draw(|frame| {
                let size = frame.area();
                let block = Block::default().borders(Borders::ALL).title("Error");
                let paragraph = Paragraph::new("Serialization Error!").block(block);

                frame.render_widget(paragraph, size);
            })
            .unwrap();
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
        Clear.render(area, buf);
        content.lines.push(Line::from(vec![
            Span::raw("URI: "),
            if matches!(self.edit_mode, EditMode::Uri) {
                Span::styled(
                    &self.entry.login.uris[0].uri,
                    Style::default().fg(Color::Cyan),
                )
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
            if matches!(self.edit_mode, EditMode::Username){
                if self.entry.login.username.is_some(){
                Span::styled(self.entry.login.username.as_ref().unwrap(), Style::default().fg(Color::Cyan))}
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
            ),
        ]));
        //if \n in the string to be rendered, there shall be a line break
        if let Some(notes) = self.entry.notes.as_deref() {

            let lines: Vec<&str> = notes.split('\n').collect();
        
            for (i, line) in lines.iter().enumerate() {
                content.lines.push(Line::from(vec![
                    if i == 0 {
                        Span::raw("Notes: ")
                    } else {
                        Span::raw("      ") // Align subsequent lines
                    },
                    if matches!(self.edit_mode, EditMode::Note) {
                        Span::styled(*line, Style::default().fg(Color::Cyan))
                    } else {
                        Span::raw(*line)
                    },
                ]));
            }
        } else {
            // When Notes is None
            content.lines.push(Line::from(vec![
                Span::raw("Notes: "),
                Span::styled("(none)", Style::default().fg(Color::Cyan)),
            ]));
        }
        

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
                KeyCode::Char(c) => {
                    if logik::validate_string_length(&self.entry.login.uris[0].uri){ 
                    self.entry.login.uris[0].uri.push(c)}
            },
                KeyCode::Backspace => {
                    self.entry.login.uris[0].uri.pop();
                }
                KeyCode::Tab => self.edit_mode = EditMode::Password,
                _ => {}
            },
            EditMode::Password => match key {
                KeyCode::Char(c) => {
                    if logik::validate_string_length(&self.entry.login.password){
                    self.entry.login.password.push(c)}
                },
                KeyCode::Backspace => {
                    self.entry.login.password.pop();
                }
                KeyCode::Tab => self.edit_mode = EditMode::Username,
                _ => {}
            },
            EditMode::Note => match key {
                KeyCode::Char(c) => {
                    if self.entry.notes.is_some(){
                        if logik::validate_string_length(&self.entry.notes.as_ref().unwrap()){
                        self.entry.notes.as_mut().unwrap().push(c);
                    }
                }
                    else {
                        self.entry.notes = Some(String::new());
                        self.entry.notes.as_mut().unwrap().push(c)
                    }
                    
                }
                KeyCode::Backspace => {
                    if self.entry.notes.is_some() {
                        self.entry.notes.as_mut().unwrap().pop();
                    } else {
                    }
                }
                KeyCode::Enter => {
                    if self.entry.notes.is_none() {
                        self.entry.notes = Some(String::new());
                    }
                    self.entry.notes.as_mut().unwrap().push('\n'); // Add a newline
                }
                KeyCode::Enter => {
                    if self.entry.notes.is_none() {
                        self.entry.notes = Some(String::new());
                    }
                    self.entry.notes.as_mut().unwrap().push('\n'); // Add a newline
                }
                KeyCode::Tab => self.edit_mode = EditMode::Uri,
                _ => {}
            },
            EditMode::Username => match key {
                KeyCode::Char(c) => {
                    if self.entry.login.username.is_some(){
                        if logik::validate_string_length(&self.entry.login.username.as_ref().unwrap()){
                        self.entry.login.username.as_mut().unwrap().push(c)}
                    }
                    else {
                        self.entry.login.username = Some(String::new());
                        self.entry.login.username.as_mut().unwrap().push(c)
                    }
                }
                KeyCode::Backspace => {
                    if self.entry.login.username.is_some() {
                        self.entry.login.username.as_mut().unwrap().pop();
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
