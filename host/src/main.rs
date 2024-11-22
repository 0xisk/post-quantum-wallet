use bytemuck::checked::cast;
use bytemuck::{Pod, Zeroable};
use risc0_ethereum_contracts::encode_seal;
use risc0_zkvm::serde::{from_slice, to_vec};
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, VerifierContext};
use std::fs::File;
use std::io::Write;
use methods::OWNER_VERIFY_ELF;

fn main() {
    let public_key = "04bfcab52a7c7ab70c6c93f830c0c24f5c8e7e6b5c2d76bb16b8c3ecf1cbd554fa15964cbb203ddeee2a47191d51f3a6b8078ec6c2383b0c0738392a88a5b07092".to_string();
    let expected_address = "0xe0954a05d5a1baf539c3b54566a1f32c357dbd1d".to_string();

    let inputs = (&public_key, &expected_address);
    let env = ExecutorEnv::builder()
        .write(&inputs)
        .expect("Serialization failed")
        .build()
        .expect("Failed to create ExecutorEnv");

    // Run the zkVM guest program and generate a receipt
    // let receipt = default_prover()
    //     .prove(env, OWNER_VERIFY_ELF) // Replace with your guest program's ELF name
    //     .expect("Failed to generate proof")
    //     .receipt;

    // Run the zkVM guest program and generate a Groth16 receipt proof
    // let succinct_receipt = default_prover()
    //     .prove_with_ctx(
    //         env,
    //         &VerifierContext::default(),
    //         OWNER_VERIFY_ELF,
    //         &ProverOpts::succinct(),
    //     )
    //     .expect("Failed to generate a proof")
    //     .receipt.inner.composite();

    let groth16_receipt = default_prover()
        .prove_with_ctx(
            env,
            &VerifierContext::default(),
            OWNER_VERIFY_ELF,
            &ProverOpts::groth16(),
        )
        .expect("Failed to generate a proof")
        .receipt;

    // Encode Groth16 receipt
    let groth16_seal = encode_seal(&groth16_receipt).expect("Failed to encode Groth16 proof");

    // Extract the journal from the receipt
    let computed_address: String = groth16_receipt.journal.decode().expect("Failed to decode");

    let jornal = groth16_receipt.journal.bytes.clone();

    // Display results
    println!(
        "Ethereum address verified: {} (Expected: {})",
        &computed_address, &expected_address
    );

    // let succinct_seal = succinct_receipt
    //     .inner
    //     .succinct()
    //     .expect("failed")
    //     .seal
    //     .clone()
    //     .iter()
    //     .flat_map(|x| x.to_le_bytes())
    //     .collect::<Vec<u8>>();

    let groth16_seal = groth16_receipt
        .inner
        .groth16()
        .expect("failed")
        .seal
        .clone()
        .iter()
        .flat_map(|x| x.to_le_bytes())
        .collect::<Vec<u8>>();

    // let file_path = "proof-succinct.txt";
    // let mut file = File::create(file_path).expect("Failed to create proof file");
    // file.write_all(&succinct_seal)
    //     .expect("Failed to write seal proof to file");

    let file_path = "proof-groth16.txt";
    let mut file = File::create(file_path).expect("Failed to create proof file");
    file.write_all(&groth16_seal)
        .expect("Failed to write seal proof to file");

    println!("Seal proof saved to {}", file_path);

    // Serialize the seal (proof) to send to the smart contract
    //let seal = encode_seal(&receipt).expect("Failed to encode receipt into a seal");

    // Send `serialized_seal`, `computed_address`, and `expected_address` to the smart contract
    //println!("Seal (proof): {:?}", seal);
}
