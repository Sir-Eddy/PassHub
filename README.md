# PassHub CLI

PassHub ist eine benutzerfreundliche Kommandozeilenanwendung (CLI) für den Zugriff und die Verwaltung von Passwörtern im rsPass-Backend. Die CLI nutzt das Rust-Bibliothekspaket `ratatui`, um eine intuitive, terminalbasierte Benutzeroberfläche bereitzustellen.

## Funktionen

**Passwortverwaltung**  
Greifen Sie auf Ihre Passwörter zu, erstellen Sie neue Einträge und bearbeiten oder löschen Sie bestehende Einträge in Ihrem rsPass-Backend.

**Sichere Authentifizierung**  
Verwenden Sie JSON Web Tokens (JWT) für eine sichere Anmeldung und API-Kommunikation.

**Account-Erstellung**  
Erstellen Sie direkt über die CLI einen neuen Account. Ein bestehender Account ist nicht erforderlich.

**Intuitive Benutzeroberfläche**  
Profitieren Sie von einer terminalbasierten Oberfläche, die durch `ratatui` bereitgestellt wird, um eine einfache und interaktive Bedienung zu ermöglichen.

**Integration mit rsPass**  
Kommunizieren Sie nahtlos mit Ihrem rsPass-Backend, das über HTTPS gesichert ist.

**Passwortgenerator**  
Erstellen Sie sichere und zufällige Passwörter direkt in der CLI.

## Sicherheit

**Argon2id-Hashing**  
Passwörter werden sofort nach der Eingabe mit Argon2id gehashed. Das Klartextpasswort wird niemals gespeichert und direkt mit `zeroize()` entfernt.

**AES-256-GCM-Verschlüsselung**  
Alle Datenübertragungen erfolgen über AES-256-GCM.

**Strenge Passwort-Richtlinien**  
Das Masterpasswort muss strenge Sicherheitsrichtlinien erfüllen.

**Regelmäßiger Login**  
Eine erneute Anmeldung ist jede Stunde erforderlich, um die Sicherheit zu gewährleisten.

## Voraussetzungen

**Laufender rsPass-Backend-Server**  
Das rsPass-Backend muss installiert, konfiguriert und erreichbar sein. Eine Anleitung finden Sie im [rsPass-Backend-Repository](https://github.com/Letgamer/rsPass).

**Rust-Umgebung**  
Die CLI wurde mit Rust entwickelt. Stellen Sie sicher, dass Rust und Cargo installiert sind, um PassHub auszuführen.

## Installation

1. **Repository klonen**:  
   ```bash
   git clone https://github.com/Sir-Eddy/PassHub.git
   cd PassHub
   ```

2. **Abhängigkeiten installieren**:  
   ```bash
   cargo build --release
   ```

3. **Programm ausführen**:  
   ```bash
   ./target/release/passhub
   ```

## Konfiguration

1. **Backend-URL eingeben**:  
   Beim ersten Start der CLI werden Sie aufgefordert, die URL Ihres rsPass-Backends anzugeben.

2. **Anmeldung durchführen**:  
   Geben Sie Ihre Anmeldedaten ein, um einen JWT zu erhalten. Dieser wird sicher gespeichert.

## Nutzung

**Starten der CLI**  
Führen Sie das Programm mit folgendem Befehl aus:  
```bash
./passhub
```

**Passwort abrufen**  
Navigieren Sie durch Ihre gespeicherten Passwörter und kopieren Sie Einträge bei Bedarf.

**Neues Passwort hinzufügen**  
Erstellen Sie neue Einträge direkt über die Benutzeroberfläche.

**Passwort generieren**  
Verwenden Sie die integrierte Passwortgenerator-Funktion, um sichere Passwörter zu erstellen.

**Token-Erneuerung**  
PassHub erneuert JWTs automatisch, solange die CLI aktiv ist. Wenn ein Token abläuft, werden Sie zur erneuten Anmeldung aufgefordert.

## Support

Falls Sie Fragen oder Probleme haben, erstellen Sie bitte ein Issue im [GitHub-Repository](https://github.com/Sir-Eddy/PassHub/issues).
