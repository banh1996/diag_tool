//use crate::transport::config::CONFIG;
use crate::transport::soad;
use std::io;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

/*****************************************************************************************************************
 *  transport::doip::init function
 *  brief      Initialize doip module
 *  details    -
 *  \param[in]  -
 *  \param[out] -
 *  \precondition -
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
pub fn init() {
    //let config = CONFIG.read().unwrap();

    soad::init();

    // initialize socket
}


/*****************************************************************************************************************
 *  transport::doip::connect function
 *  brief      Connect to ECU server via tcp
 *  details    -
 *  \param[in]  dest_addr:  String of ipv4/ipv6:port
 *                          eg: 192.168.1.3:13400
 *  \param[out] -
 *  \precondition: role must be client
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
 pub fn connect(dest_addr: String) -> Result<Arc<Mutex<TcpStream>>, io::Error> {
    match soad::connect(dest_addr) {
        Ok(stream) => {
            // Wrap the stream and cvar in Arc and return both in a tuple
            Ok(stream)
        }
        Err(e) => {
            // Handle the error. You can print an error message or take other actions as needed.
            eprintln!("Failed to connect: {}", e);
            Err(e) // Propagate the error back to the caller.
        }
    }
}

/*****************************************************************************************************************
 *  transport::doip::disconnect function
 *  brief      Disonnect to ECU server via tcp
 *  details    -
 *  \param[in]  -
 *  \param[out] -
 *  \precondition: -
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
pub fn disconnect(stream: &Arc<Mutex<TcpStream>>) -> Result<(), io::Error> {
    if let Err(err) = soad::disconnect(stream) {
        eprintln!("doip disconnect Error: {}", err);
        return Err(err);
    }

    Ok(())
}

/*****************************************************************************************************************
 *  transport::doip::send_doip function
 *  brief      Function to send doip data to ECU
 *  details    -
 *  \param[in]  p_data:  refer to data array
 *  \param[out] -
 *  \precondition: Establish TCP connection successfully
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
pub fn send_doip(stream: &Arc<Mutex<TcpStream>>, p_data: Vec<u8>) -> Result<(), io::Error> {
    //TODO: add doip header
    //let config = CONFIG.read().unwrap();
    if let Err(err) = soad::send_tcp(stream, p_data) {
        eprintln!("send_doip Error: {}", err);
        return Err(err);
    }

    Ok(())
}


/*****************************************************************************************************************
 *  transport::doip::receive_tcp function
 *  brief      Function to receive doip data to ECU
 *  details    -
 *  \param[in]  p_data:  refer to data array
 *  \param[out] -
 *  \precondition: Establish TCP connection successfully
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
 pub fn receive_doip(stream: &Arc<Mutex<TcpStream>>, timeout: u64) -> Result<Vec<u8>, io::Error> {
    //TODO: add doip header
    //let config = CONFIG.read().unwrap();
    match soad::receive_tcp(stream, timeout) {
        Ok(data) => {
            // Process the received data
            Ok(data)
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}