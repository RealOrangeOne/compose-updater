use std::collections::HashSet;
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub struct ComposeProject {
    compose_file: PathBuf,
}

impl ComposeProject {
    pub fn new(compose_file: &PathBuf) -> ComposeProject {
        ComposeProject {
            compose_file: compose_file.to_owned(),
        }
    }

    fn working_directory(&self) -> &Path {
        self.compose_file
            .parent()
            .expect("Failed to get parent of compose file")
    }

    pub fn pull(&self) -> bool {
        Command::new("docker-compose")
            .current_dir(self.working_directory())
            .args(&["-f", &self.compose_file.to_string_lossy()])
            .arg("pull")
            .status()
            .is_ok()
    }

    pub fn get_images(&self) -> HashSet<String> {
        let output = Command::new("docker-compose")
            .current_dir(self.working_directory())
            .args(&["-f", &self.compose_file.to_string_lossy()])
            .args(&["images", "-q"])
            .output()
            .expect("Failed to get images");
        let stdout = String::from_utf8(output.stdout).expect("Failed to parse output");
        stdout.trim().split('\n').map(String::from).collect()
    }

    pub fn down(&self) -> bool {
        match Command::new("docker-compose")
            .stdout(Stdio::null())
            .current_dir(self.working_directory())
            .args(&["-f", &self.compose_file.to_string_lossy()])
            .arg("down")
            .status()
        {
            Ok(s) => s.success(),
            Err(_) => false,
        }
    }

    pub fn up(&self) -> bool {
        match Command::new("docker-compose")
            .stdout(Stdio::null())
            .current_dir(self.working_directory())
            .args(&["-f", &self.compose_file.to_string_lossy()])
            .args(&["up", "-d"])
            .status()
        {
            Ok(s) => s.success(),
            Err(_) => false,
        }
    }
}

impl fmt::Display for ComposeProject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.compose_file.display())
    }
}
