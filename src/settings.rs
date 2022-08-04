pub struct Config{  // Will contain the required configuration for the ransomware to work 
                    // For example: the directory to encrypt
                    // Or the cypher type to use
                    // Etc...
}
impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 || args[1] != "--key"{
            return Err("secure parameter '--key' missing as first argument");
        }
        if args[2].parse::<i32>().unwrap() != 1234 {
            return Err("Invalid execution key ");
        }
        Ok(Config{})
    }
}
