#[cfg(feature = "cli")]
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

#[cfg(feature = "gui")]
extern crate tauri;

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

#[cfg(feature = "cli")]
mod cli {
    pub mod cli;
}

#[cfg(feature = "gui")]
mod gui {
    pub mod gui;
}


fn main() {
    #[cfg(feature = "gui")]
    {
        gui::gui::run_gui();
    }

    #[cfg(feature = "cli")]
    {
        cli::cli::run_cli();
    }
}
