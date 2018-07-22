fn main() {
    if is_x86_feature_detected!("aes") {
        println!("aes");
    }
}
