use ethers::utils::hex;
use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use zkvm::ECDSA_VERIFY_ELF;

/// Given an sec
///
/// 1. Write inputs in the Env
/// 2. Create a default prover
/// 3. Initialize the prover with the ZKVM ELF binary
/// 4. Generate a Prove
pub fn prove_ecdsa_verification(
    owner_address: String,
    message: &[u8],
    signature: &Signature,
    rec_id_bytes: u8,
) -> Receipt {
    // TODO: change to use refs
    // let inputs = ECDSAInputs {
    //     owner_address,
    //     message: message.to_vec(),
    //     signature: *signature,
    //     rec_id_bytes
    // };
    let inputs = (owner_address, message, signature, rec_id_bytes);
    let env = ExecutorEnv::builder()
        .write(&inputs)
        .expect("Failed to write inputs")
        .build()
        .expect("Failed to build inputs");

    let prover = default_prover();

    prover
        .prove(env, ECDSA_VERIFY_ELF)
        .expect("Failed to generate a proof")
        .receipt
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;
    use ::hex::encode;
    use ethers::{
        abi::Bytes,
        utils::{
            hash_message,
            hex::{self, traits::FromHex},
            keccak256, secret_key_to_address,
        },
    };
    use k256::{
        ecdsa::{signature::SignerMut, RecoveryId, SigningKey},
        elliptic_curve::rand_core::OsRng,
        EncodedPoint,
    };
    use sha3::{Digest, Keccak256};
    use zkvm::ECDSA_VERIFY_ID;

    #[test]
    fn test_verify_ecdsa() {
        // Record the start time
        let start_time = Instant::now();

        let priv_key =
            hex::decode(b"4c0883a69102937d6231471b5dbb6204fe5129617082792ae468d01a3f362318")
                .expect("Failed to encode priv key");
        let signing_key = SigningKey::from_slice(&priv_key).unwrap();
        let message =
            b"ECDSA proves knowledge of a secret number in the context of a single message";
        let (signature, rec_id) = signing_key
            .sign_recoverable(message)
            .expect("Failed to sign a recoverable signature");

        let eth_address = hex::encode(&signing_key.verifying_key().to_sec1_bytes());

        // Get resulting receipt from the verified signature
        let receipt = prove_ecdsa_verification(eth_address, message, &signature, rec_id.to_byte());

        // Verify the receipt
        receipt.verify(ECDSA_VERIFY_ID).expect("Failed to verify");
        let receipt_owner_address: String =
            receipt.journal.decode().expect("Failed to decode journal");

        println!(
            "Verified the signature over message with address {}",
            receipt_owner_address,
        );

        // Record the end time
        let end_time = Instant::now();
        let duration = end_time.duration_since(start_time);

        // Convert to minutes and seconds
        let minutes = duration.as_secs() / 60;
        let seconds = duration.as_secs() % 60;

        println!(
            "Test completed in {} minutes and {} seconds.",
            minutes, seconds
        );
    }
}
