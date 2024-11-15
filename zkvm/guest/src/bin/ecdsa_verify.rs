use hex::encode;
use k256::{
    ecdsa::{signature::Verifier, RecoveryId, Signature, VerifyingKey},
    EncodedPoint,
};
use risc0_zkvm::guest::env;

fn main() {
    let (owner_address, message, signature, rec_id_bytes): (String, Vec<u8>, Signature, u8) =
        env::read();

    let rec_id = match RecoveryId::from_byte(rec_id_bytes) {
        Some(v) => v,
        None => panic!("Failed to convert back recover id"),
    };

    let verifying_key = VerifyingKey::recover_from_msg(&message, &signature, rec_id)
        .expect("Failed to recover public key");

    // Verify the signature
    verifying_key
        .verify(&message, &signature)
        .expect("Failed to verify signature");

    // Check Eth address with Owner
    let recovered_address = encode(verifying_key.to_sec1_bytes());

    // Assertion
    assert_eq!(owner_address, recovered_address);

    println!("Verified the signature over message {:?}", owner_address);

    env::commit(&owner_address);
}
