use log::debug;
use std::io::{self, Error, ErrorKind};
use std::sync::{Arc, Mutex};
use serde_json::Value;
use utils;
use rand::Rng;

use crate::transport;
use crate::executor::parameters::SequenceItem;

pub struct Executor {
    s_diag_obj: Arc<Mutex<transport::diag::Diag>>,
}

impl Executor {

/*****************************************************************************************************************
 *  executor::executor::execute_cmd function
 *  brief      Function to execute items in sequence json file
 *  details    -
 *  \param[in]  item: refer to SequenceItem
 *  \param[out] -
 *  \precondition -
 *  \reentrant:  TRUE
 *  \return     error code if any
 ****************************************************************************************************************/
pub fn execute_cmd(&mut self, item: SequenceItem) -> Result<(), io::Error> {
    // Get timeout value
    let mut timeout: u64 = 1000; //1000ms as default
    match utils::common::parse_duration_to_milliseconds(item.timeout.as_str()) {
        Some(temp) => {timeout = temp},
        None => debug!("Invalid duration: {}", item.timeout),
    }

    let mut stream = self.s_diag_obj.lock().unwrap();
    match item.name.as_str() {
        "socket" => {
            match &item.action {
                Value::String(single_action_str) => {
                    match single_action_str.as_str() {
                        "connect" => {
                            match stream.connect() {
                                Ok(()) => debug!("Connected successfully!"),
                                Err(err) => {
                                    eprintln!("Failed to connect: {}", err);
                                    return Err(err);
                                }
                            }
                        }
                        "disconnect" => {
                            match stream.disconnect() {
                                Ok(()) => debug!("Disconnected successfully!"),
                                Err(err) => {
                                    eprintln!("Failed to disconnect: {}", err);
                                    return Err(err);
                                }
                            }
                        }
                        _ => eprintln!("Invalid single action format: {}", single_action_str),
                    }
                }
                Value::Array(multiple_actions) => {
                    debug!("Not allow Multiple Actions in socket object:");
                    for action in multiple_actions {
                        if let Value::String(action_string) = action {
                            println!("{}", action_string);
                        }
                    }
                }
                _ => {
                    debug!("socket Invalid action format");
                }
            }
        }
        "send_doip" => {
            match &item.action {
                Value::String(single_action_str) => {
                    match single_action_str.as_str() {
                        "activation" => {
                            match stream.send_doip_routing_activation() {
                                Ok(()) => debug!("Send doip successfully!"),
                                Err(err) => {
                                    eprintln!("Failed to send doip activation: {}", err);
                                    return Err(err);
                                }
                            }
                            match stream.receive_doip(timeout) {
                                Ok(Some(data)) => debug!("Receive doip data {:?} successfully!", data),
                                Ok(None) => debug!("Receive doip successfully!"),
                                Err(err) => {
                                    eprintln!("Failed to Receive doip activation: {}", err);
                                    return Err(err);
                                }
                            }
                        }
                        _ => eprintln!("Invalid single action format: {}", single_action_str),
                    }
                }
                Value::Array(multiple_actions) => {
                    debug!("Not allow Multiple Actions in socket object:");
                    for action in multiple_actions {
                        if let Value::String(action_string) = action {
                            debug!("{}", action_string);
                        }
                    }
                }
                _ => debug!("send_doip Invalid action format"),
            }
        }
        "send_diag" => {
            match &item.action {
                Value::String(single_action_str) => {
                    match single_action_str.as_str() {
                        _ => debug!("Please use action&expect of send_diag format like this: [\"1001\"]")
                    }
                }
                Value::Array(multiple_actions) => {
                    // Handle Value::Array case
                    let mut action_vecs: Vec<Vec<u8>> = Vec::new();
                    for action_str in multiple_actions.iter() {
                        if let Some(action) = action_str.as_str() {
                            let parsed_action: Vec<u8> = action
                                .chars()
                                .collect::<Vec<char>>()
                                .chunks(2)
                                .map(|chunk| {
                                    let hex_str: String = chunk.iter().collect();
                                    u8::from_str_radix(&hex_str, 16).unwrap_or(0)
                                })
                                .collect();
                            action_vecs.push(parsed_action);
                        }
                    }
                    // Now 'action_vecs' contains the parsed Vec<u8> for multiple actions
                    for (i, action) in action_vecs.iter().enumerate() {
                        let hex_action: Vec<String> = action.iter().map(|&x| format!("0x{:02X}", x)).collect();
                        let u8_action = utils::common::hex_strings_to_u8(&hex_action);
                        let first_byte = u8_action[0];
                        match stream.send_diag(u8_action) {
                            Ok(()) => {}
                            Err(err) => {
                                eprintln!("Failed to send diag activation: {}", err);
                                return Err(err);
                            }
                        }
                        //Check suppress reply bit
                        if (first_byte & 0x80) == 0x80 {
                            debug!("found suppress bit, ignore checking respond diag");
                            //Ignore DoIP ACK
                            match stream.receive_doip(timeout) {
                                Ok(Some(_data)) => {}
                                Ok(None) => {}
                                Err(err) => eprintln!("Failed to Receive doip activation: {}", err),
                            }
                            continue;
                        }
                        match stream.receive_diag(timeout) {
                            Ok(data) => {
                                // Access the "expect" array
                                if let Some(expect_array) = item.expect.as_array() {
                                    if i < expect_array.len() {
                                        // Access the "expect" value at the specified index
                                        let expect_value = &expect_array[i];
                                        // Check if the value is a string
                                        if let Some(expect_str) = expect_value.as_str() {
                                            debug!("Sent {:?}, Expect at index {}: {}, Receive {:?}", hex_action, i, expect_str, data);
                                            if utils::common::compare_expect_value(expect_str, data) == false {
                                                return Err(Error::new(ErrorKind::InvalidData, "Diag data received is not expected"));
                                            }
                                        } else {
                                            eprintln!("Value at index {} is not a string.", i);
                                            return Err(Error::new(ErrorKind::InvalidData, "wrong sequence json format"));
                                        }
                                    }
                                }
                                // debug!("Receive diag data {:?} successfully! {}", data, utils::common::check_expect(&item.expect.as_str(), data));
                            }
                            Err(err) => {
                                return Err(err);
                            }
                        }
                    }
                }
                _ => {
                    eprintln!("send_diag Invalid action format");
                    return Err(Error::new(ErrorKind::InvalidData, "wrong sequence json format"));
                }
            }
        }
        s if s.starts_with("securityaccess_") => {
            // Extract the number after "securityaccess_"
            if let Ok(level) = u8::from_str_radix(&s[15..], 16) {
                debug!("Security Access level: {}", level);
                let mut rng = rand::thread_rng();
                //get parameter of secure-access in action item
                let message_id: u16 = 0x0001;
                let authentication_method: u16 = 0x0001;
                let mut algorithm = "";
                let mut iv: &str = "";
                let mut clientkey = "";
                let mut serverkey = "";
                match &item.action {
                    Value::Array(multiple_actions) => {
                        //get security-access parameters in action item
                        for action_str in multiple_actions.iter() {
                            if let Some(action) = action_str.as_str() {
                                let parts: Vec<&str> = action.split(':').collect();
                                if parts.len() == 2 {
                                    match parts[0] {
                                        "algorithm" => algorithm = parts[1],
                                        "iv" => iv = parts[1],
                                        "clientkey" => clientkey = parts[1],
                                        "serverkey" => serverkey = parts[1],
                                        _ => (),
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        eprintln!("Not enough parameters");
                        return Err(Error::new(ErrorKind::InvalidData, "wrong sequence json format"));
                    }
                }
                // Convert message_id and authentication_method to big endian bytes
                let message_id_bytes = message_id.to_be_bytes();
                let authentication_method_bytes = authentication_method.to_be_bytes();
                let specified_bytes: [u8; 6] = [
                    0x27,
                    level,
                    message_id_bytes[0], // Big endian byte 1 of message_id
                    message_id_bytes[1], // Big endian byte 2 of message_id
                    authentication_method_bytes[0], // Big endian byte 1 of authentication_method
                    authentication_method_bytes[1], // Big endian byte 2 of authentication_method
                ];
                let mut byte_array: Vec<u8> = Vec::with_capacity(52);
                byte_array.extend_from_slice(&specified_bytes);
                if algorithm == "AES-128-CMAC" {
                    let mut iv_bytes: [u8; 16] = [0; 16];
                    if iv == "random" {
                        for i in 0..iv_bytes.len() {
                            iv_bytes[i] = rng.gen::<u8>();
                        }
                    }
                    //add iv
                    byte_array.extend_from_slice(&iv_bytes);

                    // Generating random seed
                    let mut seed_bytes: [u8; 16] = [0; 16];
                    for i in 0..seed_bytes.len() {
                        seed_bytes[i] = rng.gen::<u8>();
                    }

                    //Create Request seed packet
                    match utils::excrypto::encrypt_aes128_ctr(&seed_bytes, &iv_bytes, clientkey) {
                        Ok(encrypted_data_record) => {
                            debug!("Secure-access ctr encrypted_data_record: {:?}", encrypted_data_record);
                            let encrypted_first_16_bytes: Vec<u8> = encrypted_data_record.iter().take(16).cloned().collect();
                            //add encrypted_data_record
                            byte_array.extend_from_slice(&encrypted_first_16_bytes);

                            match utils::excrypto::encrypt_aes128_cmac(&byte_array, clientkey) {
                                Ok(authentication_data) => {
                                    debug!("Secure-access cmac Encrypted data: {:?}", authentication_data);
                                    let authentication_data_first_16_bytes: Vec<u8> = authentication_data.iter().take(16).cloned().collect();
                                    //add authentication_data
                                    byte_array.extend_from_slice(&authentication_data_first_16_bytes);
                                }
                                Err(err) => {
                                    eprintln!("Encryption error: {}", err);
                                    return Err(err);
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("Encryption error: {}", err);
                            return Err(err);
                        }
                    }
                }

                // Send ClientRequestSeed
                match stream.send_diag(byte_array) {
                    Ok(()) => {}
                    Err(err) => {
                        eprintln!("Failed to send diag Secure access: {}", err);
                        return Err(err);
                    }
                }
                match &item.expect {
                    Value::Array(multiple_expects) => {
                        //get security-access parameters in expect item
                        for expect_str in multiple_expects.iter() {
                            if let Some(expect) = expect_str.as_str() {
                                match stream.receive_diag(timeout) {
                                    Ok(data) => {
                                        debug!("Sent secure-access, Expect: {}, Receive {:?}", expect_str, data);
                                        if utils::common::compare_expect_value(expect, data) == false {
                                            return Err(Error::new(ErrorKind::InvalidData, "secure-access Diag data received is not expected"));
                                        }
                                    }
                                    Err(err) => {
                                        return Err(err);
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        eprintln!("Not enough parameters");
                        return Err(Error::new(ErrorKind::InvalidData, "wrong sequence json format"));
                    }
                }

                //TODO: Implement SendKey
            } else {
                eprintln!("Invalid SA format: {}", s);
            }
        }
        "swdl" => {debug!("not support now")}
        _ => println!("This action name is not supported"),
    }
    Ok(())
}

// Public function that returns a new Executor object
pub fn create_executor(s_diag_obj: Arc<Mutex<transport::diag::Diag>>) -> Self {
    Executor { s_diag_obj }
}

}
