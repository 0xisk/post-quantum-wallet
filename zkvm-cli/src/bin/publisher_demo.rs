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
    simple_account: Address,

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
    let public_key = "8318535b54105d4a7aae60c08fc45f9687181b4fdfc625bd1a753fa7397fed753547f11ca8696646f2f3acb08e31016afac23e630c5d11f59f61fef57b0d2aa5".to_string();
    let expected_address = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266".to_string();

    let inputs = (&public_key, &expected_address);
    let env = ExecutorEnv::builder()
        .write(&inputs)
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
    let journal = groth16_proof_receipt.journal;
    let journal_bytes = journal.bytes.clone();
    println!("journal: {:?}", journal);

    // Encode the journal as an ABI-compatible byte array.
    // let encoded_journal = ethabi::encode(&[ethabi::Token::Bytes(journal.clone())]);
    // println!("Encoded journal: {:?}", encoded_journal);

    let address = abi_encode(&journal_bytes).context("decoding journal data")?;
    println!("journal: {:?}", address);

    // Prepare calldata (data) for the execute function
    let calldata = args
        .data
        .map(|data| hex::decode(data.strip_prefix("0x").unwrap_or(&data)).unwrap_or_default())
        .unwrap_or_default();
    println!("Prepared calldata: {:?}", calldata);

    // Execute Transaction
    let simple_account_contract = ISimpleAccountDemo::new(args.simple_account, provider);
    println!(
        "Initialized contract instance for address: {:?}",
        args.simple_account
    );

    let tx_data = simple_account_contract.execute(
        journal_bytes.into(),
        args.recipient,
        args.amount,
        calldata.into(),
        seal.into(),
    );

    let runtime = tokio::runtime::Runtime::new()?;
    let pending_tx = runtime.block_on(tx_data.send())?;
    println!("Transaction sent. Pending transaction: {:?}", pending_tx);

    let final_receipt = runtime.block_on(pending_tx.get_receipt())?;
    println!("Transaction receipt: {:?}", final_receipt);

    Ok(())
}
