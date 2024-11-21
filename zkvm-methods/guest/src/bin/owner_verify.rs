use risc0_zkvm::guest::env;
use tiny_keccak::{Hasher, Keccak};
use hex::encode;

fn main() {
    // Input: (public_key: String, expected_address: String)
    let (public_key, expected_address): (String, String) = env::read();

    let public_key_bytes = hex::decode(&public_key).expect("Invalid public key hex string");

    let mut hasher = Keccak::v256();
    let mut hash_output = [0u8; 32];
    hasher.update(&public_key_bytes); // Skip the first byte (prefix 0x04)
    hasher.finalize(&mut hash_output);

    // Step 4: Extract the last 20 bytes of the hash as the Ethereum address
    let ethereum_address_bytes = &hash_output[12..];
    let ethereum_address = format!("0x{}", encode(ethereum_address_bytes));

    // Step 5: Verify the computed Ethereum address matches the expected address
    assert_eq!(
        ethereum_address, expected_address,
        "Ethereum address does not match!"
    );

    // Commit the verified Ethereum address to the journal
    env::commit(&ethereum_address);
}
