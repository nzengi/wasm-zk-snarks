use wasm_bindgen::prelude::*;
use crate::keygen::generate_key_with_timestamp;

#[wasm_bindgen]
pub fn wasm_generate_key() -> Vec<u8> {
    let (key, _timestamp) = generate_key_with_timestamp();
    key.to_vec()
}

#[wasm_bindgen]
pub fn wasm_current_timestamp() -> u64 {
    crate::utils::current_timestamp()
}
