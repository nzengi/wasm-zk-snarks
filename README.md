# ZK-SNARKs Library for Rust and WASM

## Overview

This library provides an implementation of Zero-Knowledge Succinct Non-Interactive Argument of Knowledge (ZK-SNARKs) for both **Rust** and **WebAssembly (WASM)**. It includes functions for generating and verifying ZK-SNARK proofs in blockchain applications and cryptographic protocols. The library is designed to be lightweight, secure, and optimized for performance.

### Features:
- **Generate and Verify ZK-SNARK Proofs:** Easily generate ZK-SNARK proofs and verify them using simple APIs.
- **WASM Support:** The library is fully compatible with WebAssembly, allowing it to be used in JavaScript environments such as browsers and Node.js.
- **Cryptographically Secure:** Built with industry-standard cryptographic algorithms like SHA-256 and secure random number generation.
- **Optimized for Blockchain:** Perfect for blockchain applications where privacy and verification are critical.

---

## Getting Started

### Requirements

- **Rust** (version 1.56 or higher)
- **wasm-bindgen** for WebAssembly support

### Installation

Add this crate to your `Cargo.toml` dependencies:

```toml
[dependencies]
zk-snarks-lib = "0.1.0"
wasm-bindgen = "0.2"
sha2 = "0.10"
getrandom = "0.2"
```

### Usage
Rust Example
Here's how you can generate and verify a ZK-SNARK proof in a Rust environment.

```rust
use zk_snarks_lib::{generate_zk_proof, verify_zk_proof};
use zk_snarks_lib::utils::random_witness;

fn main() {
    // Generate a random witness
    let witness = random_witness();
    
    // Generate a ZK-SNARK proof
    let proof = generate_zk_proof(&witness);
    println!("Proof: {}", proof);

    // Verify the proof with the correct verification key
    let is_valid = verify_zk_proof(&witness, 123456);  // Random verification key
    println!("Is the proof valid? {}", is_valid);
}
```

### WebAssembly Example (JavaScript)
To use the library in a JavaScript project, build the WASM module using wasm-pack and import it as follows:

```javascript
import init, { generate_zk_proof, verify_zk_proof } from './pkg/zk_snarks_lib.js';

async function run() {
    await init();
    
    const witness = new Uint8Array([/* ...random data... */]);
    const proof = generate_zk_proof(witness);
    console.log(`Generated Proof: ${proof}`);

    const isValid = verify_zk_proof(witness, 123456);  // Random verification key
    console.log(`Is the proof valid? ${isValid}`);
}

run();
```
