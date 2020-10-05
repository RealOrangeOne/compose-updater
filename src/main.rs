#[macro_use]
extern crate log;
use glob::glob;
use std::path::PathBuf;
use std::process::exit;
use structopt::StructOpt;

mod compose;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    #[structopt(name = "files")]
    files: Vec<String>,

    #[structopt(short, long)]
    verbose: bool,
}

fn get_files(files: &[String]) -> Option<Vec<PathBuf>> {
    let mut all_files = Vec::new();
    for file in files {
        for path in glob(&file).ok()?.filter_map(Result::ok) {
            if path.is_file() {
                all_files.push(path);
            }
        }
    }

    Some(all_files)
}

fn main() {
    let opts = Opt::from_args();
    env_logger::builder()
        .format_timestamp(None)
        .filter_level(if opts.verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .format_module_path(false)
        .init();

    if opts.files.is_empty() {
        error!("Must specify some files");
        exit(1);
    }

    debug!("Searching for files...");
    let compose_files = match get_files(&opts.files) {
        Some(f) => f,
        None => exit(1),
    };

    let compose_projects: Vec<compose::ComposeProject> = compose_files
        .iter()
        .map(compose::ComposeProject::new)
        .collect();

    info!("Found {} projects", compose_projects.len());
}
