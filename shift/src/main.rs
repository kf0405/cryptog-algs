use shift::*;
fn main() {
    println!("{:?}", encrypt("test_file.txt", 5 as u8).unwrap());
    println!("{:?}", decrypt("test_file_enc.txt", 5 as u8).unwrap());
}
