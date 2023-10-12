use std::io::{self, Read, Error, ErrorKind};
use crate::transport;
use log::debug;
use std::fs::File;

use crate::utils;

fn extract_erase_values(erase_content: &str) -> (String, String) {
    let mut erase_start_addr = String::new();
    let mut erase_length_addr = String::new();
    for lines in erase_content.lines()
    {
        let start_addr_parts: Vec<&str> = lines
            .split(',')
            .map(|s| s.trim())
            .collect();
        if start_addr_parts.len() == 2 {
            erase_start_addr = start_addr_parts[0]
                .trim_matches(&['{', '}', ' ', '\n', '\t', '\r', '\x0c', ';'][..])
                //.trim_start_matches("0x")
                .to_string();
            erase_length_addr = start_addr_parts[1]
                .trim_matches(&['{', '}', ' ', '\n', '\t', '\r', '\x0c', ';'][..])
                //.trim_start_matches("0x")
                .to_string();
        }
    }
    (erase_start_addr, erase_length_addr)
}

fn extract_value(contents: &str, field: &str) -> String {
    if let Some(start_idx) = contents.find(field) {
        let start_idx = start_idx + field.len();
        if let Some(end_idx) = contents[start_idx..].find(';') {
            let mut value = contents[start_idx..start_idx + end_idx].trim();
            if value.starts_with('=') {
                // Handling cases like 'field = value'
                value = &value[1..];
            }
            // Trim quotation marks, semicolons, and whitespace
            value.trim_matches(&['"', ' '][..]).to_string()
        } else {
            "".to_string()
        }
    } else {
        "".to_string()
    }
}

/*****************************************************************************************************************
 *  utils::parse function
 *  brief      Parse vbf swdl file to get header parameters
 *  details    -
 *  \param[in]  sw_filename  path to swdl file
 *  \param[out] -
 *  \precondition -
 *  \reentrant:  FALSE
 *  \return -
 ****************************************************************************************************************/
