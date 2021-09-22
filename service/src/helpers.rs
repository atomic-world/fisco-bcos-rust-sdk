use serde_json::Value as JSONValue;

pub fn parse_json_string(value: &JSONValue) -> String {
    match value.as_str() {
        Some(value) => value.to_owned(),
        None => String::from(""),
    }
}

pub fn parse_json_string_array(value: &JSONValue) -> Vec<String> {
    match value.as_array() {
        Some(list) => list.into_iter().map(
            |item| parse_json_string(item)
        ).collect(),
        None => vec![],
    }
}

pub fn convert_hex_str_to_u32(hex_str: &str) -> u32 {
    u32::from_str_radix(hex_str.to_owned().trim_start_matches("0x"), 16).unwrap()
}