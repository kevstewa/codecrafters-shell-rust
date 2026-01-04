use std::io::{self, Write};
use std::process::Command;

mod path_searcher;
use crate::path_searcher::{BuiltIn, CommandEvaluator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let evaluator = CommandEvaluator::from_env_path()?;

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input: Vec<&str> = input.split_whitespace().collect();

        match evaluator.eval_cmd(input[0]) {
            BuiltIn::Exit => break,
            BuiltIn::Echo => println!("{}", input[1..].join(" ")),
            BuiltIn::Type => {
                let command = evaluator.eval_cmd(input[1]);

                if let BuiltIn::Unknown = command {
                    if let Some(s) = &evaluator {
                        if let Some(p) = s.find(input[1]) {
                            println!("{} is {}", input[1], p.display());
                        } else {
                            println!("{}: not found", input[1])
                        }
                    } else {
                        println!("{}: not found", input[1])
                    }
                } else {
                    println!("{} is a shell builtin", input[1])
                }
            }
            BuiltIn::Exec(cmd) => {
                if let Ok(output) = Command::new(&cmd).args(&input[1..]).output() {
                    io::stdout().write_all(&output.stdout).unwrap();
                    io::stdout().flush().unwrap();
                } else {
                    println!("{}: command not found", &cmd);
                }
            }
            BuiltIn::Pwd => {
                println!("{}", cwd.display());
            }
            BuiltIn::Unknown => println!("{}: command not found", &input[0]),
        }
    }

    Ok(())
}
