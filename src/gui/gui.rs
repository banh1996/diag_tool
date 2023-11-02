#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use log::debug;
use std::sync::{Arc, Mutex};
use serde_json::{self, Value};
use std::env;
// use serde_json::json;

use crate::executor::executor::Executor;
use crate::executor::parameters::SequenceItem;
use crate::utils; // Import the parse config module
use crate::executor::parse_sequence;
use crate::transport::diag;
use crate::transport::config::{Config, Ethernet, Doip, Parameters, CONFIG};

use std::path::PathBuf;
use tauri::api::dialog::blocking::FileDialogBuilder;


#[derive(Debug, serde::Serialize)]
enum GUIError {
    Error
}


lazy_static::lazy_static! {
    static ref EXECUTOR_OBJ: Arc<Mutex<Executor>> = Arc::new(Mutex::new(Executor::create_executor(Arc::new(Mutex::new(diag::create_diag())))));
    static ref SWDLPATHS: Arc<Mutex<Vec<PathBuf>>> = Arc::new(Mutex::new(Vec::new()));
    static ref SEQUENCEPATH: Arc<Mutex<PathBuf>> = Arc::new(Mutex::new(PathBuf::new()));
}

#[tauri::command]
async fn updateconfig(config: serde_json::Value) -> Result<(), GUIError> {
    lazy_static::lazy_static! {
        static ref LOCK: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
    }
    let _lock = match LOCK.try_lock() {
        Ok(guard) => guard,
        Err(_) => {
            return Err(GUIError::Error);
        }
    };
    let clone_config = config.clone();

    debug!("updateconfig config {:?}", clone_config);
    let config_string = serde_json::to_string(&config).unwrap();
    match utils::parse_config::parse_content(config_string) {
        Ok(()) => {
            debug!("Parse config json done!");
        },
        Err(e) => {
            eprintln!("parse config file error {}!", e);
            return Err(GUIError::Error);
        }
    }

    Ok(())
}


