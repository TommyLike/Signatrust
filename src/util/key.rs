use hex;

pub fn encode_u8_to_hex_string(value: &[u8]) -> String {
    value
        .iter()
        .map(|n| format!("{:02X}", n))
        .collect::<String>()
}

pub fn decode_hex_string_to_u8(value: &String) -> Vec<u8> {
    hex::decode(value).unwrap()
}
