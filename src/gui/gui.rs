#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use log::debug;
use std::sync::{Arc, Mutex};
use serde_json::{self, Value};
use std::env;

use crate::executor::executor::Executor;
use crate::executor::parameters::SequenceItem;
use crate::utils; // Import the parse config module
//use crate::executor::parse_sequence; // Import the parse sequence module
use crate::transport::diag;
use crate::transport::config::{Config, Ethernet, Doip, CONFIG};

use std::collections::HashMap;

// use tauri::State;
// use tauri::Window;

// struct Counter(AtomicUsize);

#[derive(Debug, serde::Serialize)]
enum GUIError {
    Error
}

#[derive(Default)]
struct Database(Arc<Mutex<HashMap<String, String>>>);

lazy_static::lazy_static! {
    static ref EXECUTOR_OBJ: Arc<Mutex<Executor>> = Arc::new(Mutex::new(Executor::create_executor(Arc::new(Mutex::new(diag::create_diag())))));
}



#[tauri::command]
fn connect( remoteip: String,
            port: String,
            role: String,
            vendor: String,
            doipversion: String,
            testeraddr: String,
            ecuaddr: String,
            sgaaddr: String,
            activationcode: String)
    -> Result<(), GUIError> {
    //Parse config
    println!("connect: {} {} {} {} {} {} {} {} {}",
    remoteip, port, role, vendor, doipversion, testeraddr, ecuaddr, sgaaddr, activationcode);
    *CONFIG.write().expect("Failed to acquire write lock") = Config {
        ethernet: Ethernet {
            interface: String::from("eth0"),
            local_ipv4: Some(String::from("localhost")),
            local_ipv6: Some(String::from("localhost")),
            remote_ip: remoteip,
            remote_port: port,
            role: String::from("client"),
            vendor: vendor,
        },
        doip: Doip {
            version: if doipversion == "ISO13400_2" { 0x2 } else { 0x3 },
            inverse_version: if doipversion == "ISO13400_2" { 0xfd } else { 0 },
            tester_addr: utils::common::hex_to_u16(testeraddr.as_str()),
            ecu_addr: utils::common::hex_to_u16(ecuaddr.as_str()),
            sga_addr: utils::common::hex_to_u16(sgaaddr.as_str()),
            activation_code: utils::common::hex_to_u16(activationcode.as_str()) as u8,
        },
    };

    // connect to SGA
    let config: std::sync::RwLockReadGuard<'_, Config> = CONFIG.read().unwrap();
    debug!("get config {:?}", config);
    let item = SequenceItem {
        name: String::from("socket"),
        description: String::from("connect to SGA"),
        action: Value::String(String::from("connect")),
        expect: Value::Array(vec![
            Value::String(String::from("*")),
        ]),
        timeout: String::from("2s"),
        fail: String::from(""),
    };
    match Executor::execute_cmd(EXECUTOR_OBJ.clone(), item, &config.ethernet.vendor) {
        Ok(()) => {
            debug!("Command executed successfully!")
        }
        Err(err) => {
            eprintln!("Error executing command: {}, STOP", err);
            return Err(GUIError::Error);
        }
    }

    // send doip activation code
    let item = SequenceItem {
        name: String::from("send_doip"),
        description: String::from("doip activation"),
        action: Value::String(String::from("activation")),
        expect: Value::Array(vec![
            Value::String(String::from("*")),
        ]),
        timeout: String::from("2s"),
        fail: String::from(""),
    };
    match Executor::execute_cmd(EXECUTOR_OBJ.clone(), item, &config.ethernet.vendor) {
        Ok(()) => {
            debug!("Command executed successfully!")
        }
        Err(err) => {
            eprintln!("Error executing command: {}, STOP", err);
            return Err(GUIError::Error);
        }
    }

    Ok(())
}


#[tauri::command]
fn disconnect() -> Result<(), GUIError> {
    let config = CONFIG.read().unwrap();
    let item = SequenceItem {
    name: String::from("socket"),
    description: String::from("disconnect"),
    action: Value::String(String::from("disconnect")),
    expect: Value::Array(vec![
        Value::String(String::from("*")),
    ]),
    timeout: String::from("10s"),
    fail: String::from(""),
    };

    match Executor::execute_cmd(EXECUTOR_OBJ.clone(), item, &config.ethernet.vendor) {
        Ok(()) => debug!("Command executed successfully!"),
        Err(err) => {
            eprintln!("Error executing command: {}, STOP", err);
            return Err(GUIError::Error);
        }
    }
    Ok(())
}


#[tauri::command]
fn senduds(value: String) -> Result<String, GUIError> {
    let config = CONFIG.read().unwrap();
    let action_value = Value::Array(vec![Value::String(String::from(value))]);
    let item = SequenceItem {
        name: String::from("send_diag"),
        description: String::from("Send Diagnostic message"),
        action: action_value,
        expect: Value::Array(vec![
            Value::String(String::from("*")),
        ]),
        timeout: String::from("10s"),
        fail: String::from(""),
    };

    match Executor::execute_cmd(EXECUTOR_OBJ.clone(), item, &config.ethernet.vendor) {
        Ok(()) => {
            debug!("Command executed successfully!");
            Ok("senduds successfully".to_string())
        }
        Err(err) => {
            eprintln!("Error executing command: {}, STOP", err);
            Err(GUIError::Error)
        }
    }
}

#[tauri::command]
fn senddoip(value: String) {
    let config = CONFIG.read().unwrap();
    let action_value = Value::Array(vec![Value::String(String::from(value))]);
    let item = SequenceItem {
        name: String::from("send_doip"),
        description: String::from("Send Doip message"),
        action: action_value,
        expect: Value::Array(vec![
            Value::String(String::from("*")),
        ]),
        timeout: String::from("10s"),
        fail: String::from(""),
    };

    match Executor::execute_cmd(EXECUTOR_OBJ.clone(), item, &config.ethernet.vendor) {
        Ok(()) => debug!("Command executed successfully!"),
        Err(err) => {
            eprintln!("Error executing command: {}, STOP", err);
        }
    }
}


#[tauri::command]
fn parse(input: String) {
    println!("got {}", input);
}

pub fn run_gui() {
    /* Setup */
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    debug!("Debug logging enabled");

    /* Run GUI */
    tauri::Builder::default()
    //.manage(Counter(AtomicUsize::new(0)))
    .manage(Database(Default::default()))
    .invoke_handler(tauri::generate_handler![
        senduds,
        senddoip,
        parse,
        connect,
        disconnect
    ])
    .run(tauri::generate_context!(
        "src/gui/frontend/tauri.conf.json"
    ))
    .expect("error while running tauri application");
}