use crate::transport::config::CONFIG;
use crate::transport::doip;



/*****************************************************************************************************************
 *  transport::diag::init function
 *  brief      Initialize doip module
 *  details    -
 *  \param[in]  -
 *  \param[out] -
 *  \precondition -
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
pub fn init() {
    let config = CONFIG.read().unwrap();
    println!("ethernet: {:?}", &config.ethernet.local_ipv4);
    println!("doip: {:?}", &config.doip);

    // initialize
    doip::init();
}


/*****************************************************************************************************************
 *  transport::diag::connect function
 *  brief      Connect to ECU server via tcp
 *  details    -
 *  \param[in]  dest_addr:  String of ipv4/ipv6:port
 *                          eg: 192.168.1.3:13400
 *  \param[out] -
 *  \precondition: role must be client
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
pub fn connect() -> Result<(), i32> {
    let config = CONFIG.read().unwrap();
    // Extract the local IPv4 as a regular String or use an empty string if it's None.
    let local_ipv4 = if let Some(ipv4) = &config.ethernet.local_ipv4 {
        ipv4.to_string()
    } else {
        String::new()
    };
    // Concatenate the local IPv4 and port using the format! macro.
    let server_addr = format!("{}:{}", local_ipv4, config.ethernet.remote_port);

    if let Err(err) = doip::connect(server_addr) {
        eprintln!("diag connect Error: {}", err);
        return Err(err);
    }

    Ok(())
}


/*****************************************************************************************************************
 *  transport::diag::disconnect function
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
    if let Err(err) = doip::disconnect() {
        eprintln!("diag disconnect Error: {}", err);
        return Err(err);
    }

    Ok(())
}


/*****************************************************************************************************************
 *  transport::diag::send_diag function
 *  brief      Function to send uds data to ECU
 *  details    -
 *  \param[in]  p_data:  refer to data array
 *  \param[out] -
 *  \precondition: Activate doip successfully
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
pub fn send_diag(p_data: &[i8]) -> Result<(), i32> {
    //TODO
    if let Err(err) = doip::send_doip(p_data) {
        eprintln!("send_doip Error: {}", err);
        return Err(err);
    }

    Ok(())
}