use log::debug;
use std::sync::{Arc, Mutex};
use std::io::{self, Error, ErrorKind, Write};
use serde_json::{self, Value};

use std::thread;
use std::env;
use getopts::Options;


use crate::executor::executor::Executor;
use crate::executor::parameters::SequenceItem;
use crate::transport::config::CONFIG;
use crate::utils; // Import the parse config module
use crate::executor::parse_sequence; // Import the parse sequence module
use crate::transport::diag;

/*****************************************************************************************************************
 *  cli::parse function
 *  brief      Parse command lines and execute them
 *  details    -
 *  \param[in]  input: command string
 *  \param[out] -
 *  \precondition -
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
pub fn parse(executor_obj: Arc<Mutex<Executor>>, input: &str) -> Result<(), io::Error> {
    let config = CONFIG.read().unwrap();
    // Split the input based on ":" and collect the parts into a vector
    let parts: Vec<&str> = input.splitn(2, ':').collect();

    if parts.len() < 2 {
        eprintln!("use format like this send_diag:1001");
        return Err(Error::new(ErrorKind::InvalidInput, "wrong input format"));
    }

    let name = parts[0].trim();
    let action = parts[1].trim();
    let trimmed_action = action.replace(" ", "");
    let mut action_value: Value = Value::Null;

    match name {
        "socket" | "send_doip" => {
            action_value = Value::String(String::from(trimmed_action))
        }
        "send_diag" => {
            action_value = Value::Array(vec![Value::String(String::from(trimmed_action))])
        }
        s if s.starts_with("securityaccess_") => {
            let result: Result<Value, serde_json::Error> = serde_json::from_str(trimmed_action.as_str());
            match result {
                Ok(parsed_json) => {
                    action_value = parsed_json;
                }
                Err(e) => {
                    println!("Error parsing securityaccess action: {}", e);
                }
            }
        }
        "swdl" => {
            let result: Result<Value, serde_json::Error> = serde_json::from_str(trimmed_action.as_str());
            match result {
                Ok(parsed_json) => {
                    action_value = parsed_json;
                }
                Err(e) => {
                    println!("Error parsing securityaccess action: {}", e);
                }
            }
        }
        _ => {
            action_value = Value::Null;
            println!("Handling other cases: {}", name);
        }
    }

    let item = SequenceItem {
        name: String::from(name),
        description: String::from("item_description"),
        action: action_value,
        expect: Value::Array(vec![
            Value::String(String::from("*")),
        ]),
        timeout: String::from("10s"),
        fail: String::from(""),
    };

    match Executor::execute_cmd(Arc::clone(&executor_obj), item, &config.ethernet.vendor) {
        Ok(()) => debug!("Command executed successfully!"),
        Err(err) => {
            eprintln!("Error executing command: {}, STOP", err);
            return Err(err);
        }
    }

    Ok(())
}

pub fn run_cli() {
    // Define the available command-line options
    let mut opts = Options::new();
    opts.optopt("c", "config", "set input config json file name", "config.json");
    opts.optopt("s", "sequence", "set input sequence json file name", "sequence.json");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("d", "debug", "enable debug log");

    // Parse the command-line arguments
    let args: Vec<String> = env::args().collect();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    // Handle the parsed options
    if matches.opt_present("h") {
        print_usage(&args[0], &opts);
        return;
    }

    if matches.opt_present("debug") {
        env::set_var("RUST_LOG", "debug");
        env_logger::init();
        debug!("Debug logging enabled");
    }

    /* handle json config file */
    if let Some(config_filename) = matches.opt_str("config") {
        // Read the JSON file into a string
        //if let Ok() = parse_config::parse(config_filename)
        match utils::parse_config::parse(config_filename) {
            Ok(()) => {
                debug!("Parse config json done!");
            },
            Err(e) => {
                eprintln!("parse config file error {}!", e);
                return;
            }
        }
    } else {
        eprintln!("Error: --config option is required");
        print_usage(&args[0], &opts);
        return;
    }

    /* init transport module */
    let diag_obj = Arc::new(Mutex::new(diag::create_diag()));

    //Init Executor object
    let executor_obj = Arc::new(Mutex::new(Executor::create_executor(diag_obj)));
    let executor_obj_clone = Arc::clone(&executor_obj);

    /* handle json sequence file */
    thread::spawn(move || {
        if let Some(sequence_filename) = matches.opt_str("sequence") {
            match parse_sequence::parse(sequence_filename, executor_obj) {
                Ok(()) => {}
                Err(err) => {
                    eprintln!("Error reading sequence file {}", err);
                }
            };
        } else {
            eprintln!("Error: --sequence option is required");
            print_usage(&args[0], &opts);
        }
    });

    //handle CLI
    loop {
        let mut input = String::new();
        print!("CMD>>> ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        // Trim the input to remove leading/trailing whitespaces and newline characters
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        //parse cli
        match utils::cli::parse(Arc::clone(&executor_obj_clone), input) {
            Ok(()) => {}
            Err(err) => {
                eprintln!("Failed to do cli: {}", err);
            }
        }
    }
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}