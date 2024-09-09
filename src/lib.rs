pub mod proof;
pub mod keygen;
pub mod utils;
pub mod tests;

use wasm_bindgen::prelude::*;
use getrandom::getrandom;  // getrandom ile rastgele sayı üretiyoruz

#[wasm_bindgen]
pub fn generate_zk_proof(witness: &[u8]) -> String {
    let mut buf = [0u8; 8];  // Rastgele bir u64 sayısı için 8 byte
    getrandom(&mut buf).expect("Failed to generate random lambda");
    let lambda = u64::from_le_bytes(buf);  // Rastgele lambda değeri üretildi

    let keys = keygen::generate_keys(lambda);
    let hash = utils::hash_witness(witness);
    let proof = proof::generate_proof(keys.proving_key, &hash, witness);
    
    match proof {
        Some(proof) => format!("Proof generated: {:?}", proof.proof_value),
        None => "Failed to generate proof.".to_string(),
    }
}

#[wasm_bindgen]
pub fn verify_zk_proof(witness: &[u8], vk: u64) -> bool {
    let hash = utils::hash_witness(witness);
    let proof = proof::generate_proof(vk, &hash, witness).unwrap();
    proof::verify_proof(vk, &hash, proof)
}
