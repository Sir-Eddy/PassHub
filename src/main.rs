//Andere Dateien: src/view.rs, src/api.rs, src/logik.rs
mod view;
mod api; 
mod logik;

//Importieren benötigter Bibliotheken
use std::{io, string};
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};
use log::{debug, info, warn, error};
use env_logger;

//Main Funktion
fn main() {

    //Logging initialisieren
    env_logger::init();

    //Initialisierung des Terminals
    //let mut terminal = ratatui::init();
    //terminal.clear()?;

    //Abruf der BackendURL
    let backend_url: String = logik::get_backend_url();
    debug!("Backend URL: {}", backend_url);

}
