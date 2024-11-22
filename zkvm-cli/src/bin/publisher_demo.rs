use std::ops::Add;

use alloy::{
    hex,
    network::EthereumWallet,
    providers::{Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
    sol_types::SolValue,
};
use alloy_primitives::{Address, U256};
use anyhow::{Context, Result};
use clap::Parser;
use ethers::abi::{ethabi, Token};
use risc0_ethereum_contracts::{encode_seal, groth16::abi_encode};
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, VerifierContext};
use url::Url;
use zkvm_methods::OWNER_VERIFY_ELF;

alloy::sol!(
    #[sol(rpc, all_derives)]
    "../contracts/src/ISimpleAccountDemo.sol"
);

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    chain_id: u64,

    #[clap(long)]
    eth_wallet_private_key: PrivateKeySigner,

    #[clap(long)]
    rpc_url: Url,

    #[clap(long)]
    contract: Address,

    #[clap(long)]
    public_key: String,

    #[clap(long)]
    expected_address: String,

    #[clap(long)]
    recipient: Address,

    #[clap(long)]
    amount: U256,

    #[clap(long)]
    data: Option<String>, // Optional calldata parameter as hex string
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    println!("Starting the program...");

    let args = Args::parse();
    println!("Parsed arguments: {:?}", args);

    let private_key = args.eth_wallet_private_key;
    let wallet = EthereumWallet::from(private_key.clone());
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(args.rpc_url);
    println!("Initialized provider with wallet.");

    // Generate Proof
    let public_key = args.public_key;
    let expected_address = args.expected_address;

    let encoded_inputs = ethabi::encode(&[
        Token::Bytes(hex::decode(public_key).expect("Failed to decode public key")),
        Token::Address(expected_address.parse().expect("Failed to decode address")),
    ]);

    let env = ExecutorEnv::builder()
        .write(&encoded_inputs)
        .expect("Writing failed")
        .build()
        .expect("Failed to create ExecutorEnv");

    // TODO: replace with STARK
    let groth16_proof_receipt = default_prover()
        .prove_with_ctx(
            env,
            &VerifierContext::default(),
            OWNER_VERIFY_ELF,
            &ProverOpts::groth16(),
        )?
        .receipt;
    println!("Generated proof and receipt: {:?}", groth16_proof_receipt);

    let seal = encode_seal(&groth16_proof_receipt)?;
    println!("Encoded seal: {:?}", seal);

    // Extract the journal from the receipt.
    let journal = groth16_proof_receipt.journal.bytes.clone();

    let owner_address = Address::abi_decode(&journal, true).expect("Failed to decode");
    println!("owner_address: {:?}", owner_address);

    // Prepare calldata (data) for the execute function
    let calldata = args
        .data
        .map(|data| hex::decode(data.strip_prefix("0x").unwrap_or(&data)).unwrap_or_default())
        .unwrap_or_default();

    // Execute Transaction
    let simple_account_contract = ISimpleAccountDemo::new(args.contract, provider);
    println!(
        "Initialized contract instance for address: {:?}",
        args.contract
    );

    let tx_data = simple_account_contract.execute(
        args.recipient,
        args.amount,
        calldata.into(),
        seal.into(),
        owner_address,
    );

    let runtime = tokio::runtime::Runtime::new()?;
    let pending_tx = runtime.block_on(tx_data.send())?;
    println!("Transaction sent. Pending transaction: {:?}", pending_tx);

    let final_receipt = runtime.block_on(pending_tx.get_receipt())?;
    println!("Transaction receipt: {:?}", final_receipt);

    Ok(())
}
