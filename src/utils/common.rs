use hex;

/*****************************************************************************************************************
 *  utils::common::parse_duration_to_milliseconds function
 *  brief      Function to convert time string to integer milliseconds
 *  details    -
 *  \param[in]  duration_str: refer to timeout string. Ex: "2s", "100ms", "80m", "3h"
 *  \param[out] -
 *  \precondition -
 *  \reentrant:  TRUE
 *  \return     Milliseconds if any
 ****************************************************************************************************************/
pub fn parse_duration_to_milliseconds(duration_str: &str) -> Option<u64> {
    let numeric_part: String = duration_str.chars().take_while(|c| c.is_digit(10)).collect();
    let unit_part: &str = &duration_str[numeric_part.len()..];

    match numeric_part.parse::<u64>() {
        Ok(value) => {
            let seconds = match unit_part {
                "s" => value * 1000,
                "m" => value * 60_000,
                "h" => value * 3_600_000,
                "ms" => value,
                _ => return None, // Invalid unit
            };
            Some(seconds)
        }
        Err(_) => None, // Failed to parse the numeric part
    }
}


/*****************************************************************************************************************
 *  utils::common::vec_hex_strings_to_u8 function
 *  brief      Function to convert hex Vec<String> to Vec<u8> array
 *  details    Example: "11223344" -> 0x11 0x22 0x33 0x44
 *  \param[in]  hex_strings: refer to hex string
 *  \param[out] -
 *  \precondition -
 *  \reentrant:  TRUE
 *  \return     Vec<u8> if any
 ****************************************************************************************************************/
pub fn vec_hex_strings_to_u8(hex_strings: &Vec<String>) -> Vec<u8> {
    hex_strings
        .iter()
        .map(|hex_str| {
            u8::from_str_radix(&hex_str[2..], 16).unwrap_or(0)
        })
        .collect()
}


/*****************************************************************************************************************
 *  utils::common::compare_expect_value function
 *  brief      Function to compare string and vec<u8> type in hex format
 *  details    Example:
 *              "22f186*" and "22f186" => match:true
 *              "22f186*" and "22f18622" => match:true
 *              "22f186*" and "22f287" => not match: false
 *              "22f186*" and "122f186" => not match: false
 *  \param[in]  string_a: refer to string.
 *              vec_b: refer to u8 vec array
 *  \param[out] -
 *  \precondition -
 *  \reentrant:  TRUE
 *  \return     true if matching, false if not match
 ****************************************************************************************************************/
pub fn compare_expect_value(string_a: &str, vec_b: Vec<u8>) -> bool {
    // Convert vec_b to a hex string format
    let string_b: String = vec_b.iter().map(|byte| format!("{:02X}", byte)).collect();

    let mut iter_a = string_a.chars().flat_map(char::to_lowercase);
    let mut iter_b = string_b.chars().flat_map(char::to_lowercase);

    while let (Some(char_a), Some(char_b)) = (iter_a.next(), iter_b.next()) {
        if char_a == '*' {
            // If '*' is encountered in string_a, treat it as a match and continue
            continue;
        }

        if char_a != char_b {
            return false; // Mismatch detected
        }
    }

    true
}


/*****************************************************************************************************************
 *  utils::common::hex_string_to_bytes function
 *  brief      Function to convert hex string to Vec<u8> array
 *  details    Example: "0x11223344" -> 0x11 0x22 0x33 0x44
 *  \param[in]  hex_strings: refer to hex string
 *  \param[out] -
 *  \precondition -
 *  \reentrant:  TRUE
 *  \return     Vec<u8> if any
 ****************************************************************************************************************/
pub fn hex_string_to_bytes(hex_string: &str) -> Result<Vec<u8>, hex::FromHexError> {
    let hex_string_without_prefix = hex_string.trim_start_matches("0x");
    hex::decode(hex_string_without_prefix)
}


/*****************************************************************************************************************
 *  utils::common::hex_to_u16 function
 *  brief      Function to convert hex string to u16
 *  details    Example: "0x1122" -> 0x1122, "1122" -> 0x1122
 *  \param[in]  hex_string: refer to hex string
 *  \param[out] -
 *  \precondition -
 *  \reentrant:  TRUE
 *  \return     u16, 0 if error
 ****************************************************************************************************************/
pub fn hex_to_u16(hex_string: &str) -> u16 {
    // Remove the leading "0x" if it exists.
    let hex_string_without_prefix = hex_string.trim_start_matches("0x");
    let mut hex_bytes = Vec::new();
    match hex::decode(hex_string_without_prefix) {
        Ok(bytes) => {
            hex_bytes.extend(bytes);
        }
        Err(e) => {
            println!("Error: {}", e);
            return 0;
        }
    }
    // Convert the hex string to a u16.
    if hex_bytes.len() < 2{
        return 0;
    }
    u16::from_be_bytes([hex_bytes[0], hex_bytes[1]])
}