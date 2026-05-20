use caesar::*;
fn main() {
    encrypt("test_file.txt", 5 as u8);
    decrypt("test_file_enc.txt", 5 as u8);
}
