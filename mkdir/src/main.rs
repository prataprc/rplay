extern crate getopts;

mod error;
mod args;

use std::env;
use std::fs;
use std::path::Path;
use error::Error;
use args::{parse_args, CmdOptions};


//fn parsemode(mode: &String) -> Result(u32, &str) {
//    let mode = matches.opt_str("m").unwrap_or("")
//
//    if mode == "" { return Ok(0) } // leave it to default permission.
//
//    if mode[0] == '0' {
//        modeparse1(mode)
//    } else if mode.len() == 9 {
//        modeparse2(mode)
//    } else if mode.len() == 5 {
//        modeparse3(mode)
//    } else {
//        Err("invalid mode")
//    }
//}
//
//// parse 0777 format
//fn modeparse1(mode &String) -> Result<u32, &str> {
//    mode.insert(1, 'o');
//    mode.parse().or(Err("invalid mode"))
//}
//
//// parse rwxr-xr-x format
//fn modeparse2(mode &String) -> Result<u32, &str> {
//    if mode.all(|ch| ch == 'r' || ch == 'w' || ch == 'x' || ch == '-') {
//        mode.reverse()
//            .enumerate()
//            .filter(|(i, ch)| { ch != '-' })
//            .fold(m, |m, (i, _)| { m | (1<<i) })
//    } else {
//        Err("invalid mode")
//    }
//}
//
//// a=rwx
//fn modeparse3(mode &String) -> Result<u32, &str> {
//    match  mode[0] {
//        'a' => modeparse
//        'u' =>
//        'g' =>
//        'o' =>
//    }
//        Err("invalid mode")
//    } else if mode[1] != '=' {
//        Err("invalid mode")
//    } else mode[2..].all(|ch| ch == 'r' || ch == 'w' || ch == '
//    }
//}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let opts = parse_args(&args[0], &args[1..]);

    for path in &opts.paths {
        if let Ok(md) = fs::metadata(path) {
            if opts.verbose && md.is_dir() {
                println!("{}: Directory exists {:?}", opts.progname, path);
                continue
            }
        }

        if opts.recurse {
            create_recurse(&opts, path.as_ref())?;
        } else {
            create_dir(&opts, path.as_ref())?;
        }
    }
    Ok(())
}

fn create_recurse(opts: &CmdOptions, path: &Path) -> Result<(), Error> {
    use std::path::Component::{RootDir, CurDir, ParentDir, Prefix, Normal};
    use std::path::PathBuf;

    let mut d = PathBuf::new();
    for comp in path.components() {
        match comp {
            RootDir | CurDir | ParentDir => d.push(comp),
            Normal(name) => { d.push(name); create_dir(opts, d.as_ref())?; },
            Prefix(_) => panic!("not supported"),
        }
    }
    Ok(())
}

fn create_dir(opts: &CmdOptions, path: &Path) -> Result<(), Error> {
    if let Ok(md) = fs::metadata(path) {
        if md.is_dir() {
            return Ok(())
        }
    }

    fs::create_dir_all(path)?;
    if opts.verbose {
        println!("{}: Created directory {:?}", opts.progname, path);
    }
    Ok(())
}
