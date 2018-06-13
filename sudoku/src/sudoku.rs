extern crate rand;
use self::rand::{Rng};

use std::iter::{repeat,Iterator};

pub fn rand<R>(size: usize, level: usize, rng: &mut R) -> Vec<i32>
    where R: Rng
{
    let (nsize, ssize, fsize) = (size*size, size, size*size*size*size);
    let nums: Vec<i32> = (1..=(nsize as i32)).collect();
    let mut working = Vec::with_capacity(fsize);
    // initialize working table with random set of possible values.
    (0..fsize).for_each(|_| {
        let mut values = nums.clone();
        rng.shuffle(&mut values);
        working.push(values);
    });

    let mut solver = Solver{
        nsize, ssize, fsize,
        counts_rowreduce: 0, counts_colreduce: 0, counts_gridreduce: 0,
        counts_fixhidden: 0,
    };
    if let Some(result) = solver.roll(0, &working) {
        if let Some(mut table) = solver.verify(result) {
            let skip = fsize-level;
            return to_problem(&table, fsize, skip, rng)
        }
    }
    panic!("impossible situation")

}

pub fn problems<R>(size: usize, level: usize, rng: R) -> ProblemIterator<R>
    where R: Rng
{
    ProblemIterator{ size, rng, level }
}

pub fn solve(problem: &Vec<i32>) -> Option<Vec<i32>> {
    let (nsize, ssize, fsize) = fsize_to_tuple(problem.len())?;

    let nums: Vec<i32> = (1..=(nsize as i32)).collect();
    let mut working: Vec<Vec<i32>> = repeat(nums).take(fsize).collect();
    (0..fsize).filter(|&off| problem[off] > 0)
              .for_each(|off| { working[off] = vec![problem[off]] });

    let mut solver = Solver{
        nsize, ssize, fsize,
        counts_rowreduce: 0, counts_colreduce: 0, counts_gridreduce: 0,
        counts_fixhidden: 0,
    };

    if let Some(result) = solver.roll(0, &working) {
        return solver.verify(result)
    }
    None
}

pub fn solve_str(s: &str) -> Option<Vec<i32>> {
    let mut problem: Vec<i32> = Vec::new();
    for item in s.trim_matches(|p| p == '[' || p == ']').split(",") {
        let val = item.parse();
        if let Err(_err) = val {
            return None
        }
        problem.push(val.unwrap());
    }
    solve(&problem)
}


pub fn pretty_print(table: &Vec<i32>) {
    if let Some((nsize, _ssize, _fsize)) = fsize_to_tuple(table.len()) {
        for i in 0..nsize {
            print!("    ");
            for j in 0..nsize {
                print!("{} ", table[i*nsize+j]);
            }
            println!("");
        }
    }
}

fn to_problem<R>(table: &Vec<i32>, fsize: usize, skip: usize, rng: &mut R)
    -> Vec<i32>
    where R: Rng
{
    let mut count = skip;
    let mut problem = table.clone();
    while count > 0 {
        let off = (rng.next_u32() as usize) % fsize;
        if problem[off] > 0 {
            problem[off] = 0;
            count = count - 1;
        }
    }
    problem
}

fn fsize_to_tuple(fsize: usize) -> Option<(usize, usize, usize)> {
    let ssize = (fsize as f64).sqrt().sqrt() as usize;
    if fsize != (ssize*ssize*ssize*ssize) {
        return None
    }
    let nsize = ssize * ssize;
    Some((nsize, ssize, fsize))
}

pub struct ProblemIterator<R> where R: Rng {
    size: usize,
    rng: R,
    level: usize,
}

impl<R> Iterator for ProblemIterator<R> where R: rand::Rng {
    type Item=Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(rand(self.size, self.level, &mut self.rng))
    }
}

#[derive(Copy,Clone)]
struct Solver {
    nsize: usize,
    ssize: usize,
    fsize: usize,
    counts_rowreduce: usize,
    counts_colreduce: usize,
    counts_gridreduce: usize,
    counts_fixhidden: usize,
}

impl Solver {
    fn roll(
        &mut self, off: usize, table: &Vec<Vec<i32>>) -> Option<Vec<Vec<i32>>>
    {
        let mut ntable = table.clone();

        if off >= ntable.len() {
            return Some(ntable)

        } else if ntable[off].len() == 1 {
            return self.roll(off+1, &ntable)
        }

        ntable = self.naked_single(ntable)?;
        ntable = self.hidden_single(ntable)?;

        for num in ntable[off].clone() {
            ntable[off] = vec![num];
            if let Some(t) = self.roll(off+1, &ntable) {
                return Some(t)
            }
        }
        None
    }

    fn naked_single(
        &mut self, mut table: Vec<Vec<i32>>) -> Option<Vec<Vec<i32>>>
    {
        for off in 0..self.fsize {
            if table[off].len() == 1 {
                table = self.reduce_row(table, off)?;
                table = self.reduce_col(table, off)?;
                table = self.reduce_grid(table, off)?;
            }
        }
        Some(table)
    }

