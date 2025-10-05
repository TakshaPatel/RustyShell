# RustyShell
RustyShell is a minimal, fun, and educational command-line shell written in Rust.  
It includes basic shell commands like listing files, changing directories, running programs, and more.  

> Note: This is primarily a learning project — it’s not meant to replace Bash, Zsh, or other production shells. But if you want to replace it, no one is going to stop you
> Another Note: the run cmd only works on UNIX-based systems

---

## Features

- `exit` — exits the shell
- `l` — list files in the current directory (directories are colorized)
- `show <file>` — display the contents of a file
- `goto <dir>` — change the current directory
- `pwd` — show the current directory
- `echo <text>` — print text to the console
- `run <program>` — run an executable in the current directory
- `clear` — clear the terminal screen
- `help` — display a help menu


## Usage

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/RustyShell.git
   cd RustyShell
2. Build the project with Cargo:
   ```bash
   cargo build --release
3. Run it
   ```bash
   cargo run

---
## Screenshot:

<img width="1195" height="958" alt="Screenshot 2025-10-05 at 7 29 25 PM" src="https://github.com/user-attachments/assets/d26a213f-1fa4-4407-a7a4-a0cb3a551631" />
