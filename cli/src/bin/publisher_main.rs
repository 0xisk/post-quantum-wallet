use alloy::{
    network::EthereumWallet,
    providers::{Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
    sol_types::SolValue,
};
use alloy_primitives::{Address, U256};
use anyhow::{Context, Result};
use clap::Parser;
use risc0_ethereum_contracts::encode_seal;
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, VerifierContext};
use url::Url;
use methods::OWNER_VERIFY_ELF;

alloy::sol!(
    #[sol(rpc, all_derives)]
    "../contracts/src/ISimpleAccountFactory.sol"
);

alloy::sol!(
    #[sol(rpc, all_derives)]
    "../contracts/src/ISimpleAccount.sol"
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
    entry_point: Address,
    #[clap(long)]
    factory: Address,
    #[clap(long)]
    input: U256,
    #[clap(long)]
    recipient: Address,
    #[clap(long)]
    amount: U256,
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    println!("Starting the program...");
    let args = Args::parse();
    let private_key = args.eth_wallet_private_key;
    let wallet = EthereumWallet::from(private_key.clone());
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(args.rpc_url);

    // Deploy SimpleAccount
    let factory_contract = ISimpleAccountFactory::new(args.factory, provider.clone());
    let runtime = tokio::runtime::Runtime::new()?;
    let nonce = runtime.block_on(async {
        provider
            .get_transaction_count(private_key.address())
            .await
            .context("Failed to fetch nonce")
    })?;
    let gas_price = runtime.block_on(provider.get_gas_price())?;
    let gas_limit = 100_000u64;
    let max_fee = gas_price + 1_000_000_000u128;
    let create_account_tx = factory_contract
        .simpleCreateAccount(private_key.address(), U256::ZERO)
        .nonce(nonce)
        .gas(gas_limit)
        .max_fee_per_gas(max_fee)
        .max_priority_fee_per_gas(1_000_000_000u128);
    let receipt_pending_tx = runtime.block_on(create_account_tx.send())?;
    let receipt_pending_tx = runtime.block_on(receipt_pending_tx.get_receipt())?;
    let logs = receipt_pending_tx.inner.logs();
    if logs.is_empty() {
        panic!("No logs found. Receipt: {:?}", receipt_pending_tx);
    }
    println!("Logs: {:?}", logs);

    let simple_account_address = logs[0].address();
    println!("Deployed SimpleAccount at: {:?}", simple_account_address);
    println!("Deployed SimpleAccount at: {:?}", simple_account_address);

    // Generate Proof
    let input = args.input.abi_encode();
    let env = ExecutorEnv::builder().write_slice(&input).build()?;
    let groth16_proof_receipt = default_prover()
        .prove_with_ctx(
            env,
            &VerifierContext::default(),
            OWNER_VERIFY_ELF,
            &ProverOpts::groth16(),
        )?
        .receipt;
    let seal = encode_seal(&groth16_proof_receipt)?;

    // Execute Transaction
    let simple_account_contract = ISimpleAccount::new(simple_account_address, provider);
    let tx_data = simple_account_contract.execute(args.recipient, args.amount, seal.into());
    let pending_tx = runtime.block_on(tx_data.send())?;
    let final_receipt = runtime.block_on(pending_tx.get_receipt())?;
    println!("Transaction receipt: {:?}", final_receipt);

    Ok(())
}
