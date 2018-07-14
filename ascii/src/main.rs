#![feature(iterator_step_by)]

use std::fmt::Write;

fn main() {
    let mut rows = Vec::with_capacity(256 / 8);
    let n_row = rows.capacity();
    let mut s  = String::new();
    (0..n_row).for_each(|_| rows.push(String::with_capacity(128)));
    for i in (0_usize..=255) {
        let row = &mut rows[i % n_row];
        s.clear();
        write!(s, "{:?}", i as u8 as char);
        write!(row, "{:4} {:8} ", i, s).unwrap();
    }
    (0..n_row).for_each(|i| println!("{}", rows[i]) );
}
