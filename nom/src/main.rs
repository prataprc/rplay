#[macro_use] extern crate nom;
extern crate regex;

use nom::types::CompleteStr as NS;

named!(null<&str, &str>, tag!("null"));
named!(tuple<&str, f64 >,
    ws!(alt!(
        null => { |_| 1.0 } |
        tag!("true") => { |_| 2.0 } |
        tag!("false") => { |_| 3.0 }
    ))
);

named!(identifier(NS) -> NS, ws!(re_match!(r"^[a-z]+")));

fn main() {
    assert_eq!(Ok(("null", 2.0)), tuple(" \t true null"));
    assert_eq!(Ok(("null", 2.0)), tuple(" \t truenull"));
    assert_eq!(Ok(("null", 2.0)), tuple(" \t true "));

    println!("{:?}", identifier(NS("  foo ]")));
    assert_eq!(
      tuple(" \t null abc fg"), Ok((&"fg"[..], 10.0))
    );
}
