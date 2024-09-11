use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// A simple structure representing a cryptographic proof.
#[derive(Serialize, Deserialize, Clone)]
pub struct Proof {
    pub data: HashMap<String, String>,
}

/// Generates a cryptographic proof from the input data (as a HashMap).
pub fn create_proof(data: &HashMap<String, String>) -> Proof {
    // In a real zk-SNARK implementation, this would be where we create a succinct proof.
    // This is a simplified example returning the input as a "proof".
    Proof {
        data: data.clone(),
    }
}

/// Verifies the cryptographic proof against the original data.
pub fn verify_proof(proof: &Proof, data: &HashMap<String, String>) -> bool {
    // Simple verification to check if the proof's data matches the original input.
    proof.data == *data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_verify_proof() {
        let mut data = HashMap::new();
        data.insert(String::from("key1"), String::from("value1"));

        let proof = create_proof(&data);
        assert!(verify_proof(&proof, &data));
    }

    #[test]
    fn test_invalid_proof() {
        let mut data = HashMap::new();
        data.insert(String::from("key1"), String::from("value1"));

        let mut modified_data = HashMap::new();
        modified_data.insert(String::from("key1"), String::from("wrong_value"));

        let proof = create_proof(&data);
        assert!(!verify_proof(&proof, &modified_data));
    }
}
