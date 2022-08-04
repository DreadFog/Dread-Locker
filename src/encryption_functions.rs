use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::process;
use std::fs::File;
pub fn encrypt_dir(path: &str) {
    let file_list = fs::read_dir("/mnt/f/Programming/Rust/dread_locker/test_dir").unwrap_or_else(|err| {
        eprintln!("Error reading the folder's content");
        process::exit(1);
    });
    for file in file_list {
        println!("Encrypting '{}'", file.as_ref().unwrap().path().display());
        encrypt_file(file.unwrap().path());
    }
}

fn encrypt_file(file_path: PathBuf) {
    let mut file = File::open(file_path.clone()).unwrap_or_else(|err| {
        eprintln!("Error opening {} : {}", file_path.display(), err);
        process::exit(1);
    });
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    println!("Content before encryption: {}", contents);
}