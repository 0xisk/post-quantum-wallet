use std::ops::Add;

use alloy_primitives::Address;
use alloy_sol_types::{abi::Decoder, SolValue};
use ethabi::{ParamType, Token};
use hex::encode;
use risc0_zkvm::guest::env;
use tiny_keccak::{Hasher, Keccak};

fn main() {
    // Input: (public_key: String, expected_address: String)
    let encoded_inputs: Vec<u8> = env::read();

    let params_inputs = vec![ParamType::Bytes, ParamType::Address];

    let decoded_inputs = ethabi::decode(&params_inputs, &encoded_inputs).expect("Failed to decode");

    let public_key = match &decoded_inputs[0] {
        Token::Bytes(bytes) => bytes.clone(),
        _ => panic!("Failed to decode public key"),
    };

    let expected_address = Address::from(match &decoded_inputs[1] {
        Token::Address(addr) => addr.0,
        _ => panic!("Failed to decode address"),
    });

    let actual_address = Address::from_raw_public_key(&public_key);
    println!("ethereum_address_Address: {:?}", actual_address);

    // Step 5: Verify the computed Ethereum address matches the expected address
    assert_eq!(
        actual_address, expected_address,
        "Ethereum address does not match!"
    );

    // Commit the verified Ethereum address to the journal
    env::commit_slice(&actual_address.abi_encode().as_slice());
}
