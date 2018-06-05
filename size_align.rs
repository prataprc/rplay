#! /usr/bin/env rustr

#![feature(never_type)]

use std::mem::{size_of, align_of, size_of_val, align_of_val};

trait MyTrait {
}

impl MyTrait for i32 {
}

fn main() {
    let fnptr: fn() -> usize = size_of::<i8>;
    let mut x = 10;
    let constptr: *const i32  = &x;
    let mutptr: *mut i32  = &mut x;
    let reference = &x;
    let traitobj: &MyTrait = &x;

    println!("Rust has abstraction without hiding memory layout !!");
    println!("In that, size and alignment is the starting point");
    println!("primitive.u8          {:<4} {:<4}", size_of::<u8>(), align_of::<u8>());
    println!("primitive.u16 ....... {:<4} {:<4}", size_of::<u16>(), align_of::<u16>());
    println!("primitive.u32         {:<4} {:<4}", size_of::<u32>(), align_of::<u32>());
    println!("primitive.u64 ....... {:<4} {:<4}", size_of::<u64>(), align_of::<u64>());
    println!("primitive.u128        {:<4} {:<4}", size_of::<u128>(), align_of::<u128>());
    println!("primitive.bool ...... {:<4} {:<4}", size_of::<bool>(), align_of::<bool>());
    println!("primitive.char        {:<4} {:<4}", size_of::<char>(), align_of::<char>());
    println!("primitive.f32 ....... {:<4} {:<4}", size_of::<f32>(), align_of::<f32>());
    println!("primitive.f64         {:<4} {:<4}", size_of::<f64>(), align_of::<f64>());
    println!("primitive.isize ..... {:<4} {:<4}", size_of::<isize>(), align_of::<isize>());
    println!("primitive.usize       {:<4} {:<4}", size_of::<usize>(), align_of::<usize>());
    println!("primitive.never! .... {:<4} {:<4}", size_of::<!>(), align_of::<!>());
    println!("primitive.constptr    {:<4} {:<4}", size_of_val(&constptr), align_of_val(&constptr));
    println!("primitive.mutptr .... {:<4} {:<4}", size_of_val(&mutptr), align_of_val(&mutptr));
    println!("primitive.reference   {:<4} {:<4}", size_of_val(&reference), align_of_val(&reference));
    println!("primitive.funcptr ... {:<4} {:<4}", size_of_val(&fnptr), align_of_val(&fnptr));
    println!("primitive.traitobj    {:<4} {:<4}", size_of_val(&traitobj), align_of_val(&traitobj));
}
