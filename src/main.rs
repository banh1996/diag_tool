extern crate getopts;
extern crate log;
extern crate serde_json;
extern crate serde;

use log::{debug};
use std::env;
use getopts::Options;
//use log::{debug, info};

mod utils {
    pub mod parse_config;
}

mod transport {
    pub mod diag;
    pub mod doip;
    pub mod soad;
    pub mod config;
}

use utils::parse_config; // Import the parse config module
use std::fs;

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
        parse_config::parse(config_filename);
    } else {
        eprintln!("Error: --config option is required");
        print_usage(&args[0], &opts);
        return;
    }

    /* init UDS layer */
    //TODO
    transport::diag::init();

    /* handle json sequence file */
    //TODO
    if let Some(sequence_filename) = matches.opt_str("sequence") {
        let sequence_contents = match fs::read_to_string(&sequence_filename) {
            Ok(contents) => contents,
            Err(err) => {
                eprintln!("Error reading file {}: {}", sequence_filename, err);
                return;
            }
        };
        debug!("File contents: {}", sequence_contents);
    } else {
        eprintln!("Error: --sequence option is required");
        print_usage(&args[0], &opts);
        return;
    }

    //TODO: spawn thread to handl sequence, main thread to handle CLI

}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}