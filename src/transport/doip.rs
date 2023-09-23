//TODO: full compliance for ISO13400-1

use crate::transport::config::CONFIG;
use crate::transport::soad;
use std::io::{self, Error, ErrorKind};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

/*****************************************************************************************************************
 *  Define all gloval macro & variable here
 ****************************************************************************************************************/
lazy_static::lazy_static! {
    static ref G_IS_ROUTING_SUCCESS: AtomicBool = AtomicBool::new(false); // Initial value
}


/* define all global struct and variable here */
#[derive(Debug)]
struct DoipHeader {
    version: u8,
    inverse_version: u8,
    type_field: u16,
    length: u32,
}


// construct the DoIPHeader to bytes
fn construct_doip_header(type_field: u16, length: u32) -> Result<Vec<u8>, io::Error> {
    let config = CONFIG.read().unwrap();

    let header = DoipHeader {
        version: config.doip.version,
        inverse_version: config.doip.inverse_version,
        type_field: type_field as u16,
        length: length as u32,
    };

    let mut header_bytes = Vec::new();
    header_bytes.push(header.version as u8);
    header_bytes.push(header.inverse_version as u8);
    header_bytes.extend_from_slice(&header.type_field.to_be_bytes());
    header_bytes.extend_from_slice(&header.length.to_be_bytes());

    // Return the serialized bytes
    Ok(header_bytes)
}


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
 *  brief       Function to establish connection with ECU via tcp
 *  details     If role is client, connect to ECU-server. Otherwise(role is server), bind ip and start to listen
 *              In case role is server, function will return accepted socket object.
 *              In case role is client, function will return connected socket object.
 *  \param[in]  dest_addr:  String of ipv4/ipv6:port
 *                          eg: "192.168.1.3:13400"
 *  \param[out] -
 *  \precondition: -
 *  \reentrant: FALSE
 *  \return:    TcpStream object after establishedconnection
 *              Error code if any
 ****************************************************************************************************************/
 pub fn connect(dest_addr: String) -> Result<Arc<Mutex<TcpStream>>, io::Error> {
    match soad::connect(dest_addr) {
        Ok(stream) => {
            G_IS_ROUTING_SUCCESS.store(false, Ordering::Relaxed);
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
 *  brief        Disonnect to ECU server via tcp
 *  details      -
 *  \param[in]   stream: TcpStream that used with mutex to prevent race condition when sending/reading data
 *  \param[out]  -
 *  \precondition: -
 *  \reentrant:  FALSE
 *  \return      Error code if any
 ****************************************************************************************************************/
pub fn disconnect(stream: &Arc<Mutex<TcpStream>>) -> Result<(), io::Error> {
    G_IS_ROUTING_SUCCESS.store(false, Ordering::Relaxed);
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
 *  \param[in]  stream: TcpStream that used with mutex to prevent race condition when sending/reading data
 *              p_data: doip payload
 *              type_field: type of doip (e.g: 0x8001)
 *  \param[out] -
 *  \precondition: Establish TCP connection successfully
 *  \reentrant:  FALSE
 *  \return     Error code if any
 ****************************************************************************************************************/
pub fn send_doip(stream: &Arc<Mutex<TcpStream>>, p_data: Vec<u8>, type_field: u16) -> Result<(), io::Error> {
    let config = CONFIG.read().unwrap();

    let mut doip_header_bytes: Vec<u8>;

    // Check type field to append address
    match type_field {
        0x8001 => { //diagnostic message request
            // Get the DoIPHeader to a Vec<u8>
            let doip_header = construct_doip_header(type_field, 4 + p_data.len() as u32);

            // Handle the doip_header_bytes Result and get the Vec<u8> value
            doip_header_bytes = match doip_header {
                Ok(bytes) => bytes,
                Err(e) => {
                    // Handle the error if necessary
                    return Err(e);
                }
            };

            //Add tester&ECU addr to doip payload
            doip_header_bytes.extend_from_slice(&config.doip.tester_addr.to_be_bytes());
            doip_header_bytes.extend_from_slice(&config.doip.ecu_addr.to_be_bytes());
        },
        0x0005 => { // Routing activation request
            // Get the DoIPHeader to a Vec<u8>
            let doip_header = construct_doip_header(type_field, 2 + p_data.len() as u32);

            // Handle the doip_header_bytes Result and get the Vec<u8> value
            doip_header_bytes = match doip_header {
                Ok(bytes) => bytes,
                Err(e) => {
                    // Handle the error if necessary
                    return Err(e);
                }
            };
            doip_header_bytes.extend_from_slice(&config.doip.tester_addr.to_be_bytes());
        },
        _ => {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid type field at sending doip layer"));
        }
    }

    // Combine the DoIP header with the original p_data
    let mut combined_data = Vec::new();
    combined_data.extend_from_slice(&doip_header_bytes);
    combined_data.extend_from_slice(&p_data);

    if let Err(err) = soad::send_tcp(stream, combined_data) {
        eprintln!("send_doip Error: {}", err);
        return Err(err);
    }

    Ok(())
}


/*****************************************************************************************************************
 *  transport::doip::send_doip_diag function
 *  brief      Function to send doip data that from diag layer request to ECU
 *  details    -
 *  \param[in]  stream: TcpStream that used with mutex to prevent race condition when sending/reading data
 *              p_data: doip payload
 *  \param[out] -
 *  \precondition: Establish TCP connection successfully
 *  \reentrant:  FALSE
 *  \return     Error code if any
 ****************************************************************************************************************/
pub fn send_doip_diag(stream: &Arc<Mutex<TcpStream>>, p_data: Vec<u8>) -> Result<(), io::Error> {
    if !G_IS_ROUTING_SUCCESS.load(Ordering::Relaxed) {
        return Err(Error::new(ErrorKind::WouldBlock, "Do activation routing before send diag messages!"));
    }

    if let Err(e) = send_doip(stream, p_data, 0x8100) {
        eprintln!("send_doip_diag Error: {}", e);
        return Err(e);
    }

    Ok(())
}


/*****************************************************************************************************************
 *  transport::doip::send_doip_routing_activation function
 *  brief      Function to send doip activate routing request to ECU
 *  details    -
 *  \param[in]  stream: TcpStream that used with mutex to prevent race condition when sending/reading data
 *  \param[out] -
 *  \precondition: Establish TCP connection successfully
 *  \reentrant:  FALSE
 *  \return     Error code if any
 ****************************************************************************************************************/
pub fn send_doip_routing_activation(stream: &Arc<Mutex<TcpStream>>) -> Result<(), io::Error> {
    let config = CONFIG.read().unwrap();
    let mut p_data = Vec::new();
    p_data.push(config.doip.activation_code);

    // Add four bytes with value 0x00 reserved for ISO to the end of the vector in one line
    p_data.extend(std::iter::repeat(0x00).take(4));
    if let Err(e) = send_doip(stream, p_data, 0x0005) {
        eprintln!("send_doip_diag Error: {}", e);
        return Err(e);
    }

    Ok(())
}


/*****************************************************************************************************************
 *  transport::doip::receive_doip function
 *  brief      Function to receive doip data to ECU
 *  details    -
 *  \param[in]  stream: TcpStream that used with mutex to prevent race condition when sending/reading data
 *              timeout: timeout(milliseconds) to wait for new doip data. If there's no data, return error
 *  \param[out] -
 *  \precondition: Establish TCP connection successfully
 *  \reentrant:  FALSE
 *  \return     Vec contains received data
 *              Error code if any
 ****************************************************************************************************************/
pub fn receive_doip(stream: &Arc<Mutex<TcpStream>>, timeout: u64) -> Result<Option<Vec<u8>>, io::Error> {
    let config = CONFIG.read().unwrap();

    loop {
        match soad::receive_tcp(stream, timeout) {
            Ok(data) => {
                // parse doip header
                // check length
                if data.len() < 8 {
                    return Err(Error::new(ErrorKind::InvalidData, "DoIp fragment invalid"));
                }
                // Separate the data into header and payload using split_at
                let (header_bytes, payload_bytes) = data.split_at(8);
                // Convert header_bytes to Vec<u8>
                let payload: Vec<u8> = payload_bytes.to_vec();
                // Convert header_bytes to DoipHeader struct
                let header = DoipHeader {
                    version: header_bytes[0],
                    inverse_version: header_bytes[1],
                    type_field: u16::from_be_bytes([header_bytes[2], header_bytes[3]]),
                    length: u32::from_be_bytes([
                        header_bytes[4],
                        header_bytes[5],
                        header_bytes[6],
                        header_bytes[7],
                    ]),
                };

                if header.length != payload.len() as u32 {
                    return Err(Error::new(ErrorKind::InvalidData, "DoIp Length invalid"));
                }

                // check version doip
                if header.version != config.doip.version ||
                   header.inverse_version != config.doip.inverse_version {
                    continue;
                }

                // check type doip header
                match header.type_field {
                    0x8001 => { //diagnostic message reply, forward doip payload to diag layer
                        // Make sure diag payload exist
                        if payload.len() < 4 { //4 bytes for tester-ecu address
                            return Err(Error::new(ErrorKind::InvalidData, "Diag Length invalid"));
                        }

                        let (addresses_bytes, diag_payload_bytes) = payload.split_at(4);
                        let diag_payload: Vec<u8> = diag_payload_bytes.to_vec();
                        // Check addresses matches with config
                        if u16::from_be_bytes([addresses_bytes[0], addresses_bytes[1]]) & config.doip.ecu_addr
                            != config.doip.ecu_addr {
                            continue;
                        }
                        if u16::from_be_bytes([addresses_bytes[2], addresses_bytes[3]]) & config.doip.tester_addr
                            != config.doip.tester_addr {
                            continue;
                        }
                        return Ok(Some(diag_payload));
                    },
                    0x8002 => { //DoIP message ACK, ignore
                        continue;
                    },
                    0x0006 => { // Routing activation successful
                        G_IS_ROUTING_SUCCESS.store(true, Ordering::Relaxed);
                        return Ok(None);
                    },
                    _ => {
                        continue;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err(e);
            }
        }
    }
}