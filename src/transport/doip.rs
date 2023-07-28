use crate::transport::config::CONFIG;
use crate::transport::soad;


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
    let config = CONFIG.read().unwrap();
    println!("ethernet: {:?}", &config.ethernet.local_ipv4);
    println!("doip: {:?}", &config.doip);

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
pub fn connect(dest_addr: String) -> Result<(), i32> {
    if let Err(err) = soad::connect(dest_addr) {
        eprintln!("doip connect Error: {}", err);
        return Err(err);
    }

    Ok(())
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
pub fn disconnect() -> Result<(), i32> {
    //let config = CONFIG.read().unwrap();
    if let Err(err) = soad::disconnect() {
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
 pub fn send_doip(p_data: &[i8]) -> Result<(), i32> {
    //TODO
    //let config = CONFIG.read().unwrap();
    if let Err(err) = soad::send_tcp(p_data) {
        eprintln!("send_doip Error: {}", err);
        return Err(err);
    }

    Ok(())
}