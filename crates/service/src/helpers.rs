use serde_json::Value;

pub(crate) fn parse_serde_json_string_value(value: &Value) -> String {
    match value.as_str() {
        Some(value) => value.to_owned(),
        None => String::from(""),
    }
}

pub(crate) fn parse_serde_json_string_array_value(value: &Value) -> Vec<String> {
    match value.as_array() {
        Some(list) => list.into_iter().map(
            |item| parse_serde_json_string_value(item)
        ).collect(),
        None => vec![],
    }
}