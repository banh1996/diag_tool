use crate::transport::config::CONFIG;
use crate::transport::doip;
use std::io::{self, Error, ErrorKind};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use log::debug;

// Define the Diag trait
pub trait Transport {
    fn init();
    fn connect(&mut self) -> Result<(), io::Error>;
    fn disconnect(&mut self) -> Result<(), io::Error>;
    fn send_diag(&mut self, p_data: Vec<u8>) -> Result<(), io::Error>;
    fn receive_diag(&mut self, timeout: u64) -> Result<Vec<u8>, io::Error>;
}

pub struct Diag {
    stream: Option<Arc<Mutex<TcpStream>>>,
}

// Implement the Diag trait for the Diag struct
impl Transport for Diag {
    fn init() {
        Diag::init();
    }

    fn connect(&mut self) -> Result<(), io::Error> {
        self.connect()
    }

    fn disconnect(&mut self) -> Result<(), io::Error> {
        self.disconnect()
    }

    fn send_diag(&mut self, p_data: Vec<u8>) -> Result<(), io::Error> {
        self.send_diag(p_data)
    }

    fn receive_diag(&mut self, timeout: u64) -> Result<Vec<u8>, io::Error> {
        self.receive_diag(timeout)
    }
}

impl Diag {
/*****************************************************************************************************************
 *  transport::diag::init function
 *  brief      Initialize diag module
 *  details    -
 *  \param[in]  -
 *  \param[out] -
 *  \precondition -
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
pub fn init() {
    let config = CONFIG.read().unwrap();
    debug!("ethernet: {:?}", &config.ethernet.local_ipv4);
    debug!("doip: {:?}", &config.doip);

    // initialize
    doip::init();
}


/*****************************************************************************************************************
 *  transport::diag::connect function
 *  brief       Function to establish connection with ECU via tcp
 *  details     If role is client, connect to ECU-server. Otherwise(role is server), bind ip and start to listen
 *              In case role is server, function will return accepted socket object.
 *              In case role is client, function will return connected socket object.
 *  \param[in]  -
 *  \param[out] -
 *  \precondition: -
 *  \reentrant: FALSE
 *  \return:    Error code if any
 ****************************************************************************************************************/
pub fn connect(&mut self) -> Result<(), io::Error> {
    let config = CONFIG.read().unwrap();
    // Extract the local IPv4 as a regular String or use an empty string if it's None.
    let local_ipv4 = if let Some(ipv4) = &config.ethernet.local_ipv4 {
        ipv4.to_string()
    } else {
        String::new()
    };
    // Concatenate the local IPv4 and port using the format! macro.
    let server_addr = format!("{}:{}", local_ipv4, config.ethernet.remote_port);

    match doip::connect(server_addr) {
        Ok(stream) => {
            // TODO
            self.stream = Some(stream); //transfer stream ownership to self.stream
            Ok(())
        }
        Err(e) => {
            // Handle the error. You can print an error message or take other actions as needed.
            eprintln!("Failed to connect: {}", e);
            Err(e) // Propagate the error back to the caller.
        }
    }
}


/*****************************************************************************************************************
 *  transport::diag::disconnect function
 *  brief        Disonnect to ECU server via tcp
 *  details      -
 *  \param[in]   -
 *  \param[out]  -
 *  \precondition: -
 *  \reentrant:  FALSE
 *  \return      Error code if any
 ****************************************************************************************************************/
pub fn disconnect(&mut self) -> Result<(), io::Error> {
    //TODO
    //let config = CONFIG.read().unwrap();
    match &mut self.stream {
        Some(stream) => {
            //drop tcp stream
            if let Err(e) = doip::disconnect(stream) {
                return Err(e);
            }
            Ok(())
        }
        None => Err(io::Error::new(
            io::ErrorKind::NotConnected,
            "Not connected to any server",
        )),
    }
}


/*****************************************************************************************************************
 *  transport::diag::send_diag function
 *  brief      Function to send diag data to ECU
 *  details    -
 *  \param[in]  p_data: refer to data array
 *  \param[out] -
 *  \precondition: Establish TCP connection successfully
 *  \reentrant:  FALSE
 *  \return     Error code if any
 ****************************************************************************************************************/
pub fn send_diag(&mut self, p_data: Vec<u8>) -> Result<(), io::Error> {
    //TODO add diag header
    match &mut self.stream {
        Some(stream) => {
            //drop tcp stream
            if let Err(e) = doip::send_doip_diag(stream, p_data) {
                return Err(e);
            }
            Ok(())
        }
        None => Err(io::Error::new(
            io::ErrorKind::NotConnected,
            "Not connected to any server",
        )),
    }
}


/*****************************************************************************************************************
 *  transport::diag::receive_diag function
 *  brief      Function to receive diag data to ECU
 *  details    -
 *  \param[in]  timeout: timeout to wait for new diag data. If there's no data, return error
 *  \param[out] -
 *  \precondition: Establish TCP connection successfully
 *  \reentrant:  FALSE
 *  \return     Vec contains received data
 *              Error code if any
 ****************************************************************************************************************/
pub fn receive_diag(&mut self, timeout: u64) -> Result<Vec<u8>, io::Error> {
    //TODO: add diag header
    //let config = CONFIG.read().unwrap();
    match &mut self.stream {
        Some(stream) => {
            //drop tcp stream
            match doip::receive_doip(stream, timeout) {
                Ok(Some(data)) => {
                    // Process the received data
                    debug!("Received {} bytes: {:?}", data.len(), data);
                    Ok(data)
                },
                Ok(None) => {
                    Err(Error::new(ErrorKind::InvalidData, "No any diag payload found"))
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    Err(e)
                }
            }
        }
        None => Err(io::Error::new(
            io::ErrorKind::NotConnected,
            "Not connected to any server",
        )),
    }
}

} //end imp Transport


// Public function that returns a new Diag object
pub fn create_diag() -> Diag {
    Diag::init();

    Diag {
        stream: None, // Initialize the stream field to None
    }
}