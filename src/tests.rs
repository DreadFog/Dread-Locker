#[cfg(test)]
mod tests {
    use std::{process, fs::{self, OpenOptions}, io::{Write, Seek, SeekFrom}};

    use crate::encryption_functions;
    #[test]
    fn test_encrypt_file() {
        let file_list = fs::read_dir("/mnt/f/Programming/Rust/dread_locker/test_dir").unwrap_or_else(|err| {
            eprintln!("Error reading  content: {}", err);
            process::exit(1);
        });
        for file in file_list {
            encryption_functions::encrypt_file(file.unwrap().path(), 0);
            break;
        }
        
    }
    #[test]
    fn test_write() {
        let file_list = fs::read_dir("/mnt/f/Programming/Rust/dread_locker/test_dir").unwrap_or_else(|err| {
            eprintln!("Error reading  content: {}", err);
            process::exit(1);
        });
        for file in file_list {
            let mut file1 = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file.unwrap().path()).unwrap_or_else(|err| {
                eprintln!("Error opening : {}", err);
                process::exit(1);
            });
            file1.seek(SeekFrom::Start(0)).unwrap();
            file1.write_all("Bonjour c'est un test".as_bytes());
            break;
        }
        
    }
}