extern crate rand;
use rand::{XorShiftRng,SeedableRng};

extern crate getopts;
use getopts::{Options,ParsingStyle};

mod sudoku;

use std::env;
use std::process;
use std::time::{SystemTime,UNIX_EPOCH};
use std::fmt::{Debug};
use std::str::{FromStr};

struct CmdOptions {
    size: usize,
    seed: u32,
    easy: usize,
    count: u32,
}

fn try_usage_unwrap<T,E>(arg: Result<T,E>, options: &Options) -> T
    where E: Debug
{
    if let Err(err) = arg {
        println!("Error: {:?}", err);
        println!("{}", options.usage("^.^"));
        process::exit(1);
    }
    arg.unwrap()
}

fn argparse() -> Result<CmdOptions, <i32 as FromStr>::Err> {
    let args: Vec<String> = env::args().collect();

    let mut options = Options::new();
    options.parsing_style(ParsingStyle::FloatingFrees).long_only(true);

    options.optopt("", "size", "Sudoku subgrid size", "2,3,4");
    options.optopt("", "seed", "Seed for RNG", "INTEGER");
    options.optopt(
        "", "easy",
        "Sudoku puzzle with N visible items", "< size*size*size*size*"
    );
    options.optopt(
        "", "count", "Number of puzzles to generate and solve", "1,2,3,4"
    );

    let ms = try_usage_unwrap(options.parse(args), &options);

    let size = ms.opt_str("size").unwrap_or("3".to_string()).parse()?;
    let default = UNIX_EPOCH.elapsed().unwrap().subsec_nanos().to_string();
    let seed = ms.opt_str("seed").unwrap_or(default).parse()?;
    let default = ((60.0 / 81.0) * ((size*size*size*size) as f64)) as i32;
    let easy = ms.opt_str("easy").unwrap_or(default.to_string()).parse()?;
    let count = ms.opt_str("count").unwrap_or("1".to_string()).parse()?;

    Ok(CmdOptions{size, seed, easy, count})
}

fn main() -> Result<(), <i32 as FromStr>::Err> {
    let o = argparse()?;
    match o.count {
        1 => single(&o),
        _ => many(&o),
    };
    Ok(())
}

fn single(o: &CmdOptions) {
    println!("For size: {}, easy: {}, seed: {}\n", o.size, o.easy, o.seed);

    let seeds = [o.seed, o.seed+1, o.seed+2, o.seed+3];
    let mut rng = XorShiftRng::from_seed(seeds);

    let problem = sudoku::rand(o.size, o.easy, &mut rng);

    println!("Problem:");
    sudoku::pretty_print(&problem);
    println!("");

    let start = SystemTime::now();
    match sudoku::solve(&problem) {
        Some(solution) => {
            println!("Solution: (took {:?})", start.elapsed().unwrap());
            sudoku::pretty_print(&solution);
        },
        None => { println!("No solution"); },
    }
}

fn many(o: &CmdOptions) {
    let seeds = [o.seed, o.seed+1, o.seed+2, o.seed+3];
    let mut rng = XorShiftRng::from_seed(seeds);

    let (mut problems, mut solutions) = (0, 0);
    let mut iter = sudoku::problems(o.size, o.easy, &mut rng);
    while problems < o.count {
        let start = SystemTime::now();
        let problem = iter.next().unwrap();
        let problem_elapsed = start.elapsed().unwrap();

        let start = SystemTime::now();
        problems += 1;
        if let Some(_) = sudoku::solve(&problem) {
            solutions += 1;
        }
        let solution_elapsed = start.elapsed().unwrap();

        println!(
            "Took {:?} for problem, {:?} for solution",
            problem_elapsed,
            solution_elapsed
        );
    }
    println!("Total {} problems generated, and {} solved", problems, solutions);
}
