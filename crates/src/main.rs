use structopt::StructOpt;
use walkdir::WalkDir;

use std::path;

#[derive(Debug, StructOpt)]
#[structopt(name = "crates", about = "package analyser")]
struct Options {
    #[structopt(long = "path", parse(from_os_str))]
    path: path::PathBuf,
}

fn main() {
    let opt = Options::from_args();
    println!("working with path {:?}", opt.path);

    for e in WalkDir::new(&opt.path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        println!("{:?}", e.path());
    }
}
