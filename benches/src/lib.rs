#![feature(test)]
extern crate test;

use std::{collections::VecDeque, iter, rc::Rc};
use test::Bencher;

#[bench]
fn fast_copy1(b: &mut Bencher) {
    let mut arr = Vec::new();
    arr.resize(1000, 0);
    let data = Rc::new(arr);
    b.iter(|| {
        let mut ans = VecDeque::new();
        for x in data.as_ref().iter() {
            ans.push_back(x.clone());
        }
    });
}

#[bench]
fn fast_copy2(b: &mut Bencher) {
    let mut arr = Vec::new();
    arr.resize(1000, 0);
    let data = Rc::new(arr);
    b.iter(|| {
        let out: VecDeque<i32> = iter::FromIterator::from_iter(data.iter().cloned());
    });
}

#[bench]
fn fast_copy3(b: &mut Bencher) {
    let mut arr = Vec::new();
    arr.resize(1000, 0);
    let data = Rc::new(arr);
    b.iter(|| {
        let out: VecDeque<i32> = (*data).clone().into();
    });
}
