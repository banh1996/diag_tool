use crate::transport::config::CONFIG;
use log::debug;
use std::sync::atomic::{AtomicBool, Ordering};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::io::{Error, ErrorKind};

/*****************************************************************************************************************
 *  Define all gloval macro & variable here
 ****************************************************************************************************************/
lazy_static::lazy_static! {
    static ref G_IS_INIT_SOCKET: AtomicBool = AtomicBool::new(false);
    static ref RECEIVE_TIMEOUT: usize = 100; //default
}
const BUFFER_SIZE: usize = 4100; //default
/* end define */


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
    G_IS_INIT_SOCKET.store(true, Ordering::Relaxed);
    // Check if tester role is client, then exit without doing any
    if &config.ethernet.role == "client" {
        debug!("Role is client, Ignore initilize server socket!");
    }
} /* init */


/*****************************************************************************************************************
 *  transport::soad::connect function
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
    let config = CONFIG.read().unwrap();

    // Check if tester role is client, then call connect cmd to server. or else, start to listen socket
    if &config.ethernet.role == "client" {
        if G_IS_INIT_SOCKET.load(Ordering::Relaxed) == false {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "socket is not initilized yet!",
            ));
        }

        debug!("Connecting to server! {}", dest_addr);
        let stream = TcpStream::connect(dest_addr)?;

        // Create an Arc wrapping the TcpStream to share ownership between threads
        // I keep this in case we need to add more feature here
        let shared_stream = Arc::new(Mutex::new(stream));

        // Clone the Arc to be moved into the sender thread
        //let receiver_stream = Arc::clone(&shared_stream);

        // Spawn a new thread to handle data reception and detach it.
        // thread::spawn(move || {
        //     // Access the TcpStream through the Mutex in the closure
        //     //let mut stream = clone_stream.lock().unwrap();
        //     handle_stream_data(&receiver_stream, &receiver_cvar);
        // });

        // Return the original stream outside the closure
        Ok(shared_stream)
    }
    else if &config.ethernet.role == "server" {
        // Listen tcp stream in case tester role is server
        if G_IS_INIT_SOCKET.load(Ordering::Relaxed) == false {
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

            // accept connections and process them, spawning a new thread for each one if needed
            let stream = listener.incoming().next().unwrap()?;
            debug!("New connection coming: {}", stream.peer_addr().unwrap());

            // Create an Arc wrapping the TcpStream to share ownership between threads
            // I keep this in case we need to add more feature here
            let shared_stream = Arc::new(Mutex::new(stream));

            // Clone the Arc to be moved into the sender thread
            //let receiver_stream = Arc::clone(&shared_stream);

            // Spawn a new thread to handle data reception and detach it.
            // thread::spawn(move || {
            //     // Access the TcpStream through the Mutex in the closure
            //     //let mut stream = clone_stream.lock().unwrap();
            //     handle_stream_data(&receiver_stream);
            // });

            // close the socket server if needed
            drop(listener);
            debug!("Connected socket successfully!");

            Ok(shared_stream)
        } else {
            debug!("Socket was already initialized. Skipping...");
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Socket was already initialized",
            ))
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Invalid tester role",
        ))
    }
}
/* connect */


/*****************************************************************************************************************
 *  transport::soad::disconnect function
 *  brief        Disonnect to ECU server via tcp
 *  details      -
 *  \param[in]   stream: TcpStream that used with mutex to prevent race condition when sending/reading data
 *  \param[out]  -
 *  \precondition: -
 *  \reentrant:  FALSE
 *  \return      Error code if any
 ****************************************************************************************************************/
pub fn disconnect(stream: &Arc<Mutex<TcpStream>>) -> Result<(), io::Error> {
    // Lock the stream for access
    let stream = stream.lock().unwrap();

    // Shutdown the TcpStream
    if let Err(err) = stream.shutdown(Shutdown::Both) {
        // Handle the error. You can print an error message or take other actions as needed.
        eprintln!("Failed to shutdown TcpStream: {}", err);
        return Err(err); // Propagate the error back to the caller.
    }

    Ok(())
}
/* disconnect */


/*****************************************************************************************************************
 *  transport::soad::send_tcp function
 *  brief      Function to send tcp data to ECU
 *  details    -
 *  \param[in]  stream: TcpStream that used with mutex to prevent race condition when sending/reading data
 *              p_data: refer to data array
 *  \param[out] -
 *  \precondition: Establish TCP connection successfully
 *  \reentrant:  FALSE
 *  \return     Error code if any
 ****************************************************************************************************************/
pub fn send_tcp(stream: &Arc<Mutex<TcpStream>>, p_data: Vec<u8>) -> Result<(), io::Error> {
    // Check if the socket is connected before sending data
    if G_IS_INIT_SOCKET.load(Ordering::Relaxed) == false {
        eprint!("Not initialized yet!");
        return Err(io::Error::new(io::ErrorKind::NotConnected, "Socket is not connected"));
    }

    let mut stream_lock = stream.lock().unwrap(); //lock mutex to send tcp data

    // Check if the socket is still open before sending data
    if let Err(e) = stream_lock.write_all(&p_data) {
        if e.kind() == io::ErrorKind::BrokenPipe {
            return Err(io::Error::new(io::ErrorKind::NotConnected, "Socket is not connected"));
        } else {
            return Err(e);
        }
    }

    Ok(())
}
/* send_tcp */


/*****************************************************************************************************************
 *  transport::soad::receive_tcp function
 *  brief      Function to receive tcp data to ECU
 *  details    -
 *  \param[in]  stream: TcpStream that used with mutex to prevent race condition when sending/reading data
 *              timeout: timeout(milliseconds) to wait for new doip data. If there's no data, return error
 *  \param[out] -
 *  \precondition: Establish TCP connection successfully
 *  \reentrant:  FALSE
 *  \return     Vec contains received data
 *              Error code if any
 ****************************************************************************************************************/
pub fn receive_tcp(stream: &Arc<Mutex<TcpStream>>, timeout: u64) -> Result<Vec<u8>, io::Error> {
    // Check if the socket is connected before sending data
    if G_IS_INIT_SOCKET.load(Ordering::Relaxed) == false {
        eprint!("Not initialized yet!");
        return Err(io::Error::new(io::ErrorKind::NotConnected, "Socket is not connected"));
    }

    //lock mutex to send tcp data
    let mut stream_lock = stream.lock().unwrap();

    // Set a read timeout
    if let Err(e) = stream_lock.set_read_timeout(Some(Duration::from_millis(timeout))) {
        if e.kind() == ErrorKind::WouldBlock {
            // Timeout: Cannot set read timeout on non-blocking socket
            return Err(Error::new(ErrorKind::TimedOut, "Timeout: Cannot set read timeout on non-blocking socket"));
        } else {
            return Err(e);
        }
    }

    // Create a buffer to store the received data
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut received_data = Vec::new();

    // Attempt to read from the socket
    match stream_lock.read(&mut buffer) {
        Ok(0) => Err(Error::new(ErrorKind::ConnectionAborted, "Connection closed by peer")),
        Ok(n) => {
            // Data received successfully
            received_data.extend_from_slice(&buffer[..n]);
            Ok(received_data)
        }
        Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
            // Timeout: No data received within the specified timeout
            return Err(Error::new(ErrorKind::TimedOut, "Timeout: No data received within the specified timeout"));
        }
        Err(e) => return Err(e),
    }
}
/* read_tcp */