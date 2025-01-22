use copypasta::{ClipboardContext, ClipboardProvider};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::debug;
use ratatui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Widget, Wrap},
    Terminal,
};
use serde_json::Value;
use std::{
    error::Error,
    io::{self, stdout},
    vec,
};

use super::logik::{self, get_uris, Entry, Login, Uri};

pub fn display_data_empty() -> Entry {
    // Setup terminal for error screen
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    let new_entry: Entry;

    loop {
        terminal
            .draw(|frame| {
                let size = frame.area();
                let block = Block::default()
                    .borders(Borders::ALL)
                    .title("PassHub")
                    .border_style(Style::default().fg(Color::Rgb(255, 163, 26)))
                    .title_style(Style::default().add_modifier(Modifier::BOLD));

                let paragraph =
                    Paragraph::new("No data stored. \nPlease press + to add a new password.")
                        .block(block);

                frame.render_widget(paragraph, size);
            })
            .unwrap();

        // Wait for user input to dismiss the error screen
        if let Event::Key(key_event) = event::read().unwrap() {
            if key_event.kind == KeyEventKind::Press {
                if let KeyCode::Char('+') = key_event.code {
                    new_entry = add_entry();
                    break;
                }
            }
        }
    }
    new_entry
}

pub fn display_data(json_data: &Value) -> Result<(Vec<Entry>, bool), Box<dyn Error>> {
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
    match uris {
        Ok(_) => display_uris(entries_2),
        Err(e) => {
            debug!("Error while parsing JSON!");
            Err(Box::new(e))
        }
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
        terminal
            .draw(|f| {
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
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Rgb(255, 163, 26)))
                            .title("Add New Entry")
                            .title_style(Style::default().add_modifier(Modifier::BOLD))
                            .title_bottom("Navigate (arrow keys), Edit (type), Save (Enter)"),
                    )
                    .highlight_style(Style::default().fg(Color::Rgb(255, 163, 26)))
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

                let paragraph = Paragraph::new(field_content).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Field Content")
                        .title_style(Style::default().add_modifier(Modifier::BOLD)),
                );

                f.render_widget(paragraph, chunks[1]);
            })
            .unwrap();

        // Handle user input in the popup
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Up => popup_fields.previous(),
                    KeyCode::Down => popup_fields.next(),
                    KeyCode::Enter => {
                        if popup_fields.state.selected() == Some(4) {
                            if new_entry.notes.is_none() {
                                new_entry.notes = Some(String::new());
                            }
                            new_entry.notes.as_mut().unwrap().push('\n');
                        } else if new_entry.name.is_empty() {
                            // Ensure the name field is mandatory
                            terminal
                                .draw(|f| {
                                    let size = f.area();
                                    let popup = Paragraph::new("Name cannot be empty!")
                                        .block(Block::default().borders(Borders::ALL))
                                        .alignment(Alignment::Center);
                                    f.render_widget(popup, size);
                                })
                                .unwrap();
                            std::thread::sleep(std::time::Duration::from_secs(2));
                        } else {
                            break; // Exit if name is not empty
                        }
                    }

                    , // Finalize entry
                    KeyCode::Char(c) => {
                        // Edit the selected field
                        match popup_fields.state.selected() {
                            Some(0) => {
                                if logik::validate_string_length(&new_entry.name) {
                                    new_entry.name.push(c)
                                }
                            }
                            Some(1) => {
                                if logik::validate_string_length(&new_entry.login.uris[0].uri) {
                                    new_entry.login.uris[0].uri.push(c)
                                }
                            }
                            Some(2) => {
                                if new_entry.login.username.is_none() {
                                    new_entry.login.username = Some(String::new());
                                }
                                if logik::validate_string_length(
                                    new_entry.login.username.as_ref().unwrap(),
                                ) {
                                    new_entry.login.username.as_mut().unwrap().push(c);
                                }
                            }
                            Some(3) => {
                                if logik::validate_string_length(&new_entry.login.password) {
                                    new_entry.login.password.push(c)
                                }
                            }
                            Some(4) => {
                                if new_entry.notes.is_none() {
                                    new_entry.notes = Some(String::new());
                                }
                                if logik::validate_string_length(new_entry.notes.as_ref().unwrap())
                                {
                                    new_entry.notes.as_mut().unwrap().push(c);
                                }
                            }
                            _ => {}
                        }
                    }
                    KeyCode::Backspace => {
                        // Handle deletion of characters in the selected field
                        match popup_fields.state.selected() {
                            Some(0) => {
                                new_entry.name.pop();
                            }
                            Some(1) => {
                                new_entry.login.uris[0].uri.pop();
                            }
                            Some(2) => {
                                if let Some(username) = new_entry.login.username.as_mut() {
                                    username.pop();
                                }
                            }
                            Some(3) => {
                                new_entry.login.password.pop();
                            }
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
    }

    terminal.clear().unwrap();

    new_entry
}

