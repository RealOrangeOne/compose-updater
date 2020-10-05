use std::path::{Path, PathBuf};
use std::process::Command;

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
}
