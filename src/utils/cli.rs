use log::debug;
use std::sync::{Arc, Mutex};
use std::io::{self, Error, ErrorKind};
use transport::diag::Diag;

use utils;

pub fn parse(diag_obj: Arc<Mutex<Diag>>, input: &str) -> Result<(), io::Error> {
    let timeout: u64 = 10000; //10s
    // Split the input based on ":" and collect the parts into a vector
    let parts: Vec<&str> = input.splitn(2, ':').collect();

    if parts.len() < 2 {
        eprintln!("use format like this send_diag:1001");
        return Err(Error::new(ErrorKind::InvalidInput, "wrong input format"));
    }

    let name = parts[0].trim();
    let action = parts[1].trim();
    let trimmed_action = action.replace(" ", "");
    let mut stream = diag_obj.lock().unwrap();
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
        match stream.receive_diag(timeout) {
            Ok(data) => {debug!("Sent diag data {:02X?}, Receive {:02X?}", clone_vec_action, data)}
            Err(err) => {eprintln!("Failed to receive diag data: {}", err)}
        }
    }
    else if name == "send_doip" {
        match trimmed_action.as_str() {
            "activation" => {
                match stream.send_doip_routing_activation() {
                    Ok(()) => debug!("Send doip successfully!"),
                    Err(err) => {
                        eprintln!("Failed to send doip activation: {}", err);
                        return Err(err);
                    }
                }
                match stream.receive_doip(timeout) {
                    Ok(Some(data)) => debug!("Receive doip data {:?} successfully!", data),
                    Ok(None) => {
                        debug!("Doip activation successfully!");
                    }
                    Err(err) => {
                        eprintln!("Failed to Receive doip activation: {}", err);
                        return Err(err);
                    }
                }
            }
            _ => eprintln!("Invalid send_doip action format: {}", trimmed_action),
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

    Ok(())
}