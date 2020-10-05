use std::io::Result;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

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

    fn execute_in_dir(&self, command: &str, arguments: &[&str]) -> Result<Output> {
        Command::new(command)
            .current_dir(self.working_directory())
            .args(arguments)
            .output()
    }

    fn docker_compose(&self, arguments: &[&str])-> Result<Output> {
        let mut compose_arguments = vec!["-f", self.compose_file.to_str().expect("Path parse failed")];
        compose_arguments.extend_from_slice(arguments);
        self.execute_in_dir("docker-compose", compose_arguments.as_slice())
    }
}
