use getopts::{Matches,Options};
use error::Error;
use std::process;

#[derive(Debug)]
pub struct CmdOptions {
    pub progname: String,
    pub mode: String,
    pub recurse: bool,
    pub verbose: bool,
    pub paths: Vec<String>,
}

pub fn parse_args(progname: &str, args: &[String]) -> CmdOptions {
    let mut options = Options::new();

    match parse_result(progname, args, &mut options) {
        Ok(opts) => opts,
        Err(err) => {
            usage(&progname, &options, err);
            process::exit(1)
        }
    }
}

fn parse_result(progname: &str, args: &[String], options: &mut Options)
    -> Result<CmdOptions, Error>
{
    options.optflag(
        "p", "",
        "Create intermediate directories as required"
    );
    options.optflag(
        "v", "",
        "verbose when creating directories, listing them as they are created",
    );
    options.optopt(
        "m", "",
        "Set the file permission bits of the final created directory \
        to the specified mode",
        "0777 | a=rwx | rwxr-xr-x"
    );

    let matches = options.parse(args)?;

    let progname = progname.to_string();
    let verbose = matches.opt_present("v");
    let recurse = matches.opt_present("p");
    let mode = validate_mode(&matches)?;
    let paths = matches.free;
    Ok(CmdOptions{ progname, mode, recurse, verbose, paths })
}

fn usage(progname: &str, options: &Options, error: Error) {
    println!("Error: {}", error);
    println!("{}", options.short_usage(progname));
}

fn validate_mode(matches: &Matches) -> Result<String, Error> {
    let mode = matches.opt_str("m").unwrap_or("".to_string());

    if mode == "" {
        return Ok(mode)
    }

    let ps = ['r', 'w', 'x'];
    let is_rwx = |(i, &ch)| ch == ps[i%3] || ch == '-';
    let mode_chars: Vec<char> = mode.chars().collect();
    let ok = match mode_chars.len() {
        4 => mode_chars.iter().all(|ch| match ch { '0'..='7' => true, _ => false }),
        9 => mode_chars.iter().enumerate().all(is_rwx),
        5 if match mode_chars[0] { 'a' | 'u' | 'g' | 'o' => true, _ => false } => {
            mode_chars[1] == '=' && mode_chars[2..].iter().enumerate().all(is_rwx)
        },
        _ => false,
    };

    if ok == false { Ok(mode) } else { Err(Error::InvalidMode(mode.clone())) }
}
