use base64::Engine;
use crate::utils::err_set::ErrState;

extern crate base64;

pub fn encode_base64(input: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(input)
}

pub fn decode_base64(input: &str) -> Result<Vec<u8>, ErrState> {
    base64::engine::general_purpose::STANDARD.decode(input).map_err(|e| ErrState::new(1000, 500))
}

