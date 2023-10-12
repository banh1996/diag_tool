use log::debug;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{self, Read};

use crate::executor::parameters::{Parameters, PARAMETERS, Sequence};
use crate::executor::executor::Executor;
use crate::transport::config::CONFIG;

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
    let config = CONFIG.read().unwrap();

    // Read the JSON file
    let mut file = File::open(&sequence_filename).expect("Failed to open config file");
    let mut json_contents = String::new();
    file.read_to_string(&mut json_contents).expect("Failed to read file");

    // Deserialize the JSON content
    let seq_obj: Sequence = serde_json::from_str(&json_contents).expect("Failed to parse JSON");

    debug!("Sequence Parameter: {:#?}", seq_obj.parameter);

    // Update the PARAMETERS global variable
    *PARAMETERS.write().expect("Failed to acquire write lock") = Parameters {
        vin: seq_obj.parameter.vin,
        tester_present: seq_obj.parameter.tester_present,
        tester_present_interval: seq_obj.parameter.tester_present_interval,
    };

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