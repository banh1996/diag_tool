use log::debug;
use crate::transport;
use crate::executor::parameters::SequenceItem;
use rand::Rng;
use serde_json::Value;
use std::io::{self, Error, ErrorKind};
use utils;


/*****************************************************************************************************************
 *  executor::securityaccess::security_access_volvo function
 *  brief      Function to do security-access S27 for volvo vendor
 *  details    Function execute from start until end of security-access S27 service
 *  \param[in]  stream: point to Diag object
 *              item: read from securityaccess item in sequence json file
 *              level: level of security access
 *              timeout: timeout for security access per request
 *  \param[out] -
 *  \precondition -
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
 pub fn security_access_volvo(mut stream: std::sync::MutexGuard<transport::diag::Diag>,
                              item: SequenceItem, level: u8, timeout: u64)
                              -> Result<(), io::Error> {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    //get parameter of secure-access in action item
    let client_request_seed_message_id_bytes: [u8; 2] = [0x00, 0x01];
    let authentication_method_bytes: [u8; 2] = [0x00, 0x01];
    let mut algorithm = "";
    let mut iv: &str = "";
    let mut encryption_authentication_key = "";
    let mut proof_of_ownership_key = "";
    let mut res_seed_message: Vec<u8> = Vec::new();
    match &item.action {
        Value::Array(multiple_actions) => {
            //get security-access parameters in action item
            for action_str in multiple_actions.iter() {
                if let Some(action) = action_str.as_str() {
                    let parts: Vec<&str> = action.split(':').collect();
                    if parts.len() == 2 {
                        match parts[0] {
                            "algorithm" => algorithm = parts[1],
                            "iv" => iv = parts[1],
                            "encryption_authentication_key" => encryption_authentication_key = parts[1],
                            "proof_of_ownership_key" => proof_of_ownership_key = parts[1],
                            _ => (),
                        }
                    }
                }
            }
        }
        _ => {
            eprintln!("Not enough parameters");
            return Err(Error::new(ErrorKind::InvalidData, "wrong sequence json format"));
        }
    }
    // Convert message_id and authentication_method to big endian bytes
    let specified_bytes: [u8; 6] = [
        0x27,
        level,
        client_request_seed_message_id_bytes[0], // Big endian byte 1 of message_id
        client_request_seed_message_id_bytes[1], // Big endian byte 2 of message_id
        authentication_method_bytes[0], // Big endian byte 1 of authentication_method
        authentication_method_bytes[1], // Big endian byte 2 of authentication_method
    ];
    let mut byte_array: Vec<u8> = Vec::with_capacity(52);
    let mut client_random_number: [u8; 16] = [0; 16];
    byte_array.extend_from_slice(&specified_bytes);
    if algorithm == "AES128" {
        let mut iv_bytes: [u8; 16] = [0; 16];
        if iv == "random" {
            for i in 0..iv_bytes.len() {
                iv_bytes[i] = rng.gen::<u8>();
            }
        }
        //add iv
        byte_array.extend_from_slice(&iv_bytes);

        // Generating client random seed
        for i in 0..client_random_number.len() {
            client_random_number[i] = rng.gen::<u8>();
        }

        //Create Request seed packet
        match utils::excrypto::encrypt_aes128_ctr(&client_random_number, &iv_bytes, encryption_authentication_key) {
            Ok(encrypted_data_record) => {
                let encrypted_first_16_bytes: Vec<u8> = encrypted_data_record.iter().take(16).cloned().collect();
                //add encrypted_data_record
                byte_array.extend_from_slice(&encrypted_first_16_bytes);

                match utils::excrypto::encrypt_aes128_cmac(&byte_array, encryption_authentication_key) {
                    Ok(authentication_data) => {
                        let authentication_data_first_16_bytes: Vec<u8> = authentication_data.iter().take(16).cloned().collect();
                        //add authentication_data
                        byte_array.extend_from_slice(&authentication_data_first_16_bytes);
                    }
                    Err(err) => {
                        eprintln!("Encryption error: {}", err);
                        return Err(err);
                    }
                }
            }
            Err(err) => {
                eprintln!("Encryption error: {}", err);
                return Err(err);
            }
        }
    }

    // Send ClientRequestSeed
    match stream.send_diag(byte_array) {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Failed to send diag Secure access: {}", err);
            return Err(err);
        }
    }
    match &item.expect {
        Value::Array(multiple_expects) => {
            //get security-access parameters in expect item
            for expect_str in multiple_expects.iter() {
                if let Some(expect) = expect_str.as_str() {
                    match stream.receive_diag(timeout) {
                        Ok(data) => {
                            let data_cloned = data.clone();
                            debug!("Sent secure-access, Expect: {}, Receive {:02X?}", expect_str, data);
                            if utils::common::compare_expect_value(expect, data) == false {
                                return Err(Error::new(ErrorKind::InvalidData, "secure-access Diag data received is not expected"));
                            }
                            res_seed_message = Vec::from(data_cloned.as_slice());
                        }
                        Err(err) => return Err(err)
                    }
                }
            }
        }
        _ => {
            eprintln!("Not enough parameters");
            return Err(Error::new(ErrorKind::InvalidData, "wrong sequence json format"));
        }
    }

    // Get the last 16 bytes of the res_seed_message or all elements if it's shorter than 16 bytes
    let server_authentication_code_bytes: Vec<u8> = res_seed_message.iter().rev().take(16).cloned().rev().collect();
    // Get the index before the last 16 bytes
    let last_16_bytes_index = res_seed_message.len().saturating_sub(16);
    let server_payload_bytes: Vec<u8> = res_seed_message[0..last_16_bytes_index].to_vec();
    if utils::excrypto::verify_aes128_cmac(server_payload_bytes.as_slice(), server_authentication_code_bytes, encryption_authentication_key) == false {
        return Err(Error::new(ErrorKind::InvalidData, "SecurityAccess fail authentication"));
    }
    let iv_bytes: Vec<u8> = server_payload_bytes[4..20].to_vec();
    let encrypted_data: Vec<u8> = server_payload_bytes[20..52].to_vec();
    //verity server_proof_of_ownership, use proof_of_ownership_key
    match utils::excrypto::decrypt_aes128_ctr(encrypted_data.as_slice(), &iv_bytes, encryption_authentication_key) {
        Ok(decrypted_data) => {
            let server_random_number: Vec<u8> = decrypted_data[0..16].to_vec();
            let mut random_numbers: Vec<u8> = Vec::new();
            //server_random_number || server_proof_of_ownership
            random_numbers.extend(client_random_number);
            random_numbers.extend(server_random_number);
            let mut rand_iv_bytes: [u8; 16] = [0; 16];
            if iv == "random" {
                for i in 0..rand_iv_bytes.len() {
                    rand_iv_bytes[i] = rng.gen::<u8>();
                }
            }
            //Create Client SendKey message
            let message_id_bytes: Vec<u8> = vec![0x00, 0x03];
            let specified_bytes: [u8; 4] = [
                0x27,
                level+1,
                message_id_bytes[0], // Big endian byte 1 of message_id
                message_id_bytes[1], // Big endian byte 2 of message_id
            ];
            let mut byte_array: Vec<u8> = Vec::with_capacity(50);
            byte_array.extend_from_slice(&specified_bytes);
            byte_array.extend_from_slice(&rand_iv_bytes);
            //calculate client_proof_of_ownership
            let mut client_proof_of_ownership: Vec<u8> = Vec::new();
            match utils::excrypto::encrypt_aes128_cmac(&random_numbers, proof_of_ownership_key) {
                Ok(encrypted_data) => {
                    client_proof_of_ownership.extend(&encrypted_data);
                }
                Err(err) => {
                    eprintln!("Encryption error: {}", err);
                    return Err(err);
                }
            }
            //calculate encrypted_data_record for Client_Send_Key
            match utils::excrypto::encrypt_aes128_ctr(&client_proof_of_ownership, &rand_iv_bytes, encryption_authentication_key) {
                Ok(encrypted_data_record) => {
                    debug!("Secure-access ctr encrypted_data_record: {:?}", encrypted_data_record);
                    //add encrypted_data_record
                    byte_array.extend_from_slice(&encrypted_data_record);
                    match utils::excrypto::encrypt_aes128_cmac(&byte_array, encryption_authentication_key) {
                        Ok(authentication_data) => {
                            let authentication_data_first_16_bytes: Vec<u8> = authentication_data.iter().take(16).cloned().collect();
                            //add authentication_data
                            byte_array.extend_from_slice(&authentication_data_first_16_bytes);

                            // Send Client SendKey
                            match stream.send_diag(byte_array) {
                                Ok(()) => {}
                                Err(err) => {
                                    eprintln!("Failed to send diag Secure access: {}", err);
                                    return Err(err);
                                }
                            }
                            match &item.expect {
                                Value::Array(multiple_expects) => {
                                    //get security-access parameters in expect item
                                    for expect_str in multiple_expects.iter() {
                                        if let Some(expect) = expect_str.as_str() {
                                            match stream.receive_diag(timeout) {
                                                Ok(data) => {
                                                    debug!("Sent secure-access, Expect: {}, Receive {:02X?}", expect_str, data);
                                                    if utils::common::compare_expect_value(expect, data) == false {
                                                        return Err(Error::new(ErrorKind::InvalidData, "secure-access Diag data received is not expected"));
                                                    }
                                                }
                                                Err(err) => {
                                                    return Err(err);
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    eprintln!("Not enough parameters");
                                    return Err(Error::new(ErrorKind::InvalidData, "wrong sequence json format"));
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("Encryption error: {}", err);
                            return Err(err);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Encryption error: {}", err);
                    return Err(err);
                }
            }

        }
        Err(err) => {
            eprintln!("Fail to verify server_proof_of_ownership, Decryption error: {}", err);
            return Err(err);
        }
    }
    Ok(())
}