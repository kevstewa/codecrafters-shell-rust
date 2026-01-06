use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

mod path_searcher;
use crate::path_searcher::{BuiltIn, CommandEvaluator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut evaluator = CommandEvaluator::from_env_path()?;

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

                match command {
                    BuiltIn::Exec(path) => println!("{} is {}", input[1], path.display()),
                    BuiltIn::Unknown => println!("{}: not found", input[1]),
                    _ => println!("{} is a shell builtin", input[1]),
                }
            }
            BuiltIn::Exec(cmd) => {
                if let Ok(output) = Command::new(cmd.file_name().unwrap_or_default())
                    .args(&input[1..])
                    .output()
                {
                    io::stdout().write_all(&output.stdout).unwrap();
                    io::stdout().flush().unwrap();
                } else {
                    println!("{}: command not found", &cmd.display());
                }
            }
            BuiltIn::Pwd => {
                println!("{}", evaluator.get_cwd().display());
            }
            BuiltIn::Cd => {
                let new_path = Path::new(input[1]);
                if new_path.is_dir() {
                    evaluator.set_cwd(new_path);
                } else {
                    println!("cd: {}: No such file or directory", input[1]);
                }
            }
            BuiltIn::Unknown => println!("{}: command not found", &input[0]),
        }
    }

    Ok(())
}
