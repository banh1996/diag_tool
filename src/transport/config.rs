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
    pub version: String,
    pub inverse_version: String,
    pub tester_addr: String,
    pub ecu_addr: String,
    pub activation_code: String,
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
            version: String::new(),
            inverse_version: String::new(),
            tester_addr: String::new(),
            ecu_addr: String::new(),
            activation_code: String::new(),
        },
    });
}