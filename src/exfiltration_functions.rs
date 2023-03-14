use ping::ping;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
extern crate ping;
pub fn exfiltrate_infos(infos: HashMap<String, String>) {
   let r = try_connectivity();
}

fn try_connectivity() -> Result<(), ()> {
    let p = ping(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        Some(Duration::from_millis(1000)),
        None,
        None,
        None,
        None,
    ); // TODO: privileges required?
    println!("{:?}", p);
    Ok(())
}