#[tauri::command]
async fn connect (remoteip: String,
            port: String,
            role: String,
            vendor: String,
            doipversion: String,
            testeraddr: String,
            ecuaddr: String,
            sgaaddr: String,
            activationcode: String,
            testerpresentenable: bool,
            testerpresentinterval: String)
    -> Result<(), GUIError> {
    lazy_static::lazy_static! {
        static ref LOCK: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
    }
    let _lock = match LOCK.try_lock() {
        Ok(guard) => guard,
        Err(_) => {
            return Err(GUIError::Error);
        }
    };

    //Parse config
    debug!("connect with parameters: {} {} {} {} {} {} {} {} {}",
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
        parameter: Parameters{
            vin: String::new(),
            tester_present: testerpresentenable,
            tester_present_interval: testerpresentinterval,
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
async fn disconnect() -> Result<(), GUIError> {
    lazy_static::lazy_static! {
        static ref LOCK: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
    }
    let _lock = match LOCK.try_lock() {
        Ok(guard) => guard,
        Err(_) => {
            return Err(GUIError::Error);
        }
    };

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
async fn senduds(value: String) -> Result<String, GUIError> {
    lazy_static::lazy_static! {
        static ref LOCK: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
    }
    let _lock = match LOCK.try_lock() {
        Ok(guard) => guard,
        Err(_) => {
            return Err(GUIError::Error);
        }
    };

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
async fn senddoip(value: String) -> Result<String, GUIError> {
    lazy_static::lazy_static! {
        static ref LOCK: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
    }
    let _lock = match LOCK.try_lock() {
        Ok(guard) => guard,
        Err(_) => {
            return Err(GUIError::Error);
        }
    };

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
        Ok(()) => {
            debug!("Command executed successfully!");
            Ok("senddoip successfully".to_string())
        }
        Err(err) => {
            eprintln!("Error executing command: {}, STOP", err);
            Err(GUIError::Error)
        }
    }
}

#[tauri::command]
async fn flash() -> Result<(), GUIError> {
    lazy_static::lazy_static! {
        static ref LOCK: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
    }
    let _lock = match LOCK.try_lock() {
        Ok(guard) => guard,
        Err(_) => {
            return Err(GUIError::Error);
        }
    };

    let paths =  SWDLPATHS.lock().unwrap();
    for path in paths.iter() {
        debug!("flashing {:?}", path);
        let config = CONFIG.read().unwrap();
        let action_str = format!(r#"["path:{}", "format:vbf"]"#, path.display().to_string().replace("\\", "\\\\"));
        let result: Result<Value, serde_json::Error> = serde_json::from_str(action_str.as_str());
        match result {
            Ok(action_value) => {
                let item = SequenceItem {
                    name: String::from("swdl"),
                    description: String::from("download vbf file"),
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
                        return Err(GUIError::Error);
                    }
                }
            }
            Err(e) => {
                println!("Error parsing swdl action: {}", e);
                return Err(GUIError::Error);
            }
        }
    }

    Ok(())
}

#[tauri::command]
async fn selectswdlfiles() {
    lazy_static::lazy_static! {
        static ref LOCK: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
    }
    let _lock = match LOCK.try_lock() {
        Ok(guard) => guard,
        Err(_) => {
            return;
        }
    };

    let dialog_result = FileDialogBuilder::new().pick_files();
    SWDLPATHS.lock().unwrap().clear();
    match dialog_result {
        Some(paths) => {
            for path in paths {
                let path: PathBuf = PathBuf::from(path);
                println!("Selected swdl-file: {:?}", path);
                SWDLPATHS.lock().unwrap().push(path);
            }
        },
        None => println!("User closed the folder dialog."),
    }
}

#[tauri::command]
async fn executesequence() -> Result<(), GUIError> {
    lazy_static::lazy_static! {
        static ref LOCK: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
    }
    let _lock = match LOCK.try_lock() {
        Ok(guard) => guard,
        Err(_) => {
            return Err(GUIError::Error);
        }
    };

    let path =  SEQUENCEPATH.lock().unwrap();

    match parse_sequence::parse(path.display().to_string(), EXECUTOR_OBJ.clone()) {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Error reading sequence file {}", err);
            return Err(GUIError::Error);
        }
    };

    Ok(())
}

#[tauri::command]
async fn selectsequencefile() {
    lazy_static::lazy_static! {
        static ref LOCK: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
    }
    let _lock = match LOCK.try_lock() {
        Ok(guard) => guard,
        Err(_) => {
            return;
        }
    };

    let dialog_result = FileDialogBuilder::new().pick_file();
    SEQUENCEPATH.lock().unwrap().clear();
    match dialog_result {
        Some(path) => {
            let path: PathBuf = PathBuf::from(path);
            println!("Selected sequence-file: {:?}", path);
            SEQUENCEPATH.lock().unwrap().push(path);
        },
        None => println!("User closed the folder dialog."),
    }
}

#[tauri::command]
async fn sendsecurityaccess(level: String, key: String) -> Result<(), GUIError> {
    lazy_static::lazy_static! {
        static ref LOCK: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
    }
    let _lock = match LOCK.try_lock() {
        Ok(guard) => guard,
        Err(_) => {
            return Err(GUIError::Error);
        }
    };

    let config = CONFIG.read().unwrap();
    let name: String;
    if !level.is_empty() && level.chars().all(|c| c.is_digit(16)) {//check level string should not empty and contain hex only
        //name = format!("securityaccess_{:02X}", level.parse::<u16>().unwrap());
        if let Ok(level_value) = u16::from_str_radix(&level, 16) {
            name = format!("securityaccess_{:02X}", level_value);
        } else {
            return Err(GUIError::Error);
        }
    }
    else {
        eprintln!("Error key or level format, STOP");
        return Err(GUIError::Error);
    }
    let action_str: String = format!(r#"["algorithm:AES128", "iv:random", "encryption_authentication_key:{}", "proof_of_ownership_key:{}"]"#, key, key);
    let result: Result<Value, serde_json::Error> = serde_json::from_str(action_str.as_str());
    match result {
        Ok(action_value) => {
            let item = SequenceItem {
                name: String::from(name),
                description: String::from(format!("Send security-access level {}", level)),
                action: action_value,
                expect: Value::Array(vec![
                    Value::String(String::from("*")),
                ]),
                timeout: String::from("5s"),
                fail: String::from(""),
            };
            match Executor::execute_cmd(EXECUTOR_OBJ.clone(), item, &config.ethernet.vendor) {
                Ok(()) => debug!("Command executed successfully!"),
                Err(err) => {
                    eprintln!("Error executing command: {}, STOP", err);
                    return Err(GUIError::Error);
                }
            }
        }
        Err(e) => {
            eprintln!("Error parsing security-access action: {}", e);
            return Err(GUIError::Error);
        }
    }

    Ok(())
}

#[tauri::command]
async fn triggertesterpresent(enable: bool, interval: String) -> Result<(), GUIError> {
    lazy_static::lazy_static! {
        static ref LOCK: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
    }
    let _lock = match LOCK.try_lock() {
        Ok(guard) => guard,
        Err(_) => {
            return Err(GUIError::Error);
        }
    };

    debug!("Tester-Present event: {} {}", enable, interval);

    if enable == true {
        match Executor::start_tester_present(EXECUTOR_OBJ.clone(), interval.to_string())  {
            Ok(()) => debug!("start tester present successfully!"),
            Err(err) => {
                eprintln!("Error start tester present: {}, STOP", err);
                return Err(GUIError::Error);
            }
        }
    }
    else {
        EXECUTOR_OBJ.clone().lock().unwrap().stop_tester_present();
    }

    Ok(())
}

/* for testing
use std::time::Duration;
use std::thread;
#[tauri::command]
async fn testdelay() {
    lazy_static::lazy_static! {
        static ref LOCK: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
    }
    println!("before called this");
    let _lock = match LOCK.try_lock() {
        Ok(guard) => guard,
        Err(_) => {
            println!("exit this");
            return;
        }
    };
    println!("after called this");
    thread::sleep(Duration::from_secs(10));
}
*/

pub fn run_gui() {
    /* Setup */
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    debug!("Debug logging enabled");

    /* Run GUI */
    tauri::Builder::default()
    // .manage(Database(Default::default()))
    .invoke_handler(tauri::generate_handler![
        updateconfig,
        connect,
        disconnect,
        senduds,
        senddoip,
        selectswdlfiles,
        selectsequencefile,
        flash,
        executesequence,
        sendsecurityaccess,
        triggertesterpresent
    ])
    .build(tauri::generate_context!("src/gui/frontend/tauri.conf.json"))
    .expect("error while running tauri application")
    .run(|_app, _event| {})
}

