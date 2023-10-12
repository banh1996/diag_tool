use log::debug;
use std::sync::{Arc, Mutex};
use std::io::{self, Error, ErrorKind};
use serde_json::{self, Value};

use crate::executor::executor::Executor;
use crate::executor::parameters::SequenceItem;
use crate::transport::config::CONFIG;


/*****************************************************************************************************************
 *  cli::parse function
 *  brief      Parse command lines and execute them
 *  details    -
 *  \param[in]  input: command string
 *  \param[out] -
 *  \precondition -
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
 pub fn parse(executor_obj: Arc<Mutex<Executor>>, input: &str) -> Result<(), io::Error> {
    let config = CONFIG.read().unwrap();
    // Split the input based on ":" and collect the parts into a vector
    let parts: Vec<&str> = input.splitn(2, ':').collect();

    if parts.len() < 2 {
        eprintln!("use format like this send_diag:1001");
        return Err(Error::new(ErrorKind::InvalidInput, "wrong input format"));
    }

    let name = parts[0].trim();
    let action = parts[1].trim();
    let trimmed_action = action.replace(" ", "");
    let mut action_value: Value = Value::Null;

    match name {
        "socket" | "send_doip" => {
            action_value = Value::String(String::from(trimmed_action))
        }
        "send_diag" => {
            action_value = Value::Array(vec![Value::String(String::from(trimmed_action))])
        }
        s if s.starts_with("securityaccess_") => {
            let result: Result<Value, serde_json::Error> = serde_json::from_str(trimmed_action.as_str());
            match result {
                Ok(parsed_json) => {
                    action_value = parsed_json;
                }
                Err(e) => {
                    println!("Error parsing securityaccess action: {}", e);
                }
            }
        }
        "swdl" => {
            let result: Result<Value, serde_json::Error> = serde_json::from_str(trimmed_action.as_str());
            match result {
                Ok(parsed_json) => {
                    action_value = parsed_json;
                }
                Err(e) => {
                    println!("Error parsing securityaccess action: {}", e);
                }
            }
        }
        _ => {
            action_value = Value::Null;
            println!("Handling other cases: {}", name);
        }
    }

    let item = SequenceItem {
        name: String::from(name),
        description: String::from("item_description"),
        action: action_value,
        expect: Value::Array(vec![
            Value::String(String::from("*")),
        ]),
        timeout: String::from("10s"),
        fail: String::from(""),
    };

    match Executor::execute_cmd(Arc::clone(&executor_obj), item, &config.ethernet.vendor) {
        Ok(()) => debug!("Command executed successfully!"),
        Err(err) => {
            eprintln!("Error executing command: {}, STOP", err);
            return Err(err);
        }
    }

    Ok(())
}