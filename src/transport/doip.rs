use crate::transport::config::CONFIG;
use crate::transport::soad;

/*****************************************************************************************************************
 *  transport::doip::connect function
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
    soad::connect(ip, port);

    Ok(())
}

/*****************************************************************************************************************
 *  transport::doip::disconnect function
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
    soad::disconnect();

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
    let config = CONFIG.read().unwrap();

    Ok(())
}