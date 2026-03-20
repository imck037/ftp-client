# Rust FTP Client

A fast and reliable **FTP client written in Rust** that can connect to any FTP server and includes a built-in authentication system for secure access.

---

## Features

* Connect to any standard FTP server
* Built-in authentication (username & password)
* Fast and memory-safe (powered by Rust)
* Upload and download files
* Directory navigation support
* Efficient connection handling
* Easy to extend and integrate

---

## ️ Requirements

* Rust (latest stable)

Check installation:

```bash
rustc --version
cargo --version
```

Install Rust if needed: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

---

## Installation

```bash
git clone https://github.com/imck037/ftp-client.git
cd ftp-client
cargo build
```

---

## ️ Usage

Run the client:

```bash
cargo run <ftp-server-address>
```

---

## Authentication

The client supports login using credentials:

```text
Enter the Username: <your_username>
Enter the Password: <your_username>
```

Example (CLI-style interaction):

```bash
> cargo run ftp.gnu.org
Enter the Username: anonymous
230 Login Succesful.
ftp> pasv
Entering Passive Mode.
ftp> list
150 Here come the directory listing..
.....
226 Directory Send OK.
ftp> retr file.txt
ftp> stor upload.txt
```

---

## Support Raw Commands
This client is support all raw command that an standard FTP server can handle
* `user <user>` - Send the username
* `pass <password>` – Authenticate user using password
* `list` – List directory contents
* `cwd <dir>` – Change directory
* `retr <file>` – Download file
* `stor <file>` – Upload file
* `quit` – Disconnect

---

## How It Works

1. Establishes a TCP connection to the FTP server
2. Performs authentication using credentials
3. Sends FTP commands (e.g., LIST, RETR, STOR)
4. Handles server responses
5. Transfers files over data connections

---

## Contributing

Contributions are welcome!

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Open a pull request

---

## License

This project is licensed under the GNU General Public License V3.

---