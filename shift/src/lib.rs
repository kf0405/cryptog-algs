use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

pub fn encrypt(path: &str, key: u8) -> io::Result<()> {
    let plaintext = fs::read(path)?;
    let ciphertext: Vec<u8> = plaintext.iter().map(|&x| x.wrapping_add(key)).collect();
    write_to_new_path(path, "enc", &ciphertext)?;
    Ok(())
}

pub fn decrypt(path: &str, key: u8) -> io::Result<()> {
    let ciphertext = fs::read(path)?;
    let plaintext: Vec<u8> = ciphertext.iter().map(|&x| x.wrapping_sub(key)).collect();
    write_to_new_path(path, "dec", &plaintext)?;
    Ok(())
}

fn write_to_new_path(path: &str, ending: &str, data: &[u8]) -> io::Result<()> {
    let path = Path::new(path);
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("txt");

    let new_filename = format!("{}_{}.{}", stem, ending, extension);
    let mut new_path = path.to_path_buf();
    new_path.set_file_name(new_filename);
    let mut file = File::create(&new_path)?;
    file.write_all(data)?;
    println!("File created at: {:?}", new_path);
    Ok(())
}
