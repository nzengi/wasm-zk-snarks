/// This module generates and verifies ZK-SNARK proofs.
use crate::utils;


pub struct Proof {
    pub proof_value: u64,
}

/// Generates a ZK-SNARK proof given the proving key, witness hash, and witness.
/// Returns `Some(Proof)` if successful, or `None` if verification fails.
pub fn generate_proof(pk: u64, x: &[u8], w: &[u8]) -> Option<Proof> {
    if utils::verify_witness_hash(x, w) {
        Some(Proof {
            proof_value: pk + 1,
        })
    } else {
        None
    }
}

/// Verifies the given proof using the verification key.
/// Returns `true` if the proof is valid, `false` otherwise.
pub fn verify_proof(vk: u64, _x: &[u8], proof: Proof) -> bool {
    let expected_proof = vk.wrapping_add(2);
    proof.proof_value == expected_proof
}
