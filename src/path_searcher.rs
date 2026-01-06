use std::env;
use std::path::{Path, PathBuf};

#[derive(PartialEq)]
pub enum BuiltIn {
    Exit,
    Echo,
    Type,
    Exec(PathBuf),
    Pwd,
    Cd,
    Unknown,
}

pub struct CommandEvaluator {
    dirs: Vec<PathBuf>,
    cwd: PathBuf,
}

impl CommandEvaluator {
    pub fn from_env_path() -> std::io::Result<Self> {
        let dirs = env::var_os("PATH")
            .map(|paths| env::split_paths(&paths).collect())
            .unwrap_or_default();
        let cwd = env::current_dir()?;

        Ok(Self { dirs, cwd })
    }

    pub fn find(&self, cmd: &str) -> Option<PathBuf> {
        if cmd.contains('/') {
            let p = Path::new(cmd);
            return Self::is_executable(p).then(|| p.to_path_buf());
        }

        for dir in &self.dirs {
            let candidate = dir.join(cmd);
            if Self::is_executable(&candidate) {
                return Some(candidate);
            }
        }

        None
    }

    pub fn eval_cmd(&self, command: &str) -> BuiltIn {
        match command {
            "exit" => BuiltIn::Exit,
            "echo" => BuiltIn::Echo,
            "type" => BuiltIn::Type,
            "pwd" => BuiltIn::Pwd,
            "cd" => BuiltIn::Cd,
            _ => {
                if let Some(p) = self.find(command) {
                    BuiltIn::Exec(p)
                } else {
                    BuiltIn::Unknown
                }
            }
        }
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

    pub fn get_cwd(&self) -> &PathBuf {
        &self.cwd
    }

    pub fn set_cwd(&mut self, path: &Path) {
        self.cwd = path.to_path_buf();
    }
}
