use serde::{Deserialize, Serialize};
use std::sync::RwLock;

#[derive(Debug, Deserialize, Serialize)]
pub struct Parameters {
    pub vin: String,
    pub tester_present: bool,
    pub tester_present_interval: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SequenceItem {
    pub name: String,
    pub description: String,
    pub action: serde_json::Value, // Use serde_json::Value to handle dynamic action data
    pub expect: serde_json::Value, // Use serde_json::Value to handle dynamic expect data
    pub timeout: String,
    pub fail: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FailHandler {
    pub send_diag: SequenceItem,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sequence {
    pub parameter: Parameters,
    pub sequence: Vec<SequenceItem>,
    pub fail_handler: FailHandler,
}


lazy_static::lazy_static! {
    pub static ref PARAMETERS: RwLock<Parameters> = RwLock::new(Parameters {
        vin: String::new(),
        tester_present: false,
        tester_present_interval: String::new(),
    });
}