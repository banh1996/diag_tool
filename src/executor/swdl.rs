use std::io::{self, Read, Error, ErrorKind};
use log::debug;
use std::fs::File;

//TODO: parse header and send sw data

fn extract_erase_values(erase_content: &str) -> (String, String) {
    let mut erase_start_addr = String::new();
    let mut erase_end_addr = String::new();
    let mut lines = erase_content.lines();
    if let Some(start_addr_line) = lines.next() {
        let start_addr_parts: Vec<&str> = start_addr_line
            .split(',')
            .map(|s| s.trim())
            .collect();
        if start_addr_parts.len() == 2 {
            erase_start_addr = start_addr_parts[0]
                .trim_matches(&['{', '}', ' ', '\n', '\t', '\r', '\x0c', ';'][..])
                .trim_start_matches("0x")
                .to_string();
            erase_end_addr = start_addr_parts[1]
                .trim_matches(&['{', '}', ' ', '\n', '\t', '\r', '\x0c', ';'][..])
                .trim_start_matches("0x")
                .to_string();
        }
    }
    (erase_start_addr, erase_end_addr)
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
pub fn parse_vbf(sw_filename: String) -> Result<(), io::Error> {
    // Open the file and read its content
    let mut file = File::open(&sw_filename).expect("Failed to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    // Find the start and end positions of the header section
    let header_start = match contents.find("header {") {
        Some(start) => start,
        None => {
            debug!("Header not found in the file.");
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid vbf header"));
        }
    };

    let mut open_braces = 0;
    let mut close_braces = 0;
    let mut header_end = None;

    // Find the matching closing brace for the opening brace of the header section
    for (idx, char) in contents[header_start..].chars().enumerate() {
        match char {
            '{' => open_braces += 1,
            '}' => close_braces += 1,
            _ => {}
        }

        if open_braces > 0 && open_braces == close_braces {
            header_end = Some(header_start + idx);
            break;
        }
    }

    // Parse the content and extract sw_part_number and sw_version
    if let Some(header_end) = header_end {
        let header_content = &contents[header_start..=header_end];
        //let lines: Vec<&str> = header_content.lines().collect();

        // Parse the content and extract fields from the header
        let sw_part_number = extract_value(&header_content, "sw_part_number");
        let sw_version = extract_value(&header_content, "sw_version");
        let sw_part_type = extract_value(&header_content, "sw_part_type");
        let ecu_address = extract_value(&header_content, "ecu_address");
        let data_format_identifier = extract_value(&header_content, "data_format_identifier");
        let erase = extract_value(&header_content, "erase");
        let (erase_start_addr, erase_end_addr) = extract_erase_values(&erase);
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
        debug!("erase_end_addr: {:?}", erase_end_addr);
        debug!("erase_start_addr: {:?}", erase_start_addr);
        debug!("verification_block_start: {:?}", verification_block_start);
        debug!("verification_block_length: {:?}", verification_block_length);
        debug!("verification_block_root_hash: {:?}", verification_block_root_hash);
        debug!("sw_signature_dev: {:?}", sw_signature_dev);
        debug!("file_checksum: {:?}", file_checksum);

    } else {
        debug!("Invalid header format in the file.");
    }

    Ok(())
}
