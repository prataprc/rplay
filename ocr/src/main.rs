use std::time;

fn main() {
    let tessdata = "/usr/local/Cellar/tesseract/4.1.1/share/tessdata/";
    let mut lt = leptess::LepTess::new(Some(tessdata), "eng").unwrap();
    lt.set_image("./sample.jpeg");
    println!("{}", lt.get_utf8_text().unwrap());
}
