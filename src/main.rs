#[allow(unused_imports)]
use std::io::{self, Write};

#[derive(PartialEq)]
enum BuiltIn {
    Exit,
    Echo,
    Type,
    Unknown,
}

fn eval_cmd(command: &str) -> BuiltIn {
    match command {
        "exit" => BuiltIn::Exit,
        "echo" => BuiltIn::Echo,
        "type" => BuiltIn::Type,
        _ => BuiltIn::Unknown,
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input: Vec<&str> = input.trim_start().split_whitespace().collect();

        match eval_cmd(input[0]) {
            BuiltIn::Exit => break,
            BuiltIn::Echo => println!("{}", input[1..].join(" ")),
            BuiltIn::Type => {
                let command = eval_cmd(input[1]);
                if command != BuiltIn::Unknown {
                    println!("{} is a shell builtin", input[1]);
                } else {
                    println!("{}: not found", input[1])
                }
            }
            _ => println!("{}: command not found", input[0]),
        }
    }
}
