use glob::glob;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    #[structopt(name = "files")]
    files: Vec<String>,
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
    println!("{:?}", get_files(&opts.files));
}
