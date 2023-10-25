#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use log::debug;
use std::sync::{Arc, Mutex};
use serde_json::{self, Value};

use std::env;

use crate::executor::executor::Executor;
use crate::executor::parameters::SequenceItem;
use crate::transport::config::CONFIG;
use crate::utils; // Import the parse config module
//use crate::executor::parse_sequence; // Import the parse sequence module
use crate::transport::diag;

use std::collections::HashMap;

use tauri::State;
// use tauri::Window;

// struct Counter(AtomicUsize);

#[derive(Default)]
struct Database(Arc<Mutex<HashMap<String, String>>>);

lazy_static::lazy_static! {
    static ref EXECUTOR_OBJ: Arc<Mutex<Executor>> = Arc::new(Mutex::new(Executor::create_executor(Arc::new(Mutex::new(diag::create_diag())))));
}



#[tauri::command]
fn connect() {
    let config = CONFIG.read().unwrap();
    let action_value = Value::String(String::from("connect"));
    let item = SequenceItem {
    name: String::from("socket"),
    description: String::from("item_description"),
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
fn disconnect() {
    let config = CONFIG.read().unwrap();
    let action_value = Value::String(String::from("disconnect"));
    let item = SequenceItem {
    name: String::from("socket"),
    description: String::from("item_description"),
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
fn senduds(value: String) {
    let config = CONFIG.read().unwrap();
    let action_value = Value::Array(vec![Value::String(String::from(value))]);
    let item = SequenceItem {
    name: String::from("send_diag"),
    description: String::from("item_description"),
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
    println!("{}", input);
}

pub fn run_gui() {
    /* Setup config  */
    match utils::parse_config::parse("../../json/config.json".to_string()) {
        Ok(()) => {
            debug!("Parse config json done!");
        },
        Err(e) => {
            eprintln!("parse config file error {}!", e);
            return;
        }
    }

    /* Run GUI */
    tauri::Builder::default()
    //.manage(Counter(AtomicUsize::new(0)))
    .manage(Database(Default::default()))
    .invoke_handler(tauri::generate_handler![
      senduds,
      parse,
      connect,
      disconnect
    ])
    .run(tauri::generate_context!(
      "src/gui/frontend/tauri.conf.json"
    ))
    .expect("error while running tauri application");
}