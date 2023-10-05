extern crate getopts;
extern crate log;
extern crate serde_json;
extern crate serde;
extern crate rand;

extern crate cmac;
extern crate aes;
extern crate cipher;
extern crate ctr;
extern crate hex;

use std::thread;
use log::debug;
use std::env;
use getopts::Options;
use std::sync::{Arc, Mutex};
use std::io;

mod utils {
    pub mod parse_config;
    pub mod common;
    pub mod excrypto;
}

mod executor {
    pub mod parse_sequence;
    pub mod parameters;
    pub mod securityaccess;
    pub mod swdl;
    pub mod executor;
}

mod transport {
    pub mod diag;
    pub mod doip;
    pub mod soad;
    pub mod config;
}

use utils::parse_config; // Import the parse config module
use executor::parse_sequence; // Import the parse sequence module
use transport::diag;

fn main() {
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
        match parse_config::parse(config_filename) {
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
    let clone_diag_obj = Arc::clone(&diag_obj);

    /* handle json sequence file */
    thread::spawn(move || {
        if let Some(sequence_filename) = matches.opt_str("sequence") {
            match parse_sequence::parse(sequence_filename, diag_obj) {
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

    //TODO: spawn thread to handl sequence, main thread to handle CLI
    // Spawn a new thread to handle data reception and detach it.

    loop {
        let mut input = String::new();
        print!("CMD>>> ");
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Trim the input to remove leading/trailing whitespaces and newline characters
        let input = input.trim();

        // Split the input based on ":" and collect the parts into a vector
        let parts: Vec<&str> = input.split(':').collect();

        // Ensure there are exactly two parts (before and after ":")
        if parts.len() == 2 {
            //TODO: add some actions here
            let name = parts[0].trim();
            let action = parts[1].trim();
            let trimmed_action = action.replace(" ", "");
            let mut stream = clone_diag_obj.lock().unwrap();
            if name == "send_diag" {
                //let vec_action = utils::common::hex_string_to_bytes(&trimmed_action);
                let vec_action = match utils::common::hex_string_to_bytes(&trimmed_action) {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        println!("Error when parse action: {}", e);
                        Vec::new()
                    }
                };
                let clone_vec_action = vec_action.clone();
                // Execute command
                match stream.send_diag(vec_action) {
                    Ok(()) => {debug!("Sent diag data {:02X?}", clone_vec_action)}
                    Err(err) => {eprintln!("Failed to send diag data: {}", err)}
                }
                match stream.receive_diag(3000) { //timeout 3s
                    Ok(data) => {debug!("Sent diag data {:02X?}, Receive {:02X?}", clone_vec_action, data)}
                    Err(err) => {eprintln!("Failed to receive diag data: {}", err)}
                }
            }
            else if name == "socket" {
                match trimmed_action.as_str() {
                    "connect" => {
                        match stream.connect() {
                            Ok(()) => debug!("Connected successfully!"),
                            Err(err) => {
                                eprintln!("Failed to connect: {}", err);
                            }
                        }
                    }
                    "disconnect" => {
                        match stream.disconnect() {
                            Ok(()) => debug!("Disconnected successfully!"),
                            Err(err) => {
                                eprintln!("Failed to disconnect: {}", err);
                            }
                        }
                    }
                    _ => eprintln!("Invalid socket action format: {}", trimmed_action),
                }
            }
        } else {
            eprintln!("Invalid input format. Please enter a string with exactly one ':' character.\n
                        Example: uds:22f186   doip:activation");
        }
    }


    //TODO: impliment CLI
    //loop {}
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}