pub fn parse_vbf(mut stream: std::sync::MutexGuard<transport::diag::Diag>,
                sw_filename: String, max_buffer_len: u32, timeout: u64) -> Result<(), io::Error> {
    // Open the file and read its content
    let sw_filename_clone = sw_filename.clone();
    let mut file = match File::open(sw_filename) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    let mut header_content = String::new();
    let mut brace_count = 0;
    let mut inside_header = false;

    loop {
        let mut byte = [0; 1];
        if file.read_exact(&mut byte).is_err() {
            break; // Error or end of file
        }

        let character = byte[0] as char;
        header_content.push(character);

        if inside_header {
            if character == '{' {
                brace_count += 1;
            } else if character == '}' {
                brace_count -= 1;
                if brace_count == 0 {
                    break; // End of header block
                }
            }
        } else if character == '{' {
            inside_header = true;
            brace_count += 1;
        }
    }

    // Parse the content and extract fields from the header
    let sw_part_number = extract_value(&header_content, "sw_part_number");
    let sw_version = extract_value(&header_content, "sw_version");
    let sw_part_type = extract_value(&header_content, "sw_part_type");
    let ecu_address = extract_value(&header_content, "ecu_address");
    let data_format_identifier = extract_value(&header_content, "data_format_identifier");
    let erase = extract_value(&header_content, "erase");
    let (erase_start_addr, erase_length_addr) = extract_erase_values(&erase);
    let verification_block_start = extract_value(&header_content, "verification_block_start");
    let verification_block_length = extract_value(&header_content, "verification_block_length");
    let verification_block_root_hash = extract_value(&header_content, "verification_block_root_hash");
    let sw_signature_dev = extract_value(&header_content, "sw_signature_dev");
    let file_checksum = extract_value(&header_content, "file_checksum");

    // Print the extracted fields
    debug!("sw_part_number: {:?}", sw_part_number);
    debug!("sw_version: {:?}", sw_version);
    debug!("sw_part_type: {:?}", sw_part_type);
    debug!("ecu_address: {:?}", ecu_address);
    debug!("data_format_identifier: {:?}", data_format_identifier);
    debug!("erase: {:?}", erase);
    debug!("erase_start_addr: {:?}", erase_start_addr);
    debug!("erase_length_addr: {:?}", erase_length_addr);
    debug!("verification_block_start: {:?}", verification_block_start);
    debug!("verification_block_length: {:?}", verification_block_length);
    debug!("verification_block_root_hash: {:?}", verification_block_root_hash);
    debug!("sw_signature_dev: {:?}", sw_signature_dev);
    debug!("file_checksum: {:?}", file_checksum);

    //send erase memory
    if !erase_start_addr.is_empty() && !erase_length_addr.is_empty() {
        let mut byte_vector: Vec<u8> = vec![0x31, 0x01, 0xff, 0x00];
        match utils::common::hex_string_to_bytes(erase_start_addr.as_str()) {
            Ok(bytes) => {
                byte_vector.extend(bytes);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        match utils::common::hex_string_to_bytes(erase_length_addr.as_str()) {
            Ok(bytes) => {
                byte_vector.extend(bytes);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        match stream.send_diag(byte_vector) {
            Ok(()) => {}
            Err(err) => {
                eprintln!("Failed to send diag check_memory: {}", err);
                return Err(err);
            }
        }
        match stream.receive_diag(timeout) {
            Ok(data) => {
                debug!("Sent erase, Expect: {}, Receive {:02X?}", "74*", data);
                if utils::common::compare_expect_value("7101ff00*", data) == false {
                    return Err(Error::new(ErrorKind::InvalidData, "erase Diag data received is not expected"));
                }
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    loop {
        //1. read start and length address of block
        let mut block_start_address_bytes = [0; 4];
        if file.read_exact(&mut block_start_address_bytes).is_err() {
            // UnexpectedEof error indicates end of the file
            break;
        }
        let mut block_length_bytes = [0; 4];
        file.read_exact(&mut block_length_bytes)?;
        let block_start_address = u32::from_be_bytes(block_start_address_bytes); // or u32::from_le_bytes(a_bytes) for little-endian
        let block_length = u32::from_be_bytes(block_length_bytes); // or u32::from_le_bytes(b_bytes) for little-endian
        debug!("block_start_address {:X}", block_start_address);
        debug!("block_length {:X}", block_length);

        //send Request Data Download
        let mut byte_vector: Vec<u8> = vec![0x34, 0x00, 0x44];
        byte_vector.extend_from_slice(&block_start_address.to_be_bytes());
        byte_vector.extend_from_slice(&block_length.to_be_bytes());
        match stream.send_diag(byte_vector) {
            Ok(()) => {}
            Err(err) => {
                eprintln!("Failed to send diag Request download: {}", err);
                return Err(err);
            }
        }
        match stream.receive_diag(timeout) {
            Ok(data) => {
                debug!("Sent Request Data Download, Expect: {}, Receive {:02X?}", "74*", data);
                if utils::common::compare_expect_value("74*", data) == false {
                    return Err(Error::new(ErrorKind::InvalidData, "request-download Diag data received is not expected"));
                }
            }
            Err(err) => {
                return Err(err);
            }
        }

        //2. read data block
        let mut temp_block_length = block_length;
        let mut block_seq_num: u8 = 1;
        //send_diag transfer data
        while temp_block_length != 0 {
            let data_file = file.try_clone()?;
            let data_len: u32 = std::cmp::min(max_buffer_len, temp_block_length);
            temp_block_length -= data_len;
            let mut data_blocks = Vec::with_capacity(data_len as usize);
            data_file.take(data_len as u64).read_to_end(&mut data_blocks)?;
            let mut byte_vector: Vec<u8> = vec![0x36, block_seq_num];
            if block_seq_num == 255 {
                block_seq_num = 0;
            } else {
                block_seq_num += 1;
            }
            byte_vector.extend_from_slice(&data_blocks);
            match stream.send_diag(byte_vector) {
                Ok(()) => {}
                Err(err) => {
                    eprintln!("Failed to send diag Request download: {}", err);
                    return Err(err);
                }
            }
            match stream.receive_diag(timeout) {
                Ok(data) => {
                    debug!("Sent diag transfer-block {}, Expect: 76*, Receive {:?}", block_seq_num, data);
                    if utils::common::compare_expect_value("76*", data) == false {
                        return Err(Error::new(ErrorKind::InvalidData, "request-download Diag data received is not expected"));
                    }
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        //send_diag transfer exit
        let byte_vector: Vec<u8> = vec![0x37];
        match stream.send_diag(byte_vector) {
            Ok(()) => {}
            Err(err) => {
                eprintln!("Failed to send diag Request download: {}", err);
                return Err(err);
            }
        }
        match stream.receive_diag(timeout) {
            Ok(data) => {
                debug!("Sent transfer-exit, Expect: {}, Receive {:02X?}", "77*", data);
                if utils::common::compare_expect_value("77*", data) == false {
                    return Err(Error::new(ErrorKind::InvalidData, "request-download Diag data received is not expected"));
                }
            }
            Err(err) => {
                return Err(err);
            }
        }

        //3. read checksum of block
        let mut checksum_bytes = [0; 2];
        file.read_exact(&mut checksum_bytes)?;
        let checksum = u16::from_be_bytes(checksum_bytes);
        debug!("checksum {:X}", checksum);
    }

    //send diag signature for check_memory
    let mut byte_vector: Vec<u8> = vec![0x31, 0x01, 0x02, 0x12];
    match utils::common::hex_string_to_bytes(sw_signature_dev.as_str()) {
        Ok(bytes) => {
            byte_vector.extend(bytes);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    match stream.send_diag(byte_vector) {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Failed to send diag check_memory: {}", err);
            return Err(err);
        }
    }
    match stream.receive_diag(timeout) {
        Ok(data) => {
            debug!("Sent check_memory, Expect: {}, Receive {:02X?}", "74*", data);
            if utils::common::compare_expect_value("710102121000*", data) == false {
                return Err(Error::new(ErrorKind::InvalidData, "check memory Diag data received is failed"));
            }
        }
        Err(err) => {
            return Err(err);
        }
    }

    debug!("Flashed {} successfully", sw_filename_clone);

    Ok(())
}
