# Post-Quantum Wallet

A Proof-of-Concept for Post-Quantum Ethereum Security using RISC Zero and Account Abstraction. This concept was proposed by [Aayush Gupta](https://x.com/yush_g) in an [Ethereum Research post](https://ethresear.ch/t/quantum-proof-keypairs-with-ecdsa-zk/14901/2).

## Table of Contents

1. [Post-Quantum Wallet](#post-quantum-wallet)
2. [Overview](#overview)
3. [TODOs](#todos)
4. [Concept](#concept)
    - [Problem](#problem)
    - [Solution](#solution)
5. [Implementation](#implementation)
    - [Workflow](#workflow)
6. [Challenges](#challenges)
7. [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Running](#running)
8. [Development](#development)
9. [Project Structure](#project-structure)

## Overview

This project was designed to showcase an end-to-end post-quantum secure wallet using the [Trampoline browser extension](https://github.com/eth-infinitism/trampoline) and the [Bundler](https://github.com/eth-infinitism/bundler). While some integration parts remain challenging and require more time to complete, we focused on providing a functional CLI application, `cli`, for streamlined testing and experimentation.

The solution leverages Zero-Knowledge Proofs (ZKPs) to protect users' public keys from exposure on-chain, ensuring Ethereum security in a post-quantum era.

### TODOs

- [X] Run `anvil` local Sepolia hardfork node.
- [X] Implement ZKVM methods `methods` that has the Risc0 constraints.  
- [ ] Benchmarking (time-analysis) for the `methods` guests.
- [X] Implement ZKVM host `host` that has the host program that passes the private inputs to the guests.
- [X] Implement ZKVM test `test` that has an integration tests for the zkvm methods.
- [X] Implement a simple account contract for demo purposes `SimpleAccountDemo.sol`. 
- [X] Create CLI app `cli` commands for proof generation and submission.  
- [X] Verify SNARK (Groth16) proofs on-chain in `cli` app.
- [ ] Verify STARK (Post-Quantum) proofs on-chain in `cli` app.
- [ ] Investigate recursive proof techniques for large proofs.  
- [ ] Investigate `risc0-nova` for helping proving STARK proofs.  
- [X] Deploy `EntryPoint` contract and ensure compatibility.  
- [X] Deploy `SimpleAccountFactory` contract locally.
- [X] Setup and run a local `bundler` node for full e2e app implementation with the `Trampoline`.   
- [ ] (WIP) Set up browser extension `Trampoline` for `SimpleAccount` testing.    

## Concept
### Problem
Current Ethereum security relies on ECDSA, where public keys are exposed on-chain. In a post-quantum world, quantum computers could use these public keys to derive private keys, posing a severe security risk.

### Solution
Using RISC Zero STARK ZKPs:
1. Users prove ownership of a private key without revealing their public key.
2. Proofs are verified on-chain by a smart contract wallet using Account Abstraction.
3. This ensures quantum security while enabling transaction execution.

## Implementation

### Workflow
1. **Generate Proof**: A ZKP is created using RISC Zero to prove ownership of a private key associated with an Ethereum address.
2. **Submit Proof**: The proof is sent to a smart contract wallet.
3. **Verify Proof**: The wallet verifies the proof. If valid, the transaction is executed; otherwise, it is reverted.

This solution protects public keys on-chain, ensuring Ethereum remains secure even in a post-quantum computing world.

## Challenges

- **(WIP) Verifying STARK Proofs On-Chain**: STARK proofs are significantly large, requiring innovative approaches for efficient on-chain verification. Possible solutions include:
  - **Splitting Proofs**: Dividing the proof into multiple transactions to fit within gas limits.
  - **Recursive Compression**: Leveraging recursive techniques to compress the proofs into smaller, verifiable units. 
  - Exploring solutions like **`risc0-nova`** for efficient recursive proof generation and verification.

## Getting Started
### Prerequisites

To run the project, ensure the following tools and environments are set up:

- **Rust** (nightly version support)
- **Node.js** (version 18.0.0 or later)
- **Python** (version 3.9 or later)
- **RISC Zero CLI**
- **Foundry** (Ethereum development toolchain)
- **Local Ethereum Node** (for testing or connect to a public testnet using `anvil` for example)

### Running
1. **Run Local Node**  
   To test locally, you need to run a local Ethereum node using `anvil`. For this project, we recommend forking the Sepolia testnet using Alchemy.  
   ```shell
   anvil --fork-url https://shape-sepolia.g.alchemy.com/v2/<ALCHEMY_KEY>
   ```

2. **Install Submodules**  
   The contracts require some submodules to be initialized and updated.  
   ```shell
   cd contracts
   git submodule update --init --recursive
   ```

3. **Deploy `SimpleAccountDemo` Contract**  
   This demo contract simplifies the `SimpleAccount` architecture from Account Abstraction for testing purposes.  
   ```shell
   cd contracts
   forge script script/DeploySimpleAccountDemo.sol:SimpleAccountDemoDeploy --broadcast --private-key <PRIVATE_KEY>
   ```

4. **Expected Output**
   <details>
   <summary>Click to expand</summary>

   ```plaintext
   [⠊] Compiling...
   No files changed, compilation skipped
   Script ran successfully.

   == Logs ==
   You are deploying on ChainID 11011
   Local deployment condition: false
   Deployed RiscZeroGroth16Verifier to 0xE6E340D132b5f46d1e472DebcD681B2aBc16e57E
   Deployed local RiscZeroGroth16Verifier to: 0xE6E340D132b5f46d1e472DebcD681B2aBc16e57E
   ==> SimpleAccount initialized with owner: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
   Deployed SimpleAccountDemo to: 0xc3e53F4d16Ae77Db1c982e75a937B9f60FE63690

   ## Setting up 1 EVM.

   ==========================

   Chain 11011

   Estimated gas price: 1.00000002 gwei

   Estimated total gas used for script: 2240727

   Estimated amount required: 0.00224072704481454 ETH

   ==========================

   ##### 11011
   ✅  [Success]Hash: 0x38e657f097bf5c0f06db91531026fa85721ea20644e71d9807762ad2b3f7106e
   Contract Address: 0xc3e53F4d16Ae77Db1c982e75a937B9f60FE63690
   Block: 6625031
   Paid: 0.000523628004712652 ETH (523628 gas * 1.000000009 gwei)

   ##### 11011
   ✅  [Success]Hash: 0x687fb751abd2b01537506d136155a363ad4071b3e6e40248f80aa361e633ed51
   Contract Address: 0xE6E340D132b5f46d1e472DebcD681B2aBc16e57E
   Block: 6625031
   Paid: 0.001200785010807065 ETH (1200785 gas * 1.000000009 gwei)

   ✅ Sequence #1 on 11011 | Total Paid: 0.001724413015519717 ETH (1724413 gas * avg 1.000000009 gwei)
                                                                     

   ==========================

   ONCHAIN EXECUTION COMPLETE & SUCCESSFUL.

   Transactions saved to: /home/isk/Projects/risc0/post-quantum-wallet/contracts/broadcast/DeploySimpleAccountDemo.sol/11011/run-latest.json

   Sensitive values saved to: /home/isk/Projects/risc0/post-quantum-wallet/contracts/cache/DeploySimpleAccountDemo.sol/11011/run-latest.json
   ```
   </details>

5. **Run Publisher App**  
   The `publisher` app generates a ZK proof (currently SNARK) and verifies it on-chain.  
   ```shell
   RUST_LOG=info cargo run --bin publisher --release -- \
     --chain-id 11011 \
     --eth-wallet-private-key <PRIVATE_KEY> \
     --rpc-url http://127.0.0.1:8545 \
     --contract <SIMPLE_ACCOUNT_DEMO_CONTRACT_ADDRESS> \
     --public-key <PUBLIC_KEY> \
     --expected-address <EXPECTED_OWNER_ADDRESS> \
     --recipient <RECIPIENT_ADDRESS> \
     --amount 1000000000000000000
   ```

   ```shell
      # example

      RUST_LOG=info cargo run --bin publisher --release -- \
        --chain-id 11011 \
        --eth-wallet-private-key "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" \
        --rpc-url http://127.0.0.1:8545 \
        --contract 0x2279B7A0a67DB372996a5FaB50D91eAA73d2eBe6 \
        --public-key "8318535b54105d4a7aae60c08fc45f9687181b4fdfc625bd1a753fa7397fed753547f11ca8696646f2f3acb08e31016afac23e630c5d11f59f61fef57b0d2aa5" \
        --expected-address 0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266 \
        --recipient 0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC \
        --amount 1000000000000000000
   ```

6. **Configure Publisher App**  
   By default, the app uses hardcoded values for testing (e.g., the first address from the `anvil` node). Update these values as needed.  
   ```rust
   // Generate Proof
   let public_key = "8318535b54105d4a7aae60c08fc45f9687181b4fdfc625bd1a753fa7397fed753547f11ca8696646f2f3acb08e31016afac23e630c5d11f59f61fef57b0d2aa5".to_string();
   let expected_address = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266".to_string();
   ```

### Development

1. **Install Submodules**  
   The project requires initializing and updating submodules before proceeding.  
   ```shell
   git submodule update --init --recursive
   ```

2. **Deploy `EntryPoint` Contract Locally**  

   - **Step 1:** Navigate to the bundler package directory and install dependencies.  
     ```shell
     cd packages/bundler
     yarn
     ```
   - **Step 2:** Deploy the `EntryPoint` contract to the local Sepolia hard-fork network using `anvil`.  
     ```shell
     yarn hardhat-deploy --network sepoliaFork
     ```
   - **Expected Output:**  
     <details>
     <summary>Click to expand</summary>

     ```plaintext
     Nothing to compile
     No need to generate any newer typings.
     EntryPoint already deployed at 0x5ff137d4b0fdcd49dca30c7cf57e578a026d2789
     ```
     </details>

   - **Step 3:** Copy the deployed `EntryPoint` contract address from the output.  
   - **Step 4:** Update the bundler configuration file with the new `EntryPoint` address.  
     ```shell
     vim packages/bundler/localconfig/bundler.config.json
     ```
     Replace the `"entryPoint"` field with the new address:  
     `"entryPoint": "0x5ff137d4b0fdcd49dca30c7cf57e578a026d2789"`

3. **Run Bundler Locally**  

   - **Step 1:** Preprocess and prepare the bundler.  
     ```shell
     cd packages/bundler
     yarn && yarn preprocess
     ```
   - **Step 2:** Start the bundler on the local Sepolia hard-fork node.  
     ```shell
     yarn run bundler --network sepoliaFork --unsafe
     ```

4. **Deploy `SimpleAccountFactory` Contract**  

   - **Step 1:** Navigate to the contracts package and deploy the contract.  
     ```shell
     cd packages/contracts
     forge script script/DeployFactory.sol:DeployFactory --broadcast --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
     ```
     *(The private key provided is for the first address in the local node and is used for testing purposes.)*

   - **Expected Output:**  
     <details>
     <summary>Click to expand</summary>

     ```plaintext
     ==> Factory created!
     SimpleAccountFactory deployed at: 0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9
     ```
     </details>

5. **Run Browser Extension App Locally**  

   - **Step 1:** Update the browser extension configuration with the new `EntryPoint` and `SimpleAccountFactory` addresses.  
     ```shell
     vim packages/app/src/exconfig.ts
     ```  
   - **Step 2:** Install dependencies and start the browser extension.  
     ```shell
     cd packages/app
     yarn
     yarn start
     ```

## Project structure
Below are the primary files in the project directory

```shell
├── Cargo.lock                 
├── Cargo.toml                 
├── contracts                  # Solidity smart contracts and related tools.
│   ├── broadcast              
│   ├── cache                  
│   ├── foundry.toml           
│   ├── lib                    
│   ├── out                    
│   ├── README.md              
│   ├── remappings.txt         
│   ├── script                 
│   ├── src                    
│   └── test                   
├── draft                      # Experimental and unfinished subprojects or components.
│   ├── aa-contracts           # Draft Account Abstraction contracts.
│   ├── bundler                # Draft bundler service for AA transactions.
│   └── trampoline             # Draft browser extension integration for AA wallets.
├── LICENSE                    
├── README.md                  
├── rust-toolchain.toml        # Rust toolchain configuration for project consistency.
├── cli                   # CLI application for generating and submitting proofs.
│   ├── Cargo.toml             
│   └── src                    
├── core                  # Core library for zkVM-based operations.
│   ├── Cargo.toml             
│   └── src                    
├── host                  # Host-side logic for interacting with zkVM.
│   ├── Cargo.toml             
│   ├── proof-groth16.txt      
│   ├── proof-succinct.txt     
│   └── src                   
├── methods               # Methods for zkVM computation and proof generation.
│   ├── build.rs               
│   ├── Cargo.toml             
│   ├── guest                  
│   ├── README.md              
│   └── src                   
└── test                  # Integration tests for zkVM functionality.
    ├── Cargo.toml             
    └── src                    

28 directories, 18 files
```