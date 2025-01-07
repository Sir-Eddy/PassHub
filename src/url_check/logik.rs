use super::api;
use super::view;

use directories::ProjectDirs;
use std::fs;
use std::io::Read;
use std::io::Write;
use url::Url;

pub fn get_backend_url() -> String {
    let mut backend_url = is_url_in_storage();

    // Prüfe die Erreichbarkeit der gespeicherten URL
    if let Some(ref url) = backend_url {
        if !api::check_health(url) {
            view::error_url_unreachable(&Some(url.clone()));
            backend_url = None; // Zurücksetzen, um neue Eingabe zu erzwingen
        }
    }

    loop {
        if backend_url.is_none() {
            create_file(); // Erstelle die Datei, falls sie nicht existiert
                           // Eingabe-Schleife für die URL
            loop {
                let temp_url: String = view::ask_for_url();
                if Url::parse(&temp_url).is_ok() {
                    save_backend_url(&temp_url);
                    backend_url = Some(temp_url);
                    break; // Verlasse die Eingabe-Schleife
                } else {
                    view::error_url_unavailable();
                }
            }
        }

        // Prüfe die Erreichbarkeit der URL
        if let Some(ref url) = backend_url {
            if api::check_health(url) {
                return url.clone(); // URL ist erreichbar, Rückgabe
            } else {
                view::error_url_unreachable(&Some(url.clone()));
                backend_url = None; // Zurücksetzen, um neue Eingabe zu erzwingen
            }
        }
    }
}

//Gibt URL zurück, falls sie lokal vorhanden ist. Ansonsten None
fn is_url_in_storage() -> Option<String> {
    // Hol das Projektverzeichnis
    if let Some(proj_dirs) = ProjectDirs::from("dev", "passhub", "passhub") {
        let config_dir = proj_dirs.config_dir();
        let config_file = config_dir.join("config.txt");

        // Falls die Datei existiert, lese den Inhalt
        if config_file.exists() {
            let mut file = fs::File::open(&config_file).expect("Fehler beim Öffnen der Datei");
            let mut url = String::new();
            file.read_to_string(&mut url)
                .expect("Fehler beim Lesen der Datei");

            // Entferne Whitespaces und prüfe, ob eine URL vorhanden ist
            let trimmed_url = url.trim();
            if !trimmed_url.is_empty() {
                return Some(trimmed_url.to_string());
            }
        }
    }
    None
}

//Erstellt Datei zum abspeichern der URL
fn create_file() {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "passhub", "passhub") {
        let config_dir = proj_dirs.config_dir();
        let config_file = config_dir.join("config.txt");

        // Erstelle das Konfigurationsverzeichnis, falls es nicht existiert
        fs::create_dir_all(config_dir).expect("Fehler beim Erstellen des Konfigurationsordners");

        // Erstelle die Datei
        fs::File::create(&config_file).expect("Fehler beim Erstellen der Datei");
    }
}

//Speichert die URL in der Datei, überschreibt URL falls vorhanden
fn save_backend_url(url: &str) {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "passhub", "passhub") {
        let config_dir = proj_dirs.config_dir();
        let config_file = config_dir.join("config.txt");

        // Schreibe die URL in die Datei
        let mut file = fs::File::create(&config_file).expect("Fehler beim Erstellen der Datei");
        file.write_all(url.as_bytes())
            .expect("Fehler beim Schreiben in die Datei");
    }
}
