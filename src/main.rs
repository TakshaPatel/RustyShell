use std::env;
use std::io;
use std::io::Write;
use std::fs;
use std::process::Command;
use colored::Colorize;

fn cmd_list(_args: &str) {
    for entry in fs::read_dir(".").unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name().to_string_lossy().to_string();
        let colorized = if entry.file_type().unwrap().is_dir() {
            file_name.blue().bold()
        } else {
            file_name.white()
        };
        println!("{}", colorized);
    }
}

fn cmd_pwd(_args: &str) {
    match env::current_dir() {
        Ok(path) => {
            // If successful, print the path using the `display()` method.
            println!("The current directory is {}", path.display());
        }
        Err(e) => {
            // If there's an error, print the error message.
            eprintln!("Error: Could not get the current directory. {}", e);
        }
    }
}
fn cmd_exit(_args: &str) {
    println!("Ok bye bye");
    std::process::exit(0);
}

fn cmd_clear(_args: &str) {
    clearscreen::clear().expect("failed to clear screen");
}

fn cmd_help(_args: &str) {
    println!("Commands:\n\n exit    - Exits\n l    - list files\n show    - show contents of file\n goto    - move to a dir\n help    - show help menu\n echo    - print text\n run    - run a binary program\n clear    - clear the screen\n\n\n HOW TO MODIFY:\n1. Spend ~6 hours learning the rust programing language\n2. Try to edit this code\n3. Fail to edit this code\n4. Realize that this is useless\n5. End up with 48 hours of your life wasted.");
}

fn cmd_echo(args: &str) {
    println!("{}", args);
}

fn cmd_goto(args: &str) {
    if args.is_empty() {
        eprintln!("Usage: goto [dir]");
        return
    }
    if let Err(e) = std::env::set_current_dir(args) {
        eprintln!("Failed to change directory: {}", e);
    }
}


fn cmd_show(args: &str) {
    if args.is_empty() {
        eprintln!("Usage: show <file>");
        return;
    }
    match fs::read_to_string(args) {
        Ok(contents) => println!("{}", contents),
        Err(e) => eprintln!("Failed to read file: {}", e),
    }
}

fn cmd_run(args: &str) {
    if args.is_empty() {
        println!("Usage: run <program>");
        return;
    }
    let status = Command::new(format!("./{}", args))
        .status();

    match status {
        Ok(s) => {
            if !s.success() {
                println!("Program exited with status: {}", s);
            }
        }
        Err(e) => eprintln!("Failed to run program: {}", e),
    }
}

fn main() {
    clearscreen::clear().expect("failed to clear screen");
    let cwd = env::current_dir().unwrap();
    let path = cwd.display();
    println!("
Welcome to RustyShell, A very cool shell.

If you got here by mistake, panic!  Just type 'exit' and carry on.

Type 'help' for a list of commands.

If you want to customize the look/behavior, there is not a options page, Modify the code yourself.

Note: No arrow key support yet : /
    ");

    let cmds = vec!["exit", "l", "show", "goto", "help", "echo", "run", "pwd", "clear"];

    loop {
        print!("{}", format!("RustyShell [{}]> ", path).yellow().bold());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let (cmd, args) = input.split_once(char::is_whitespace).unwrap_or((input, ""));

        if !cmds.contains(&cmd.trim()) {
            eprintln!("Command not found: {}", cmd);
        }
        match cmd {
            "exit" => cmd_exit(args),
            "help" => cmd_help(args),
            "echo" => cmd_echo(args),
            "goto" => cmd_goto(args),
            "show" => cmd_show(args),
            "run" => cmd_run(args),
            "l" => cmd_list(args),
            "pwd" => cmd_pwd(&args),
            "clear" => cmd_clear(args),
            _ => println!("Type help for more information."),
        }
    }
}