use std::process;

use chrono::{self, naive::NaiveDate};
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
pub struct Opt {
    #[structopt(long = "from")]
    pub from: String,

    #[structopt(long = "days")]
    pub days: i64,

    #[structopt(short = "p")]
    pub principle: f64,

    #[structopt(short = "r")]
    pub rate: f64,

    #[structopt(short = "n", default_value = "3")]
    pub period: f64,
}

fn main() {
    let opts = Opt::from_args();
    if opts.period > 12_f64 {
        println!("compound period, in months, cannot be greater than 12");
    }
    let datefmt = "%Y-%m-%d";
    let from = match NaiveDate::parse_from_str(&opts.from, datefmt) {
        Ok(date) => date,
        Err(_) => {
            println!("invalid {}", opts.from);
            process::exit(1);
        }
    };

    let mut accrued: Vec<(NaiveDate, f64, f64, f64)> = vec![];
    let (p, n, r) = (opts.principle, opts.period, opts.rate / 12_f64);
    let mut intr = 0_f64;
    let n_days = n * 30.41_f64;

    let mut elapsed = n_days;
    let mut till = from + chrono::Duration::days(elapsed as i64);
    while (till - from).num_days() < opts.days {
        elapsed += n_days;
        intr += calculate_interest(p + intr, n, r);
        accrued.push((till, p + intr, intr, 0_f64));
        till = from + chrono::Duration::days(elapsed as i64);
    }
    let rem_days = (opts.days as f64) - (elapsed - n_days);
    if rem_days > 0_f64 {
        let i = calculate_interest(p + intr, n, r);
        intr += (i / n_days) * (rem_days as f64);
        till = from + chrono::Duration::days(opts.days as i64);
        accrued.push((till, p + intr, intr, 0_f64));
    }
    println!("for {} days @ {} interest", opts.days, opts.rate);
    render(opts.principle, accrued);
}

fn calculate_interest(p: f64, n: f64, r: f64) -> f64 {
    (p * n * r) / 100_f64
}

fn render(principle: f64, accrued: Vec<(NaiveDate, f64, f64, f64)>) {
    for (on, accrued, interest, tds) in accrued {
        println!(
            "{} {:10.2} {:10.2} {:10.2} {:10.2}",
            on, principle, accrued, interest, tds
        );
    }
}