pub fn display_uris(mut entries: Vec<Entry>) -> Result<(Vec<Entry>, bool), Box<dyn Error>> {
    let mut to_save: bool = false;
    let name_list = entries
        .iter()
        .map(|item| item.name.clone())
        .collect::<Vec<_>>();
    let mut stateful_list = StatefulList::new(name_list);

    // Initializing the terminal with CrosstermBackend
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear().unwrap();
    enable_raw_mode().unwrap();

    let mut show_popup = false;
    let mut selected_index; // Track whether the popup is displayed
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
                    .block(ratatui::widgets::Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .border_style(Style::default().fg(Color::Rgb(255, 163, 26)))
                    .title("PassHub")
                    .title_bottom("Add entry (+), delete entry (DEL), save and log out (ESC), navigate (arrow keys)")
                    .title_style(Style::default()
                    .add_modifier(Modifier::BOLD)))
                    .highlight_style(Style::default().fg(Color::Rgb(255, 163, 26)))
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
                            KeyCode::Esc => {
                                break;
                            }
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
                            _ => popup.handle_input(key.code, key.modifiers),
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
                            entries.push(new_entry);
                            stateful_list.items.push(new_entry_name);
                            break;
                        }
                        KeyCode::Delete => {
                            if let Some(index) = stateful_list.get_selected_index() {
                                stateful_list.delete_selected();
                                entries.remove(index);
                                break;
                            };
                        }
                        KeyCode::Esc => {
                            to_save = true;
                            break;
                        } // Exit the loop
                        _ => {}
                    }
                }
            }
        }
    }

    disable_raw_mode().unwrap();
    terminal.clear()?;

    Ok((entries, to_save))
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
    fn get_selected_index(&self) -> Option<usize> {
        self.state.selected()
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

    fn delete_selected(&mut self) {
        if let Some(i) = self.state.selected() {
            self.items.remove(i);
        };
    }
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
                let block = Block::default()
                    .borders(Borders::ALL)
                    .title("Error")
                    .title_style(Style::default().add_modifier(Modifier::BOLD));
                let paragraph = match code {
                    401 => Paragraph::new("Logout successfull!").block(block),
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
                let block = Block::default()
                    .borders(Borders::ALL)
                    .title("Error")
                    .title_style(Style::default().add_modifier(Modifier::BOLD));
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
    _title: Line<'a>,
    _border_style: Style,
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
            _title: Line::from("Password Entry"),
            _border_style: Style::default(),
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
                    Style::default().fg(Color::Rgb(255, 163, 26)),
                )
            } else {
                Span::raw(&self.entry.login.uris[0].uri)
            },
        ]));
        content.lines.push(Line::from(vec![
            Span::raw("Password: "),
            if matches!(self.edit_mode, EditMode::Password) {
                Span::styled(
                    &self.entry.login.password,
                    Style::default().fg(Color::Rgb(255, 163, 26)),
                )
            } else {
                Span::raw("********")
            },
        ]));
        content.lines.push(Line::from(vec![
            Span::raw("Username: "),
            if matches!(self.edit_mode, EditMode::Username) {
                if self.entry.login.username.is_some() {
                    Span::styled(
                        self.entry.login.username.as_ref().unwrap(),
                        Style::default().fg(Color::Rgb(255, 163, 26)),
                    )
                } else {
                    Span::raw("(none)").style(Style::default().fg(Color::Rgb(255, 163, 26)))
                }
            } else if self.entry.login.username.is_some() {
                Span::styled(
                    self.entry.login.username.as_ref().unwrap(),
                    Style::default(),
                )
            } else {
                Span::styled("(none)", Style::default())
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
                        Span::styled(*line, Style::default().fg(Color::Rgb(255, 163, 26)))
                    } else {
                        Span::raw(*line)
                    },
                ]));
            }
        } else {
            content.lines.push(Line::from(vec![
                Span::raw("Notes: "),
                if matches!(self.edit_mode, EditMode::Note) {
                    Span::styled("(none)", Style::default().fg(Color::Rgb(255, 163, 26)))
                } else {
                    Span::styled("(none)", Style::default())
                },
            ]));
        }

        // Render the block and paragraph
        let block = Block::default()
            .title(Line::from("Password Entry"))
            .title_style(Style::default().add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_style(Style::default())
            .title_bottom(Line::from("Switch between fields (TAB), Return (ESC)"));

        Paragraph::new(content)
            .block(block)
            .style(self.style)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }

    pub fn handle_input(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        match self.edit_mode {
            EditMode::Uri => match (key, modifiers) {
                (
                    KeyCode::Char(c),
                    KeyModifiers::NONE | KeyModifiers::SHIFT | KeyModifiers::ALT,
                ) => {
                    if logik::validate_string_length(&self.entry.login.uris[0].uri) {
                        self.entry.login.uris[0].uri.push(c);
                    }
                }
                //Alt Gr
                (KeyCode::Char(c), m)
                    if m.contains(KeyModifiers::ALT) && m.contains(KeyModifiers::CONTROL) =>
                {
                    if logik::validate_string_length(&self.entry.login.uris[0].uri) {
                        self.entry.login.uris[0].uri.push(c);
                    }
                }
                (KeyCode::Backspace, KeyModifiers::NONE) => {
                    self.entry.login.uris[0].uri.pop();
                }
                (KeyCode::Tab, KeyModifiers::NONE) => self.edit_mode = EditMode::Password,
                // Copy (Ctrl + C)
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                    if let Some(uri) = self.entry.login.uris.first() {
                        let uri_clone = uri.uri.clone();
                        copy_to_clipboard(uri_clone);
                    }
                }
                // Paste (Ctrl + V)
                (KeyCode::Char('v'), KeyModifiers::CONTROL) => {
                    if let Some(content) = paste_from_clipboard() {
                        if logik::validate_string_length(&content) {
                            self.entry.login.uris[0].uri.push_str(&content);
                        }
                    }
                }
                _ => {}
            },
            EditMode::Password => match (key, modifiers) {
                (
                    KeyCode::Char(c),
                    KeyModifiers::NONE | KeyModifiers::SHIFT | KeyModifiers::ALT,
                ) => {
                    if logik::validate_string_length(&self.entry.login.password) {
                        self.entry.login.password.push(c);
                    }
                }
                //Alt Gr
                (KeyCode::Char(c), m)
                    if m.contains(KeyModifiers::ALT) && m.contains(KeyModifiers::CONTROL) =>
                {
                    if logik::validate_string_length(&self.entry.login.password) {
                        self.entry.login.password.push(c);
                    }
                }
                (KeyCode::Backspace, KeyModifiers::NONE) => {
                    self.entry.login.password.pop();
                }
                (KeyCode::Tab, KeyModifiers::NONE) => self.edit_mode = EditMode::Username,
                // Copy (Ctrl + C)
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                    let password_clone = self.entry.login.password.clone();
                    copy_to_clipboard(password_clone);
                }
                // Paste (Ctrl + V)
                (KeyCode::Char('v'), KeyModifiers::CONTROL) => {
                    if let Some(content) = paste_from_clipboard() {
                        if logik::validate_string_length(&content) {
                            self.entry.login.password.push_str(&content);
                        }
                    }
                }
                _ => {}
            },
            EditMode::Note => match (key, modifiers) {
                (
                    KeyCode::Char(c),
                    KeyModifiers::NONE | KeyModifiers::SHIFT | KeyModifiers::ALT,
                ) => {
                    if self.entry.notes.is_some() {
                        if logik::validate_string_length(self.entry.notes.as_ref().unwrap()) {
                            self.entry.notes.as_mut().unwrap().push(c);
                        }
                    } else {
                        self.entry.notes = Some(String::new());
                        self.entry.notes.as_mut().unwrap().push(c);
                    }
                }
                //Alt Gr
                (KeyCode::Char(c), m)
                    if m.contains(KeyModifiers::ALT) && m.contains(KeyModifiers::CONTROL) =>
                {
                    if self.entry.notes.is_some() {
                        if logik::validate_string_length(self.entry.notes.as_ref().unwrap()) {
                            self.entry.notes.as_mut().unwrap().push(c);
                        }
                    } else {
                        self.entry.notes = Some(String::new());
                        self.entry.notes.as_mut().unwrap().push(c);
                    }
                }
                (KeyCode::Backspace, KeyModifiers::NONE) => {
                    if self.entry.notes.is_some() {
                        self.entry.notes.as_mut().unwrap().pop();
                    }
                }
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    if self.entry.notes.is_none() {
                        self.entry.notes = Some(String::new());
                    }
                    self.entry.notes.as_mut().unwrap().push('\n');
                }
                (KeyCode::Tab, KeyModifiers::NONE) => self.edit_mode = EditMode::Uri,
                // Copy (Ctrl + C)
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                    if let Some(notes) = &self.entry.notes {
                        copy_to_clipboard(notes.clone());
                    }
                }
                // Paste (Ctrl + V)
                (KeyCode::Char('v'), KeyModifiers::CONTROL) => {
                    if let Some(content) = paste_from_clipboard() {
                        if self.entry.notes.is_some() {
                            if logik::validate_string_length(&content) {
                                self.entry.notes.as_mut().unwrap().push_str(&content);
                            }
                        } else {
                            self.entry.notes = Some(content);
                        }
                    }
                }
                _ => {}
            },
            EditMode::Username => match (key, modifiers) {
                (
                    KeyCode::Char(c),
                    KeyModifiers::NONE | KeyModifiers::SHIFT | KeyModifiers::ALT,
                ) => {
                    if self.entry.login.username.is_some() {
                        if logik::validate_string_length(
                            self.entry.login.username.as_ref().unwrap(),
                        ) {
                            self.entry.login.username.as_mut().unwrap().push(c);
                        }
                    } else {
                        self.entry.login.username = Some(String::new());
                        self.entry.login.username.as_mut().unwrap().push(c);
                    }
                }
                //Alt Gr
                (KeyCode::Char(c), m)
                    if m.contains(KeyModifiers::ALT) && m.contains(KeyModifiers::CONTROL) =>
                {
                    if self.entry.login.username.is_some() {
                        if logik::validate_string_length(
                            self.entry.login.username.as_ref().unwrap(),
                        ) {
                            self.entry.login.username.as_mut().unwrap().push(c);
                        }
                    } else {
                        self.entry.login.username = Some(String::new());
                        self.entry.login.username.as_mut().unwrap().push(c);
                    }
                }
                (KeyCode::Backspace, KeyModifiers::NONE) => {
                    if self.entry.login.username.is_some() {
                        self.entry.login.username.as_mut().unwrap().pop();
                    }
                }
                (KeyCode::Tab, KeyModifiers::NONE) => self.edit_mode = EditMode::Note,
                // Copy (Ctrl + C)
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                    if let Some(username) = &self.entry.login.username {
                        copy_to_clipboard(username.clone());
                    }
                }
                // Paste (Ctrl + V)
                (KeyCode::Char('v'), KeyModifiers::CONTROL) => {
                    if let Some(content) = paste_from_clipboard() {
                        if self.entry.login.username.is_some() {
                            if logik::validate_string_length(&content) {
                                self.entry
                                    .login
                                    .username
                                    .as_mut()
                                    .unwrap()
                                    .push_str(&content);
                            }
                        } else {
                            self.entry.login.username = Some(content);
                        }
                    }
                }
                _ => {}
            },
            EditMode::None => {}
            EditMode::Name => {}
        }
    }
}

fn copy_to_clipboard(content: String) {
    let mut clipboard = ClipboardContext::new().unwrap();
    clipboard.set_contents(content).unwrap();
}

fn paste_from_clipboard() -> Option<String> {
    if let Ok(mut clipboard) = ClipboardContext::new() {
        if let Ok(content) = clipboard.get_contents() {
            return Some(content);
        }
    }
    None
}
