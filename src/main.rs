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

    #[structopt(long)]
    force_cycle: bool,

    #[structopt(long)]
    force_pull: bool,
}

fn get_files(files: &[String]) -> Option<Vec<PathBuf>> {
    let mut all_files = Vec::new();
    for file in files {
        for path in glob(&file).ok()?.filter_map(Result::ok) {
            if path.is_file() {
                if let Ok(canonical_path) = path.canonicalize() {
                    all_files.push(canonical_path);
                }
            }
        }
    }

    Some(all_files)
}

fn do_update(compose_project: compose::ComposeProject, force_cycle: bool, force_pull: bool) {
    info!("Processing {}...", compose_project);
    let pre_images = compose_project.get_images();
    if pre_images.is_empty() && !force_pull {
        warn!("no running images, skipping");
        return;
    }

    compose_project.pull();

    let post_images = compose_project.get_images();

    if force_cycle || post_images != pre_images {
        info!("Changes detected - Cycling container");
        warn!("Stopping container");
        compose_project.down();
        warn!("Starting container");
        compose_project.up();
    } else {
        info!("No change to images");
    }
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

    for compose_project in compose_projects {
        do_update(compose_project, opts.force_cycle, opts.force_pull);
    }
}
