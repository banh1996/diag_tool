use crate::transport::config::CONFIG;
use crate::transport::doip;

/*****************************************************************************************************************
 *  transport::soad::init function
 *  brief      Parse json file to get config parameters
 *  details    -
 *  \param[in]  config_filename:  path to config json file
 *  \param[out] -
 *  \precondition -
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
pub fn init() {
    let config = CONFIG.read().unwrap();
    println!("ethernet: {:?}", &config.ethernet.local_ipv4);
    println!("doip: {:?}", &config.doip);

    // initialize socket
}


/*****************************************************************************************************************
 *  transport::diag::connect function
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
    let config = CONFIG.read().unwrap();
    doip::connect(ip, port);

    Ok(())
}


/*****************************************************************************************************************
 *  transport::diag::disconnect function
 *  brief      Disonnect to ECU server via tcp
 *  details    -
 *  \param[in]  -
 *  \param[out] -
 *  \precondition: tester role must be client
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
pub fn disconnect() -> Result<(), i32> {
    //TODO
    let config = CONFIG.read().unwrap();
    doip::disconnect();

    Ok(())
}


/*****************************************************************************************************************
 *  transport::diag::send_uds function
 *  brief      Function to send uds data to ECU
 *  details    -
 *  \param[in]  p_data:  refer to data array
 *  \param[out] -
 *  \precondition: Activate doip successfully
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
 pub fn send_uds(p_data: &[i8]) -> Result<(), i32> {
    //TODO

    Ok(())
}