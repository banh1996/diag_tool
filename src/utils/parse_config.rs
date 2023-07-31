use crate::transport::config::{Config, Ethernet, Doip};
use crate::transport::config::CONFIG;

use log::{debug};
use std::fs::File;
use std::io::Read;


/*****************************************************************************************************************
 *  utils::parse function
 *  brief      Parse json file to get config parameters
 *  details    -
 *  \param[in]  config_filename  path to config json file
 *  \param[out] -
 *  \precondition -
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
pub fn parse(config_filename: String) {
    // Read the JSON file and update the `CONFIG` global variable
    let mut file = File::open(&config_filename).expect("Failed to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    let ethernet: Ethernet = {
        let config_data: serde_json::Value =
            serde_json::from_str(&contents).expect("Failed to parse config.json");
        let interface = config_data["ethernet"]["interface"]
            .as_str().expect("Invalid interface field")
            .to_owned();
        let local_ipv4 = config_data["ethernet"]["local_ipv4"]
            .as_str()
            .map(|s| s.to_owned());
        let local_ipv6 = config_data["ethernet"]["local_ipv6"]
            .as_str()
            .map(|s| s.to_owned());
        let remote_ip = config_data["ethernet"]["remote_ip"]
            .as_str().expect("Invalid remote_ip field")
            .to_owned();
        let remote_port = config_data["ethernet"]["remote_port"]
            .as_str().expect("Invalid remote_port field")
            .to_owned();
        let role = config_data["ethernet"]["role"]
            .as_str().expect("Invalid role field")
            .to_owned();
        Ethernet { interface, local_ipv4, local_ipv6, remote_ip, remote_port, role }
    };

    let doip: Doip = {
        let config_data: serde_json::Value =
            serde_json::from_str(&contents).expect("Failed to parse config.json");
        let version = config_data["doip"]["version"]
            .as_str().expect("Invalid version field")
            .to_owned();
        let inverse_version = config_data["doip"]["inverse_version"]
            .as_str().expect("Invalid inverse_version field")
            .to_owned();
        let tester_addr = config_data["doip"]["tester_addr"]
            .as_str().expect("Invalid tester_addr field")
            .to_owned();
        let ecu_addr = config_data["doip"]["ecu_addr"]
            .as_str().expect("Invalid ecu_addr field")
            .to_owned();
        let activation_code = config_data["doip"]["activation_code"]
            .as_str().expect("Invalid activation_code field")
            .to_owned();
        Doip { version, inverse_version, tester_addr, ecu_addr, activation_code }
    };

    // Update the CONFIG global variable
    *CONFIG.write().expect("Failed to acquire write lock") = Config {
        ethernet,
        doip,
    };
    debug!("Parsed configuration parameters successfully!");

    // You can update other fields of `CONFIG` as needed
}
