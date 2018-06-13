extern crate getopts;

mod error;

use std::env;
use std::io::{self};
use getopts::{Options,Matches,ParsingStyle::{FloatingFrees}};
use error::Error;

fn argparse() -> Result<Matches, getopts::Fail> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.parsing_style(FloatingFrees);
    opts.parse(&args[1..])
}

fn main() -> Result<(), Error> {
    let matches = argparse()?;
    if matches.free.len() == 0 {
        echo()?;
    }

    Ok(())
}

fn echo() -> Result<(), io::Error> {
    let mut line = String::new();
    let stdin = io::stdin();
    while let Ok(_) = stdin.read_line(&mut line) {
        if line.len() == 0 {
            break;
        }
        println!("{}", line.trim());
        line.truncate(0);
    }

    Ok(())
}
