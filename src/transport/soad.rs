use crate::transport::config::CONFIG;
use log::{debug, info};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

lazy_static::lazy_static! {
    static ref G_IS_INIT_SOCKET: AtomicBool = AtomicBool::new(false);
}

fn handle_stream_data(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}


/*****************************************************************************************************************
 *  transport::soad::listen_for_connections function
 *  brief      Function to listen new tcp connection
 *  details    -
 *  \param[in]  -
 *  \param[out] -
 *  \precondition only valid in case tester role is server
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
fn listen_for_connections(listener: TcpListener) {
    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_stream_data(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
    debug!("Initilizing socket!");
}


/*****************************************************************************************************************
 *  transport::soad::init function
 *  brief      Initialize all needed things for soad layer
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

    // Check if tester role is client, then exit without doing any
    if &config.ethernet.role == "client" {
        if G_IS_INIT_SOCKET.swap(true, Ordering::Relaxed) {
            info!("Already Initilized socket!");
        }
        else {
            debug!("Role is client, Ignore initilize socket!");
        }
        return; //role is client, nothing to do
    }
}


/*****************************************************************************************************************
 *  transport::soad::connect function
 *  brief      Function to establish connection with ECU via tcp
 *  details    If role is client, connect to ECU-server. Otherwise(role is server), bind ip and start to listen
 *             In case role is server, function will return accepted socket object.
 *             In case role is client, function will return connected socket object.
 *  \param[in]  dest_addr:  String of ipv4/ipv6:port
 *                          eg: 192.168.1.3:13400
 *  \param[out] -
 *  \precondition: -
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
pub fn connect(dest_addr: String) -> Result<(), i32> {
    let config = CONFIG.read().unwrap();

    // Check if tester role is client, then call connect cmd to server. or else, start to listen socket
    if &config.ethernet.role == "client" {
        if G_IS_INIT_SOCKET.swap(true, Ordering::Relaxed) {
            debug!("Connecting to server!");
            let stream = TcpStream::connect(dest_addr).expect("Failed to connect to server");

            // Spawn a new thread to handle data reception and detach it.
            thread::spawn(move || {
                handle_stream_data(stream);
            });
        }
        else {
            debug!("Not initilized socket yet!");
            return Err(-1);
        }
    }
    else if &config.ethernet.role == "server" {
        // Listen tcp stream in case tester role is server
        if G_IS_INIT_SOCKET.swap(true, Ordering::Relaxed) {
            debug!("Socket was initialized. Skipping...");
        }
        else {
            // Extract the local IPv4 as a regular String or use an empty string if it's None.
            let local_ipv4 = if let Some(ipv4) = &config.ethernet.local_ipv4 {
                ipv4.to_string()
            } else {
                String::new()
            };
            // Concatenate the local IPv4 and port using the format! macro.
            let server_addr = format!("{}:{}", local_ipv4, config.ethernet.remote_port);

            let listener = TcpListener::bind(server_addr).unwrap();
            debug!("Server listening on port {:?}", &config.ethernet.remote_port);

            // Spawn a new thread for listening to incoming connections.
            thread::spawn(move || {
                // Call the listen_for_connections function to handle incoming connections.
                listen_for_connections(listener);
            });
        }
    }
    Ok(())
}


/*****************************************************************************************************************
 *  transport::soad::disconnect function
 *  brief      Disonnect to ECU server via tcp
 *  details    -
 *  \param[in]  -
 *  \param[out] -
 *  \precondition: -
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
pub fn disconnect() -> Result<(), i32> {
    //TODO
    //let config = CONFIG.read().unwrap();

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
    //let config = CONFIG.read().unwrap();
    // Print the data in string format
    let data_as_string: String = p_data.iter().map(|&x| x as u8 as char).collect();
    println!("Data as string: {}", data_as_string);

    // Print the data in hexadecimal format
    let data_as_hex: String = p_data
        .iter()
        .map(|&x| format!("{:02X}", x as u8))
        .collect();
    println!("Data as hex: {}", data_as_hex);

    Ok(())
}