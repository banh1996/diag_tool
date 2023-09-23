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