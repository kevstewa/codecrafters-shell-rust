#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        if command.trim() == "exit" {
            break;
        } else if command.starts_with("echo ") {
            let output = command.splitn(2, ' ').nth(1).unwrap_or("").trim();
            println!("{}", output);
            continue;
        }

        println!("{}: command not found", command.trim())
    }
}
