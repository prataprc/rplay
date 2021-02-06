cfg_if::cfg_if! {
    if #[cfg(target_pointer_width = "32")] {
        fn foo() { println!("non-unix, 32-bit functionality") }
    } else if #[cfg(unix)] {
        fn foo() { println!("unix specific functionality") }
    } else {
        fn foo() { println!("fallback implementation") }
    }
}

fn main() {
    println!("Hello, world!");
}
