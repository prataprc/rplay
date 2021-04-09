#![feature(test)]
extern crate test;

use rand::prelude::random;
use test::Bencher;

#[bench]
fn bench_1_kb(b: &mut Bencher) {
    let src: Vec<u8> = (0..1024).map(|x| x as u8).collect();
    let mut dst: Vec<u8> = Vec::with_capacity(src.len());
    dst.resize(dst.capacity(), 0);
    b.iter(|| dst.copy_from_slice(&src));
}

#[bench]
fn bench_1_mb(b: &mut Bencher) {
    let src: Vec<u8> = (0..(1024 * 1024)).map(|x| x as u8).collect();
    let mut dst: Vec<u8> = Vec::with_capacity(src.len());
    dst.resize(dst.capacity(), 0);
    b.iter(|| dst.copy_from_slice(&src));
}

#[bench]
fn bench_random_64(b: &mut Bencher) {
    b.iter(|| busy_loop(1000));
}

#[bench]
fn bench_atomicptr(b: &mut Bencher) {
    use std::{
        mem,
        sync::{
            atomic::{AtomicPtr, Ordering::SeqCst},
            mpsc, Arc,
        },
        thread,
    };

    let val1 = Arc::new(AtomicPtr::new(Box::leak(Box::new(10_u32))));
    let val2 = Arc::clone(&val1);

    let (tx, rx) = mpsc::channel::<bool>();
    thread::spawn(move || loop {
        match rx.try_recv() {
            Err(mpsc::TryRecvError::Disconnected) => break,
            _ => val2.store(Box::leak(Box::new(10)), SeqCst),
        }
    });
    let mut n = 0;
    b.iter(|| {
        // unsafe { *val1.load(SeqCst).as_mut().unwrap() = 10 };
        val1.load(SeqCst);
        n += 1;
    });
    println!("n:{}", n);
    mem::drop(tx);
}

#[bench]
fn bench_arc_clone(b: &mut Bencher) {
    use std::sync::{atomic::AtomicU64, Arc};

    b.iter(|| {
        let val = Arc::new(AtomicU64::new(0));
    })
}

fn busy_loop(count: usize) -> u64 {
    let acc: u64 = (0..count).map(|_| random::<u32>() as u64).sum();
    acc
}
