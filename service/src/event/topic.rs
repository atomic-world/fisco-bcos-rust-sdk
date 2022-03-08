use pad::{PadStr, Alignment};
use keccak_hash::keccak;
use wedpr_l_crypto_hash_sm3::WedprSm3;
use wedpr_l_utils::traits::Hash;

pub fn from_integer(value: u64) -> String {
    format!("0x{:}", hex::encode(value.to_be_bytes()).pad(64, '0', Alignment::Right, true))
}

pub fn from_address(address: &str) -> String {
    assert!(address.starts_with("0x"), "The argument should start with 0x");
    assert_eq!(address.len(), 42, "The argument should be at a length of 42");
    format!("0x000000000000000000000000{:}", address.to_owned().trim_start_matches("0x"))
}

pub fn from_bool(value: bool) -> String {
    if value {
        String::from("0x0000000000000000000000000000000000000000000000000000000000000001")
    } else {
        String::from("0x0000000000000000000000000000000000000000000000000000000000000000")
    }
}

pub fn from_str(value: &str, sm_crypto: bool) -> String {
    let hash = if sm_crypto {
        let sm3_hash = WedprSm3::default();
        sm3_hash.hash(value)
    } else {
        Vec::from(keccak(value).as_bytes())
    };
    format!("0x{:}", hex::encode(hash))
}

pub fn from_event_signature(event_signature: &str, sm_crypto: bool) -> String {
    let re = fancy_regex::Regex::new(r#"\s+"#).unwrap();
    let event_signature = re.replace_all(event_signature, "");
    from_str(&event_signature, sm_crypto)
}

#[cfg(test)]
mod tests {
    use crate::event::topic::*;

    #[test]
    fn test_from_integer() {
        assert_eq!("0x0000000000000000000000000000000000000000000000000000000000000001", from_integer(1));
        assert_eq!("0x0000000000000000000000000000000000000000000000000000000000000010", from_integer(16));
        assert_eq!("0x0000000000000000000000000000000000000000000000000000000000000012", from_integer(18));
        assert_eq!("0x0000000000000000000000000000000000000000000000000000000000000020", from_integer(32));
    }

    #[test]
    #[should_panic(expected="The argument should start with 0x")]
    fn test_from_address_with_prefix_error() {
        from_address("11b6d7495f2f04bdca45e9685ceadea4d4bd1832");
    }

    #[test]
    #[should_panic(expected="The argument should be at a length of 42")]
    fn test_from_address_with_length_error() {
        from_address("0x11b6d7495f2f04bdca45e9685ceadea4d4bd183");
    }

    #[test]
    fn test_from_address() {
        assert_eq!("0x00000000000000000000000011b6d7495f2f04bdca45e9685ceadea4d4bd1832", from_address("0x11b6d7495f2f04bdca45e9685ceadea4d4bd1832"));
    }

    #[test]
    fn test_from_str() {
        assert_eq!("0x19fa07abe78276c06a4020e2db1604f1ee14b2ebe672d29f51c1c995890a7518", from_str("nanjingboy", false));
        assert_eq!("0x0a7e6ffe38f2f44a895b7652f56317064fbfbe2d22200abf46b22966b63c7fa4", from_str("nanjingboy", true));
    }

    #[test]
    fn test_from_event_signature() {
        assert_eq!("0x08ad0c610d1cadcb6ed40e0ed05ad34c51342d4dc96d56a1bf376a64df239789", from_event_signature("event2(string,int256)", false));
        assert_eq!("0x08ad0c610d1cadcb6ed40e0ed05ad34c51342d4dc96d56a1bf376a64df239789", from_event_signature(" event2 (string, int256 )", false));
        assert_eq!("0xcc3b390a09af747470093499ab85c9a7e6e71f2fa86951239316edbb4ea3cd5a", from_event_signature("event2(string,int256)", true));
        assert_eq!("0xcc3b390a09af747470093499ab85c9a7e6e71f2fa86951239316edbb4ea3cd5a", from_event_signature("event2 ( string, int256) ", true));
    }
}