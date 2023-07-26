use crate::transport::config::CONFIG;
use log::{debug};
use std::sync::atomic::{AtomicBool, Ordering};
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

lazy_static::lazy_static! {
    static ref g_init_socket: AtomicBool = AtomicBool::new(false);
}

/*****************************************************************************************************************
 *  transport::soad::init function
 *  brief      If role is client, skip. Otherwise(server), bind and start to listen
 *  details    -
 *  \param[in]  config_filename:  path to config json file
 *  \param[out] -
 *  \precondition -
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
pub fn init() -> Result<(), i32> {
    let config = CONFIG.read().unwrap();
    println!("ethernet: {:?}", &config.ethernet.local_ipv4);
    println!("doip: {:?}", &config.doip);

    // Check if role is server
    if &config.ethernet.role == "client" {
        return Err(-1); //role is client, nothing to do
    }

    //TODO: init server socket

    Ok(())
}

/*****************************************************************************************************************
 *  transport::soad::connect function
 *  brief      Connect to ECU server via tcp
 *  details    -
 *  \param[in]  ip:  String of ipv4/ipv6
 *  \param[in]  port:  port number
 *  \param[out] -
 *  \precondition: role must be client
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/

pub fn connect(ip: String, port: i32) -> Result<(), i32> {
    //TODO: check socket is created
    if g_init_socket.swap(true, Ordering::Relaxed) {
        debug!("Socket was initialized. Skipping...");
    }
    else {
        debug!("Initilizing socket");
    }

    //get config param
    let config = CONFIG.read().unwrap();

    //TODO: connect to server

    Ok(())
}

/*****************************************************************************************************************
 *  transport::soad::disconnect function
 *  brief      Disonnect to ECU server via tcp
 *  details    -
 *  \param[in]  config_filename:  path to config json file
 *  \param[out] -
 *  \precondition: tester role must be client
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
pub fn disconnect() -> Result<(), i32> {
    //TODO
    let config = CONFIG.read().unwrap();

    Ok(())
}

/*****************************************************************************************************************
 *  transport::soad::send_tcp function
 *  brief      Function to send tcp data to ECU
 *  details    -
 *  \param[in]  p_data:  refer to data array
 *  \param[out] -
 *  \precondition: Establish TCP connection successfully
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
 pub fn send_tcp(p_data: &[i8]) -> Result<(), i32> {
    //TODO
    let config = CONFIG.read().unwrap();

    Ok(())
}