use base64::Engine;
use crate::state_services::err_set::ErrState;

extern crate base64;

pub fn encode_base64(input: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(input)
}

pub fn decode_base64(input: &str) -> Result<Vec<u8>, ErrState> {
    base64::engine::general_purpose::STANDARD.decode(input).map_err(|e| ErrState::new(12,"base64からVec<u8>への変換に失敗",None))
}

