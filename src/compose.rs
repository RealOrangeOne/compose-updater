use std::path::PathBuf;

pub struct ComposeProject {
    compose_file: PathBuf,
}

impl ComposeProject {
    pub fn new(compose_file: &PathBuf) -> ComposeProject {
        ComposeProject {
            compose_file: compose_file.to_owned(),
        }
    }
}
