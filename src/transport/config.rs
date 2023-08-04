use serde::{Deserialize, Serialize};
use std::sync::RwLock;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub ethernet: Ethernet,
    pub doip: Doip,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ethernet {
    pub interface: String,
    pub local_ipv4: Option<String>,
    pub local_ipv6: Option<String>,
    pub remote_ip: String,
    pub remote_port: String,
    pub role: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Doip {
    pub version: u8,
    pub inverse_version: u8,
    pub tester_addr: u16,
    pub ecu_addr: u16,
    pub activation_code: u8,
}


lazy_static::lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new(Config {
    //pub static ref CONFIG: Config = Config {
        ethernet: Ethernet {
            interface: String::new(),
            local_ipv4: None,
            local_ipv6: None,
            remote_ip: String::new(),
            remote_port: String::new(),
            role: String::new(),
        },
        doip: Doip {
            //version: String::new(),
            version: 0,
            inverse_version: 0,
            tester_addr: 0,
            ecu_addr: 0,
            activation_code: 0,
        },
    });
}