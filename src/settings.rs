pub struct Config{  // Will contain the required configuration for the ransomware to work 
                    // For example: the directory to encrypt
    pub cyphernumber: u8,// Or the cypher type to use
                    // Etc...
}
impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        let mut encryption_type: u8 = 0;
        let zero = "0".to_string();
        if args.len() < 3 || args[1] != "--key"{
            return Err("secure parameter '--key' missing as first argument");
        }
        if args[2].parse::<i32>().unwrap() != 1234 {
            return Err("Invalid execution key ");
        }
        let mut iterable_args = args.iter();
        if let Some(i) = iterable_args.position(|x| x.trim().to_lowercase() == "--encryption-type"){
            let encryption_type_str = args.get(i+1).unwrap_or_else(||{
                println!("Encryption type not found. Defaulting to 0");
                &zero
            });
            encryption_type = encryption_type_str.parse::<u8>().unwrap_or_else(|_|{
                println!("Encryption type not found. Defaulting to 0");
                0
            });
        }
        Ok(Config{
            cyphernumber : encryption_type,
        })
    }
}
