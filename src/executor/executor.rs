use log::debug;
use std::io::{self, Error, ErrorKind};
use std::sync::{Arc, Mutex};
use serde_json::Value;
use utils;

use crate::transport;
use crate::executor::parameters::SequenceItem;
use crate::executor::securityaccess;

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
pub fn execute_cmd(&mut self, item: SequenceItem, vendor: &str) -> Result<(), io::Error> {
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
                if vendor == "volvo" {
                    match securityaccess::security_access_volvo(stream, item, level, timeout) {
                        Ok(()) => {debug!("Security Access level {} successful", level);}
                        Err(err) => {
                            eprintln!("Failed to send diag Secure access: {}", err);
                            return Err(err);
                        }
                    }
                }
            } else {
                eprintln!("Invalid security name format: {}", s);
                return Err(Error::new(ErrorKind::InvalidInput, "Invalid security name format"));
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
