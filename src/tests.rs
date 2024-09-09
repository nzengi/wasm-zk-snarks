#[cfg(test)]
mod tests {
    use crate::utils::random_witness;
    use crate::{generate_zk_proof, verify_zk_proof};

    #[test]
    /// Test the generation of a valid ZK-SNARK proof for a randomly generated witness.
    /// Ensures that the proof generation works and includes the expected output message.
    fn test_zkp() {
        // Step 1: Generate a random witness
        let witness = random_witness();
        
        // Step 2: Generate a ZK-SNARK proof using the random witness
        let proof = generate_zk_proof(&witness);

        // Step 3: Ensure that the proof contains the "Proof generated" message
        assert!(proof.contains("Proof generated"));
    }

    #[test]
    /// Test the verification of a ZK-SNARK proof with an invalid verification key.
    /// Ensures that the verification fails when using a random invalid key.
    fn test_invalid_witness() {
        // Step 1: Generate a random witness
        let witness = random_witness();

        // Step 2: Attempt to verify the ZK-SNARK proof with an invalid verification key (random)
        let valid = verify_zk_proof(&witness, 123456);  // Random invalid verification key

        // Step 3: Ensure that the proof verification fails
        assert!(!valid);
    }
}
