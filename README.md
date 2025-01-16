# PassHub CLI 🦀

![Commit Activity](https://img.shields.io/github/commit-activity/w/Sir-Eddy/PassHub)
![Language](https://img.shields.io/github/languages/top/Sir-Eddy/PassHub?color=blue)
[![Rust](https://github.com/Sir-Eddy/PassHub/actions/workflows/rust.yml/badge.svg)](https://github.com/Sir-Eddy/PassHub/actions/workflows/rust.yml)
![Repo Size](https://img.shields.io/github/repo-size/Sir-Eddy/PassHub)
![Open Source](https://img.shields.io/badge/Open%20Source-%E2%9D%A4-red)


PassHub is a user-friendly Command-Line Interface (CLI) application for accessing and managing passwords in the rsPass backend. The CLI uses the Rust library `ratatui` to provide an intuitive, terminal-based user interface.

## Features

**Password Management**  
Access your passwords, create new entries, and edit or delete existing ones in your rsPass backend.

**Account Creation**  
Create a new account directly through the CLI—no prior account is required.

**Intuitive User Interface**  
Leverages a terminal-based interface provided by `ratatui` for easy and interactive navigation.

**Integration with rsPass**  
Seamlessly communicate with your rsPass backend secured via HTTPS.

## Security

**Argon2id Hashing**  
Argon2id is a highly secure password hashing algorithm, resistant to GPU attacks and optimized for both memory-hardness and speed. Your master password is hashed immediately upon input and never stored in plaintext. The plaintext password is securely erased from memory using `zeroize()`.

**AES-256-GCM Encryption**  
AES-256-GCM is a state-of-the-art encryption standard that ensures data integrity and confidentiality. All communications with the backend are encrypted, protecting your sensitive information from eavesdropping.

**Strict Password Policies**  
The master password must meet stringent requirements to ensure high entropy and resistance to brute-force attacks.

**Regular Login**  
For enhanced security, reauthentication is required every hour. This reduces the window of opportunity for unauthorized access in case of session hijacking.

**JWT Authentication**  
JSON Web Tokens (JWT) are used to authenticate users and maintain secure communication with the backend. Tokens are stored only in memory during runtime to minimize their exposure to external threats.

## Requirements

**Running rsPass Backend Server**  
The rsPass backend must be installed, configured, and accessible. Refer to the [rsPass backend repository](https://github.com/Letgamer/rsPass) for setup instructions.

## Installation

1. **Download the CLI**  
   Visit the [Releases](https://github.com/Sir-Eddy/PassHub/releases) page and download the appropriate binary for your operating system:  
   - **Linux**: `passhub_for_rspass_linux`  
   - **Windows**: `passhub_for_rspass_windows.exe`

2. **Set Executable Permissions (Linux only)**  
   After downloading the Linux binary, make it executable by running:  
   ```bash
   chmod +x passhub_for_rspass_linux
   ```

3. **Run the Application**  
   Execute the downloaded binary:  
   - On Linux:  
     ```bash
     ./passhub_for_rspass_linux
     ```
   - On Windows:  
     ```cmd
     passhub_for_rspass_windows.exe
     ```

4. **First-Time Configuration**  
   - **Set Backend URL**: On the first launch, you will be prompted to enter the URL of your rsPass backend.  
   - **Login or Register**: Provide your credentials or create a new account to start using the CLI.

## Configuration

1. **Set the Backend URL**  
   On first launch, the CLI will prompt you to enter the URL of your rsPass backend.

2. **Login / Register**  
   Provide your credentials or create a new account to obtain a JWT, which will be stored in main memory on runtime.


## Support

If you have any questions or issues, please open an issue in the [GitHub Repository](https://github.com/Sir-Eddy/PassHub/issues).
