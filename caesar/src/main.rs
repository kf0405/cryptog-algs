use caesar::*;
fn main() {
    println!("{:?}", encrypt("test_file.txt").unwrap());
    println!("{:?}", decrypt("test_file_enc.txt").unwrap());
}
