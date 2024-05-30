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
    for p in paths {
        let full_path = format!("{}/{}", p, input);
        if std::path::Path::new(&full_path).exists() {
            return Some(full_path);
        }
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
