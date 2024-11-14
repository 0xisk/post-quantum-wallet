use k256::ecdsa::{Signature, VerifyingKey};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use zkvm::ECDSA_VERIFY_ELF;

/// Given an sec
///
/// 1. Write inputs in the Env
/// 2. Create a default prover
/// 3. Initialize the prover with the ZKVM ELF binary
/// 4. Generate a Prove
pub fn prove_ecdsa_verification(
    verifying_key: &VerifyingKey,
    message: &[u8],
    signature: &Signature,
) -> Receipt {
    let inputs = (verifying_key.to_encoded_point(true), message, signature);
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
    use super::*;
    use k256::{
        ecdsa::{signature::SignerMut, SigningKey},
        elliptic_curve::rand_core::OsRng,
        EncodedPoint,
    };
    use zkvm::ECDSA_VERIFY_ID;

    #[test]
    fn test_verify_ecdsa() {
        let mut signing_key = SigningKey::random(&mut OsRng);
        let message =
            b"ECDSA proves knowledge of a secret number in the context of a single message";
        let signature: Signature = signing_key.sign(message);

        // Get resulting receipt from the verified signature
        let receipt = prove_ecdsa_verification(signing_key.verifying_key(), message, &signature);

        // Verify the receipt
        receipt.verify(ECDSA_VERIFY_ID).expect("Failed to verify");
        let (receipt_verifying_key, receipt_message): (EncodedPoint, Vec<u8>) =
            receipt.journal.decode().expect("Failed to decode journal");

        println!(
            "Verified the signature over message {:?} with key {}",
            std::str::from_utf8(&receipt_message[..]).unwrap(),
            receipt_verifying_key,
        );
    }
}
