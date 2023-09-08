use log::debug;
use executor::executor;
use std::fs::File;
use std::io::{self, Read};
use crate::executor::parameters::{Parameters, PARAMETERS, Sequence};


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
pub fn parse(sequence_filename: String) -> Result<(), io::Error> {
    // Read the JSON file and update the `CONFIG` global variable
    let mut file = File::open(&sequence_filename).expect("Failed to open config file");
    let mut json_contents = String::new();
    file.read_to_string(&mut json_contents).expect("Failed to read file");

    // Deserialize the JSON content
    let seq_obj: Sequence = serde_json::from_str(&json_contents).expect("Failed to parse JSON");

    debug!("Sequence Parameter: {:#?}", seq_obj.parameter);

    // Update the PARAMETERS global variable
    *PARAMETERS.write().expect("Failed to acquire write lock") = Parameters {
        vin: seq_obj.parameter.vin,
        algorithm: seq_obj.parameter.algorithm,
        key_lv1: seq_obj.parameter.key_lv1,
        key_lv2: seq_obj.parameter.key_lv2,
        key_lv3: seq_obj.parameter.key_lv3,
        key_lv4: seq_obj.parameter.key_lv4,
        tester_present: seq_obj.parameter.tester_present,
    };

    for item in seq_obj.sequence {
        // Access fields of the SequenceItem struct for processing
        debug!("Name: {}", item.name);
        debug!("Action: {:?}", item.action);
        debug!("Expect: {:?}", item.expect);
        debug!("Timeout: {}", item.timeout);
        debug!("Fail: {}", item.fail);
        debug!("--------------------------");

        //TODO: call to executor
        executor::execute_cmd(item);
    }


    Ok(())
}