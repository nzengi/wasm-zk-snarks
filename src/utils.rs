use sha2::{Sha256, Digest};
use getrandom::getrandom;

/// Hashes a given witness using the SHA-256 algorithm.
/// Returns the hash as a vector of bytes.
pub fn hash_witness(witness: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(witness);
    hasher.finalize().to_vec()
}

/// Verifies if a given hash matches the hash of a witness.
pub fn verify_witness_hash(x: &[u8], w: &[u8]) -> bool {
    let witness_hash = hash_witness(w);
    witness_hash.as_slice() == x
}

/// Generates a random `u64` value to act as a secret key (lambda).
/// Uses `getrandom` for cryptographically secure random number generation.
pub fn generate_random_lambda() -> u64 {
    let mut buf = [0u8; 8];
    getrandom(&mut buf).expect("Failed to generate random lambda");
    u64::from_le_bytes(buf)
}

/// Generates a random witness, which is a 32-byte random value.
/// Uses `getrandom` for cryptographically secure random number generation.
pub fn random_witness() -> Vec<u8> {
    let mut buf = [0u8; 32];
    getrandom(&mut buf).expect("Failed to generate random witness");
    buf.to_vec()
}
