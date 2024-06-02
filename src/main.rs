#[allow(unused_imports)]
use std::io::{self, Write};

use std::env;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");
    /*
        // get path
        let path = env::var("PATH").unwrap();
        // split path by :
        let paths: Vec<&str> = path.split(":").collect();
    */

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        // put into into struct
        // remove first 2 chars from the front if starts with '> '
        input = input.trim_start_matches("> ").to_string();

        // problem is that if the input is /usr/bin/ls it says not a command
        // so we need to check if the input is a path and if it is, we need to run it
        // if it is not a path, we need to run the command

        // split command and args at space
        let mut parts = input.trim().splitn(2, ' ');
        let command_name = parts.next().unwrap();
        let command_args = parts.next().unwrap_or("").trim();

        let command = Command {
            name: command_name.to_string(),
            args: command_args
                .split_whitespace()
                .map(|s| s.to_string())
                .collect(),
        };

        // if check command not Type::None Run command
        if Type::None != check_command(command.name.as_str()) {
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
        "type" => match check_command(command.args[0].as_str()) {
            Type::Builtin => println!("{} is a shell builtin", command.args[0]),
            Type::Path => println!(
                "{} is {}",
                command.args[0],
                check_path(command.args[0].as_str()).unwrap()
            ),
            Type::None => println!("{}: not found", command.args[0]),
        },
        "pwd" => println!("{}", std::env::current_dir().unwrap().display()),
        "cd" => {
            if command.args.len() == 0 {
                std::env::set_current_dir(std::env::var("HOME").unwrap()).unwrap();
            } else {
                std::env::set_current_dir(command.args[0].as_str()).unwrap();
            }
        }
        _ => {
            // check if it is a path to a file or executable
            // if it is, run it
            // if not, run the command

            let output = std::process::Command::new(&command.name)
                .args(&command.args)
                .output()
                .expect("failed to execute process");
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        }
    }
}

fn check_builtin(input: &str) -> bool {
    match input.trim() {
        "exit" => true,
        "echo" => true,
        "type" => true,
        "pwd" => true,
        "cd" => true,
        _ => false,
    }
}

fn check_command(input: &str) -> Type {
    // check builtin if not check path, return type
    if check_builtin(input) {
        return Type::Builtin;
    } else if check_path(input).is_some() {
        return Type::Path;
    } else {
        return Type::None;
    }
}

// check if command is in any of the paths and return the path or None
fn check_path(input: &str) -> Option<String> {
    let path = env::var("PATH").unwrap();
    let paths: Vec<&str> = path.split(":").collect();
    // add current directory to the path

    for p in paths {
        let full_path = format!("{}/{}", p, input);
        if std::path::Path::new(&full_path).exists() {
            return Some(full_path);
        }
    }

    if std::path::Path::new(input).exists() {
        return Some(input.to_string());
    }
    None
}

// struct for command
struct Command {
    name: String,
    args: Vec<String>,
}

#[derive(PartialEq)]
enum Type {
    Builtin,
    Path,
    None,
}
