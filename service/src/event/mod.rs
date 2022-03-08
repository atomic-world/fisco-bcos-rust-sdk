pub mod event_emitter;
pub mod event_service;
pub mod event_log_param;
pub mod topic;

use serde_json::{Value as JSONValue};
use ethabi::{Log, RawLog, Hash as EthHash};

use crate::abi::{ABI, ABIError};

fn convert_event_log(response: &JSONValue) -> Vec<RawLog> {
    match response["logs"].as_array() {
        Some(logs) => {
            logs.iter().map(|log| {
                let topics= match log["topics"].as_array() {
                    Some(topics) => {
                        topics.iter()
                            .map(|topic| topic.as_str().unwrap_or(""))
                            .filter(|topic| topic.len() > 0)
                            .map(|topic| EthHash::from_slice( hex::decode(topic.to_owned().trim_start_matches("0x")).unwrap_or(vec![]).as_slice()))
                            .collect()
                    },
                    None => vec![],
                };
                let data = match log["data"].as_str() {
                    Some(data) => {
                        hex::decode(data.to_owned().trim_start_matches("0x")).unwrap_or(vec![])
                    },
                    None => vec![],
                };
                RawLog { topics, data }
            }).collect()
        },
        None => vec![],
    }
}

pub fn parse_event_log(
    response: &JSONValue,
    event_name: &str,
    abi_content: &str,
    sm_crypto: bool,
) -> Result<Vec<Log>, ABIError> {
    let mut result: Vec<Log> = vec![];
    let raw_logs = convert_event_log(response);
    if raw_logs.is_empty() {
        return Ok(result)
    }

    let abi_content = if abi_content.starts_with("[") && abi_content.ends_with("]") {
        Vec::from(abi_content)
    } else {
        Vec::from(format!("[{:}]", abi_content).as_bytes())
    };
    let abi = ABI::new(
        &Some(abi_content),
        &None,
        "",
        sm_crypto,
    )?;
    for raw_log in &raw_logs {
        result.push(abi.decode_event(event_name, raw_log)?);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use ethabi::{Log, LogParam, Token};
    use crate::event::*;

    const ABI_CONTENT: &str = r#"{"anonymous":false,"inputs":[{"indexed":false,"name":"s","type":"string"},{"indexed":true,"name":"n","type":"int256"}],"name":"event2","type":"event"}"#;
    const EVENT_LOGS: &str = r#"{"filterID":"95b154cf0aa34175a4207426a058102c","logs":[{"address":"0xf2df2e7c2a2dc9bd23523f4272e1de1d08d1e9a9","blockHash":"0xff15b0064d7e9f8bbc1670d97b22fd1f9b4343cafb59267f51361c4c21ac844d","blockNumber":"375","data":"0x000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000043132333400000000000000000000000000000000000000000000000000000000","logIndex":"0x0","topics":["0x08ad0c610d1cadcb6ed40e0ed05ad34c51342d4dc96d56a1bf376a64df239789","0x0000000000000000000000000000000000000000000000000000000000000000"],"transactionHash":"0xdfd4af57ca2aaf5772427500375c191ac84cbd67adac3aa1acd3c32039d36a1d","transactionIndex":"0x0"}],"result":0}"#;
    const SM_EVENT_LOGS: &str = r#"{"filterID":"95b154cf0aa34175a4207426a058102c","logs":[{"address":"0xf2df2e7c2a2dc9bd23523f4272e1de1d08d1e9a9","blockHash":"0xff15b0064d7e9f8bbc1670d97b22fd1f9b4343cafb59267f51361c4c21ac844d","blockNumber":"375","data":"0x000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000043132333400000000000000000000000000000000000000000000000000000000","logIndex":"0x0","topics":["0xcc3b390a09af747470093499ab85c9a7e6e71f2fa86951239316edbb4ea3cd5a","0x0000000000000000000000000000000000000000000000000000000000000000"],"transactionHash":"0xdfd4af57ca2aaf5772427500375c191ac84cbd67adac3aa1acd3c32039d36a1d","transactionIndex":"0x0"}],"result":0}"#;

    #[test]
    fn test_parse_event_log() {
        let expected_logs = vec![
            Log {
                params: vec![
                    LogParam { name: "s".to_string(), value: Token::String("1234".to_string()) },
                    LogParam { name: "n".to_string(), value: Token::Int(0.into()) },
                ]
            }
        ];
        let event_logs: JSONValue = serde_json::from_str(EVENT_LOGS).unwrap();
        let logs = parse_event_log(&event_logs, "event2", ABI_CONTENT, false).unwrap();
        assert_eq!(expected_logs, logs);

        let event_logs: JSONValue = serde_json::from_str(SM_EVENT_LOGS).unwrap();
        let logs = parse_event_log(&event_logs, "event2", ABI_CONTENT, true).unwrap();
        assert_eq!(expected_logs, logs);
    }

    #[test]
    fn test_convert_event_log() {
        let expected_logs = vec![
            RawLog {
                topics: vec![
                    EthHash::from_slice(hex::decode("08ad0c610d1cadcb6ed40e0ed05ad34c51342d4dc96d56a1bf376a64df239789").unwrap().as_slice()),
                    EthHash::from_slice(hex::decode("0000000000000000000000000000000000000000000000000000000000000000").unwrap().as_slice()),
                ],
                data: hex::decode("000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000043132333400000000000000000000000000000000000000000000000000000000").unwrap(),
            }
        ];
        let event_logs: JSONValue = serde_json::from_str(EVENT_LOGS).unwrap();
        assert_eq!(expected_logs, convert_event_log(&event_logs));
    }
}