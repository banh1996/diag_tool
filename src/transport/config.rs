use serde::{Deserialize, Serialize};
use std::sync::RwLock;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub ethernet: Ethernet,
    pub doip: Doip,
    pub parameter: Parameters,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ethernet {
    pub interface: String,
    pub local_ipv4: Option<String>,
    pub local_ipv6: Option<String>,
    pub remote_ip: String,
    pub remote_port: String,
    pub role: String,
    pub vendor: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Doip {
    pub version: u8,
    pub inverse_version: u8,
    pub tester_addr: u16,
    pub ecu_addr: u16,
    pub sga_addr: u16,
    pub activation_code: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Parameters {
    pub vin: String,
    pub tester_present: bool,
    pub tester_present_interval: String,
}

lazy_static::lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new(Config {
        ethernet: Ethernet {
            interface: String::new(),
            local_ipv4: None,
            local_ipv6: None,
            remote_ip: String::new(),
            remote_port: String::new(),
            role: String::new(),
            vendor: String::new(),
        },
        doip: Doip {
            version: 0,
            inverse_version: 0,
            tester_addr: 0,
            ecu_addr: 0,
            sga_addr: 0,
            activation_code: 0,
        },
        parameter: Parameters{
            vin: String::new(),
            tester_present: false,
            tester_present_interval: String::new(),
        },
    });
}