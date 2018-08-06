#[macro_use] extern crate nom;

named!(null<&str, &str>, tag!("null"));
named!(tuple<&str, f64 >,
    ws!(alt!(
        null => { |_| 1.0 } |
        tag!("true") => { |_| 2.0 } |
        tag!("false") => { |_| 3.0 }
    ))
);

fn main() {
    assert_eq!(Ok(("null", 2.0)), tuple(" \t true null"));
    assert_eq!(Ok(("null", 2.0)), tuple(" \t truenull"));
    assert_eq!(Ok(("null", 2.0)), tuple(" \t true "));
}
