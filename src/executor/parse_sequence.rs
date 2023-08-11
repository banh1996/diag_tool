use log::debug;
use transport;
use std::fs;
use std::io::{self, Read, Error, ErrorKind};

/*****************************************************************************************************************
 *  utils::parse function
 *  brief      Parse json file to get sequence parameters
 *  details    -
 *  \param[in]  sequence_filename  path to sequence json file
 *  \param[out] -
 *  \precondition -
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
 pub fn parse(sequence_filename: String) -> Result<(), io::Error> {
    //TODO
    let sequence_contents = match fs::read_to_string(sequence_filename) {
        Ok(contents) => contents,
        Err(err) => {
            //eprintln!("Error reading file {}: {}", sequence_filename, err);
            return Err(Error::new(ErrorKind::InvalidInput, err));
        }
    };
    debug!("Parsed sequences successfully! {}", sequence_contents);
    Ok(())
}