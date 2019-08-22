use std::time;

fn main() {
    let mut primes = Primes::new();
    let now = time::SystemTime::now();
    let ps: Vec<u32> = primes.take(10000).collect();
    println!("took {:?}", now.elapsed().unwrap());
    ps.iter().for_each(|p| println!("{}", p));
}

struct Primes {
    primes: Vec<u32>,
}

impl Primes {
    fn new() -> Primes {
        Primes {
            primes: Default::default(),
        }
    }
}

impl Iterator for Primes {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.primes.len() == 0 {
            self.primes.push(2);
            Some(2)
        } else {
            let n = *self.primes.last().unwrap() + 1;
            let mut iter = (n..).skip_while(|n| self.primes.iter().any(|p| (n % p) == 0));
            let p = iter.next().unwrap();
            self.primes.push(p);
            Some(p)
        }
    }
}
