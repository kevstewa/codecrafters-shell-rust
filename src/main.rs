use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

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

struct PathSearcher {
    dirs: Vec<PathBuf>,
}

impl PathSearcher {
    fn from_env_path() -> Option<Self> {
        env::var_os("PATH").map(|paths| {
            let dirs = env::split_paths(&paths).collect();
            Self { dirs }
        })
    }

    fn find(&self, cmd: &str) -> Option<PathBuf> {
        if cmd.contains('/') {
            let p = Path::new(cmd);
            return Self::is_executable(&p).then(|| p.to_path_buf());
        }

        for dir in &self.dirs {
            let candidate = dir.join(cmd);
            if Self::is_executable(&candidate) {
                return Some(candidate);
            }
        }

        None
    }

    fn is_executable(path: &Path) -> bool {
        let metadata = match path.metadata() {
            Ok(m) if m.is_file() => m,
            _ => return false,
        };

        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;

            let mode = metadata.mode();
            mode & 0o111 != 0
        }

        #[cfg(not(unix))]
        {
            true
        }
    }
}

fn main() {
    let searcher = PathSearcher::from_env_path();

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
                } else if let Some(s) = &searcher {
                    if let Some(p) = s.find(input[1]) {
                        println!("{} is {}", input[1], p.display());
                    } else {
                        println!("{}: not found", input[1])
                    }
                } else {
                    println!("{}: not found", input[1])
                }
            }
            _ => println!("{}: command not found", input[0]),
        }
    }
}
