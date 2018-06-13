#![cfg(test)]

use rand::{XorShiftRng,SeedableRng,Rng};
use std::time::{UNIX_EPOCH};

#[test]
fn xorshiftrng_distribution() {
    let seed = UNIX_EPOCH.elapsed().unwrap().subsec_nanos();
    let mut r = XorShiftRng::from_seed([seed, seed+1, seed+2, seed+3]);
    let mut distribution = [0; 256];
    for (i, n) in r.gen_iter::<u8>().enumerate() {
        distribution[(n as usize)] += 1;
        if i > 256 { break }
    }
    let zeros = distribution.iter().fold(
        0, |acc, &x| if x == 0 {acc+1} else {acc},
    );
    println!("{} out of 256 numbers where not generated", zeros);
}
