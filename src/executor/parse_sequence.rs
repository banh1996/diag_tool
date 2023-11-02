use log::debug;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{self, Read, Error, ErrorKind};

use crate::executor::parameters::Sequence;
use crate::executor::executor::Executor;
use crate::transport::config::CONFIG;

/*****************************************************************************************************************
 *  executor::parse function
 *  brief      parse_content json file to get sequence parameters
 *  details    -
 *  \param[in]  json_contents  json file content
 *  \param[out] -
 *  \precondition -
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
pub fn parse_content(json_contents: String, executor_obj: Arc<Mutex<Executor>>) -> Result<(), io::Error> {
    let config = CONFIG.read().unwrap();

    // Deserialize the JSON content
    let seq_obj: Sequence = match serde_json::from_str(&json_contents) {
        Ok(obj) => obj,
        Err(err) => {
            eprintln!("Failed to parse JSON: {}", err);
            return Err(Error::new(ErrorKind::InvalidData, "wrong sequence json format"));
        }
    };

    // Check tester-present
    if config.parameter.tester_present == true {
        match Executor::start_tester_present(Arc::clone(&executor_obj), config.parameter.tester_present_interval.to_string())  {
            Ok(()) => debug!("start tester present successfully!"),
            Err(err) => {
                eprintln!("Error start tester present: {}, STOP", err);
                return Err(err);
            }
        }
    }

    for item in seq_obj.sequence {
        // Access fields of the SequenceItem struct for processing
        debug!("Name: {}", item.name);
        debug!("Description: {}", item.description);
        debug!("Action: {:?}", item.action);
        debug!("Expect: {:?}", item.expect);

        match Executor::execute_cmd(Arc::clone(&executor_obj), item, &config.ethernet.vendor) {
            Ok(()) => debug!("Command executed successfully!"),
            Err(err) => {
                eprintln!("Error executing command: {}, STOP", err);
                return Err(err);
            }
        }
    }

    Ok(())
}


/*****************************************************************************************************************
 *  executor::parse function
 *  brief      Parse json file to get sequence parameters
 *  details    -
 *  \param[in]  sequence_filename  path to sequence json file
 *  \param[out] -
 *  \precondition -
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
pub fn parse(sequence_filename: String, executor_obj: Arc<Mutex<Executor>>) -> Result<(), io::Error> {
    // Read the JSON file
    let mut json_contents = String::new();
    match File::open(&sequence_filename) {
        Ok(mut file) => {
            file.read_to_string(&mut json_contents).expect("Failed to read file");
            if let Err(err) = file.read_to_string(&mut json_contents) {
                eprintln!("Failed to read file: {}", err);
                return Err(Error::new(ErrorKind::InvalidData, "Cannot read sequence file"));
            }
        }
        Err(err) => {
            eprintln!("Failed to open config file: {}", err);
            return Err(Error::new(ErrorKind::NotFound, "Not found sequence file"));
        }
    };

    return parse_content(json_contents, executor_obj);
}