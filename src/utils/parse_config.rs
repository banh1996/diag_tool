use crate::transport::config::{Config, Ethernet, Doip, Parameters, CONFIG};
use std::io::{self, Read, Error, ErrorKind};
use log::debug;
use std::fs::File;

/*****************************************************************************************************************
 *  utils::parse_content function
 *  brief      Parse json file to get config parameters
 *  details    -
 *  \param[in]  content  config json file content
 *  \param[out] -
 *  \precondition -
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
pub fn parse_content(content: String) -> Result<(), io::Error> {
    let config_data: serde_json::Value =
        serde_json::from_str(&content).expect("Failed to parse config.json");

    let ethernet: Ethernet = {
        // let config_data: serde_json::Value =
        //     serde_json::from_str(&contents).expect("Failed to parse config.json");
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
        let vendor = config_data["ethernet"]["vendor"]
            .as_str().expect("Invalid vendor field")
            .to_owned();
        Ethernet { interface, local_ipv4, local_ipv6, remote_ip, remote_port, role, vendor }
    };

    let doip: Doip = {
        // let config_data: serde_json::Value =
        //     serde_json::from_str(&contents).expect("Failed to parse config.json");
        let version_string = config_data["doip"]["version"]
            .as_str().expect("Invalid version field")
            .to_owned();
        let inverse_version_string = config_data["doip"]["inverse_version"]
            .as_str().expect("Invalid inverse_version field")
            .to_owned();
        let tester_addr_string = config_data["doip"]["tester_addr"]
            .as_str().expect("Invalid tester_addr field")
            .to_owned();
        let ecu_addr_string = config_data["doip"]["ecu_addr"]
            .as_str().expect("Invalid ecu_addr field")
            .to_owned();
        let sga_addr_string = config_data["doip"]["sga_addr"]
            .as_str().expect("Invalid sga_addr field")
            .to_owned();
        let activation_code_string = config_data["doip"]["activation_code"]
            .as_str().expect("Invalid activation_code field")
            .to_owned();

        let version_string = version_string.trim_start_matches("0x");
        let version: u8 = match u8::from_str_radix(version_string, 16) {
            Ok(result) => result,
            Err(err) => {
                let error_message = format!("version string in json file not correct type: {}", err);
                eprintln!("version string in json file not correct type: {}", err);
                return Err(Error::new(ErrorKind::InvalidInput, error_message));
            }
        };
        let inverse_version_string = inverse_version_string.trim_start_matches("0x");
        let inverse_version: u8 = match u8::from_str_radix(inverse_version_string, 16) {
            Ok(result) => result,
            Err(err) => {
                let error_message = format!("version string in json file not correct type: {}", err);
                eprintln!("version string in json file not correct type: {}", err);
                return Err(Error::new(ErrorKind::InvalidInput, error_message));
            }
        };
        let tester_addr_string = tester_addr_string.trim_start_matches("0x");
        let tester_addr: u16 = match u16::from_str_radix(tester_addr_string, 16) {
            Ok(result) => result,
            Err(err) => {
                let error_message = format!("version string in json file not correct type: {}", err);
                eprintln!("version string in json file not correct type: {}", err);
                return Err(Error::new(ErrorKind::InvalidInput, error_message));
            }
        };
        let ecu_addr_string = ecu_addr_string.trim_start_matches("0x");
        let ecu_addr: u16 = match u16::from_str_radix(ecu_addr_string, 16) {
            Ok(result) => result,
            Err(err) => {
                let error_message = format!("version string in json file not correct type: {}", err);
                eprintln!("version string in json file not correct type: {}", err);
                return Err(Error::new(ErrorKind::InvalidInput, error_message));
            }
        };
        let sga_addr_string = sga_addr_string.trim_start_matches("0x");
        let sga_addr: u16 = match u16::from_str_radix(sga_addr_string, 16) {
            Ok(result) => result,
            Err(err) => {
                let error_message = format!("version string in json file not correct type: {}", err);
                eprintln!("version string in json file not correct type: {}", err);
                return Err(Error::new(ErrorKind::InvalidInput, error_message));
            }
        };
        let activation_code_string = activation_code_string.trim_start_matches("0x");
        let activation_code: u8 = match u8::from_str_radix(activation_code_string, 16) {
            Ok(result) => result,
            Err(err) => {
                let error_message = format!("version string in json file not correct type: {}", err);
                eprintln!("version string in json file not correct type: {}", err);
                return Err(Error::new(ErrorKind::InvalidInput, error_message));
            }
        };
        Doip { version, inverse_version, tester_addr, ecu_addr, sga_addr, activation_code }
    };

    let parameter: Parameters = {
        let vin_string = config_data["parameter"]["vin"]
            .as_str().expect("Invalid vin number field")
            .to_owned();
        let vin_string = vin_string.trim_start_matches("0x");
        let tester_present_bool = config_data["parameter"]["tester_present"].
                                        as_bool().expect("Invalid tester_present field").to_owned();
        let tester_present_interval_string = config_data["parameter"]["tester_present_interval"].
                                                     as_str().expect("Invalid interval field").to_owned();
        Parameters {
            vin: vin_string.to_string(),
            tester_present: tester_present_bool,
            tester_present_interval: tester_present_interval_string
        }
    };

    // Update the CONFIG global variable
    *CONFIG.write().expect("Failed to acquire write lock") = Config {
        ethernet,
        doip,
        parameter,
    };
    debug!("Parsed configuration parameters successfully!");

    Ok(())
}


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
pub fn parse(config_filename: String) -> Result<(), io::Error> {
    // Read the JSON file and update the `CONFIG` global variable
    let mut file = File::open(&config_filename).expect("Failed to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    return parse_content(contents);
}