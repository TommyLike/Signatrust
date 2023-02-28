use hex;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn encode_u8_to_hex_string(value: &[u8]) -> String {
    value
        .iter()
        .map(|n| format!("{:02X}", n))
        .collect::<String>()
}

pub fn decode_hex_string_to_u8(value: &String) -> Vec<u8> {
    hex::decode(value).unwrap()
}

pub fn generate_api_token() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(40).map(char::from).collect()
}
