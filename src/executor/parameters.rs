use serde::{Deserialize, Serialize};
use std::sync::RwLock;

#[derive(Debug, Deserialize, Serialize)]
pub struct Parameters {
    pub vin: String,
    pub algorithm: String,
    pub key_lv1: String,
    pub key_lv2: String,
    pub key_lv3: String,
    pub key_lv4: String,
    pub tester_present: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SequenceItem {
    pub name: String,
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
        algorithm: String::new(),
        key_lv1: String::new(),
        key_lv2: String::new(),
        key_lv3: String::new(),
        key_lv4: String::new(),
        tester_present: false,
    });
}