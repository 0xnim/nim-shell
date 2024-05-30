#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        // put into into struct
        let command = Command {
            // name is the first word
            name: input.trim().split_whitespace().next().unwrap().to_string(),
            // args are the rest of the words
            args: input
                .trim()
                .split_whitespace()
                .skip(1)
                .map(|s| s.to_string())
                .collect(),
        };
        if check_command(&command) {
            run_command(&command);
        } else {
            println!("{}: command not found", input.trim());
        }
    }
}

fn run_command(command: &Command) {
    match command.name.as_str() {
        "exit" => std::process::exit(0),
        "echo" => println!("{}", command.args.join(" ")),
        "type" => {
            if check_builtin(command.args[0].as_str()) {
                println!("{} is a shell builtin", command.args[0]);
            } else {
                println!("{} not found", command.args[0]);
            }
        }
        _ => {}
    }
}

fn check_builtin(input: &str) -> bool {
    match input.trim() {
        "exit" => true,
        "echo" => true,
        "type" => true,
        _ => false,
    }
}

fn check_command(command: &Command) -> bool {
    check_builtin(command.name.as_str())
}

// struct for command
struct Command {
    name: String,
    args: Vec<String>,
}
