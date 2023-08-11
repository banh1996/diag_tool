extern crate getopts;
extern crate log;
extern crate serde_json;
extern crate serde;

use std::thread;
use log::debug;
use std::env;
use getopts::Options;
//use log::{debug, info};

mod utils {
    pub mod parse_config;
}

mod executor {
    pub mod parse_sequence;
}

mod transport {
    pub mod diag;
    pub mod doip;
    pub mod soad;
    pub mod config;
}

use utils::parse_config; // Import the parse config module
use executor::parse_sequence; // Import the parse sequence module


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
    //TODO: pass Diag object to executor
    let mut diag_obj = transport::diag::create_diag();

    // Call connect method, test
    match diag_obj.connect() {
        Ok(()) => debug!("Connected successfully!"),
        Err(err) => eprintln!("Failed to connect: {}", err),
    }

    // Call activate method, test
    match diag_obj.send_doip_routing_activation() {
        Ok(()) => debug!("send_doip_routing_activation successfully!"),
        Err(err) => eprintln!("Failed to send_doip_routing_activation: {}", err),
    }

    /* handle json sequence file */
    //TODO
    if let Some(sequence_filename) = matches.opt_str("sequence") {
        match parse_sequence::parse(sequence_filename) {
            Ok(()) => {}
            Err(err) => {
                eprintln!("Error reading sequence file {}", err);
                return;
            }
        };
    } else {
        eprintln!("Error: --sequence option is required");
        print_usage(&args[0], &opts);
        return;
    }

    //TODO: spawn thread to handl sequence, main thread to handle CLI
    // Spawn a new thread to handle data reception and detach it.
    thread::spawn(move || {
        let p_data: Vec<u8> = vec![b'a', b'b', b'c', 2, 3, 4, 5, 6, 7, 8, b'z'];
        debug!("Sending data");
        match diag_obj.send_diag(p_data) {
            Ok(()) => debug!("Sent data successfully!"),
            Err(err) => eprintln!("Failed to connect: {}", err),
        }

        match diag_obj.receive_diag( 10) {
            Ok(data) => {
                // Process the received data
                debug!("Received {} bytes: {:?}", data.len(), data);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }

        if let Err(err) = diag_obj.disconnect() {
            eprintln!("diag disconnect Error: {}", err);
            return;
        }
        else {
            debug!("Disconnected!");
        }
    });

    //TODO: impliment CLI
    loop {}
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}