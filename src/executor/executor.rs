use log::debug;
use std::io;
use crate::executor::parameters::{Parameters, PARAMETERS, SequenceItem, Sequence};

pub fn execute_cmd(item: SequenceItem) -> Result<(), io::Error> {
    debug!("execute UDS cmd here");
    Ok(())
}