    fn hidden_single(
        &mut self, mut table: Vec<Vec<i32>>) -> Option<Vec<Vec<i32>>>
    {
        let mut items: Vec<i32> = repeat(0).take(self.nsize).collect();

        // row-wise
        for irow in 0..self.nsize {
            for icol in 0..self.nsize {
                for &num in &table[irow*self.nsize + icol] {
                    items[(num as usize) - 1] += 1;
                }
            }
            for (num, &count) in items.iter().enumerate() {
                if count != 1 { continue }
                let num = (num as i32) + 1;
                for icol in 0..self.nsize {
                    if table[irow*self.nsize + icol].contains(&num) {
                        table[irow*self.nsize + icol] = vec![num];
                        self.counts_fixhidden += 1;
                        break
                    }
                }
            }
        }

        // col-wise
        let mut items: Vec<i32> = repeat(0).take(self.nsize).collect();
        for icol in 0..self.nsize {
            for irow in 0..self.nsize {
                for &num in &table[irow*self.nsize + icol] {
                    items[(num as usize) - 1] += 1;
                }
            }
            for (num, &count) in items.iter().enumerate() {
                if count != 1 { continue }
                let num = (num as i32) + 1;
                for irow in 0..self.nsize {
                    if table[irow*self.nsize + icol].contains(&num) {
                        table[irow*self.nsize + icol] = vec![num];
                        self.counts_fixhidden += 1;
                        break
                    }
                }
            }
        }

        // grid-wise
        let mut items: Vec<i32> = repeat(0).take(self.nsize).collect();
        for grid in 0..self.nsize {
            for (irow, icol) in self.grid_points(grid) {
                for &num in &table[irow*self.nsize + icol] {
                    items[(num as usize) - 1] += 1;
                }
            }
            for (num, &count) in items.iter().enumerate() {
                if count != 1 { continue }
                let num = (num as i32) + 1;
                for (irow, icol) in self.grid_points(grid) {
                    if table[irow*self.nsize + icol].contains(&num) {
                        table[irow*self.nsize + icol] = vec![num];
                        self.counts_fixhidden += 1;
                        break
                    }
                }
            }
        }
        Some(table)
    }

    fn reduce_row(
        &mut self, mut table: Vec<Vec<i32>>, off: usize)
        -> Option<Vec<Vec<i32>>>
    {
        let (row, col) = self.off2point(off);
        let num = table[row * self.nsize + col][0];
        for icol in 0..self.nsize {
            if icol != col {
                let ns = self.filter_cell(&table[row*self.nsize+icol], num)?;
                let n = table[row*self.nsize + icol].len() - ns.len();
                self.counts_rowreduce += n;
                table[row*self.nsize + icol] = ns;
            }
        }
        Some(table)
    }

    fn reduce_col(
        &mut self, mut table: Vec<Vec<i32>>, off: usize)
        -> Option<Vec<Vec<i32>>>
    {
        let (row, col) = self.off2point(off);
        let num = table[row * self.nsize + col][0];
        for irow in 0..self.nsize {
            if irow != row {
                let ns = self.filter_cell(&table[irow*self.nsize+col], num)?;
                let n = table[irow*self.nsize + col].len() - ns.len();
                self.counts_colreduce += n;
                table[irow*self.nsize + col] = ns;
            }
        }
        Some(table)
    }

    fn reduce_grid(
        &mut self, mut table: Vec<Vec<i32>>, off: usize)
        -> Option<Vec<Vec<i32>>>
    {
        let (row, col) = self.off2point(off);
        let num = table[row * self.nsize + col][0];
        let grid = self.point2grid((row, col));
        for (irow, icol) in self.grid_points(grid) {
            if icol != col && irow != row {
                let ns = self.filter_cell(&table[irow*self.nsize+icol], num)?;
                let n = table[irow*self.nsize + icol].len() - ns.len();
                self.counts_gridreduce += n;
                table[irow*self.nsize + icol] = ns;
            }
        }
        Some(table)
    }

    fn filter_cell(&self, nums: &Vec<i32>, num: i32) -> Option<Vec<i32>> {
        let ns: Vec<i32> = nums
            .iter()
            .filter(|&&x| x != num).map(|&x| x).collect();
        match ns.len() {
            0 => None,
            _ => Some(ns),
        }
    }

    fn grid_points(&self, grid: usize) -> impl Iterator<Item=(usize, usize)> {
        let (nsize, ssize) = (self.nsize, self.ssize);
        (0..nsize).map(move |i| {
            let row = ((grid / ssize) * ssize) + (i/ssize);
            let col = ((grid % ssize) * ssize) + (i%ssize);
            (row, col)
        })
    }

    pub fn verify(&self, table: Vec<Vec<i32>>) -> Option<Vec<i32>> {
        let (nsize, ssize, _fsize) = (self.nsize, self.ssize, self.fsize);

        let mut item: Vec<usize> = (1..=nsize).map(|_| 0).collect();
        if table.iter().any(|cell| cell.len() > 1 ) {
            return None
        }

        let nsize = nsize as usize;
        for row in 0..nsize {
            (0..nsize).for_each(|col| {
                item[table[row*nsize + col][0] as usize -1] += 1;
            });
            if item.iter().any(|&x| x != row + 1) {
                return None
            }
        }
        for col in 0..nsize {
            (0..nsize).for_each(|row| {
                item[table[row*nsize + col][0] as usize-1] += 1;
            });
            if item.iter().any(|&x| x != nsize + col + 1) {
                return None
            }
        }
        for grid in 0..nsize {
            (0..nsize).for_each(|i| {
                let row = ((grid / ssize) * ssize) + (i/ssize);
                let col = ((grid % ssize) * ssize) + (i%ssize);
                item[table[row*nsize + col][0] as usize - 1] += 1;
            });
            if item.iter().any(|&x| x != (nsize*2) + grid + 1) {
                return None
            }
        }
        Some(table.iter().map(|cell| cell[0]).collect())
    }

    #[inline]
    fn off2point(&self, off: usize) -> (usize, usize) {
        (off/self.nsize, off%self.nsize) // (y, x)
    }

    #[inline]
    fn point2grid(&self, (y, x): (usize, usize)) -> usize {
        ((y/self.ssize) * self.ssize) + (x/self.ssize)
    }
}
