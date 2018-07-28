#[macro_use] extern crate nom;

named!(null<&str, &str>, tag!("null"));
named!(tuple<&str, f64 >,
    ws!(alt!(
        null => { |_| 30.0 } |
        tag!("fg") => { |_| 10.0 } |
        tag!("abc") => { |_| 20.0 }
    ))
);

fn main() {
    println!("10");
    assert_eq!(
      tuple(" \t null abc fg"), Ok((&"fg"[..], 10.0))
    );
}
