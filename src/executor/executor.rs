use log::debug;
use std::io;
use std::sync::{Arc, Mutex};
use serde_json::Value;
use utils;
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
                                Err(err) => eprintln!("Failed to connect: {}", err),
                            }
                        }
                        "disconnect" => {
                            match stream.disconnect() {
                                Ok(()) => debug!("Disconnected successfully!"),
                                Err(err) => eprintln!("Failed to disconnect: {}", err),
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
                    println!("Invalid action format");
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
                                Err(err) => eprintln!("Failed to send doip activation: {}", err),
                            }
                            match stream.receive_doip(timeout) {
                                Ok(Some(data)) => debug!("Receive doip data {:?} successfully!", data),
                                Ok(None) => debug!("Receive doip successfully!"),
                                Err(err) => eprintln!("Failed to Receive doip activation: {}", err),
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
                    println!("Invalid action format");
                }
            }
        }
        "send_diag" => println!("Not support now!"),
        _ => println!("It's something else!"),
    }
    Ok(())
}

// Public function that returns a new Executor object
pub fn create_executor(s_diag_obj: Arc<Mutex<transport::diag::Diag>>) -> Self {
    Executor { s_diag_obj }
}

}

// Public function that returns a new Diag object
// pub fn create_executor() -> Exe {
//     Exe::init();

//     Exe {
//         //stream: None, // Initialize the stream field to None
//         s_diag_obj : Mutex::new(transport::diag::create_diag())
//     }
// }