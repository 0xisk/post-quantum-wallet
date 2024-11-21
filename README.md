# Post-Quantum Wallet
Post-Quantum Wallet using Risc0 and Account-Abstraction

## Getting Started

### Prerequisite
#### Install Submodules

#### Run local node
1. To be able to run it locally you need to run a local node using `anvil`, I have been using Sepolia hard-fork network from Alchemy.

```shell
anvil --fork-url https://shape-sepolia.g.alchemy.com/v2/<ALCHEMY_KEY>
```

#### Deploy `EntryPoint` contract locally
1. Run the following commands.
```shell
cd packages/bundler
yarn
# `sepoliaFork` is the Local Sepholia hardfork node  
yarn hardhat-deploy --network sepoliaFork 
```
2. You should be able to see that expected output.
```shell
$ yarn hardhat-deploy --network sepoliaFork                                                       1 ✘  5s  
yarn run v1.22.19
$ lerna run hardhat-deploy --stream --no-prefix -- --network sepoliaFork
lerna notice cli v5.6.2
lerna info Executing command in 1 package: "yarn run hardhat-deploy --network sepoliaFork"
$ hardhat deploy --network sepoliaFork
Nothing to compile
No need to generate any newer typings.
EntryPoint already deployed at 0x5ff137d4b0fdcd49dca30c7cf57e578a026d2789
lerna success run Ran npm script 'hardhat-deploy' in 1 package in 1.4s:
lerna success - @account-abstraction/bundler
Done in 1.62s.
```
3. Copy the `entrypoint` contract address and assign it to the bundler localconfig:
```shell
cd packages/bundler/packages/bundler/localconfig
vim bundler.config.json # Or with using any editor
```
4. Change `"entryPoint": "0x5ff137d4b0fdcd49dca30c7cf57e578a026d2789"` with your new address. 

#### Run bundler locally
1. Run the following commands:
```shell
cd packages/bundler

yarn && yarn preprocess

yarn run bundler --network sepoliafork --unsafe  
```

### Deploy `SimpleAccountFactory` contract
1. Run the following commands:
```shell
cd packages/contracts
forge script script/DeployFactory.sol:DeployFactory --broadcast --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 # That is a private key for the first provided address in the local node -for testing purposes-
```

2. You should be able to get an output similar to this.
```shell
$ forge script script/DeployFactory.sol:DeployFactory --broadcast --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

[⠊] Compiling...
No files changed, compilation skipped
Script ran successfully.

== Logs ==
  ==> Factory created!
  SimpleAccountFactory deployed at: 0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9

## Setting up 1 EVM.

==========================

Chain 11011

Estimated gas price: 0.000000359 gwei

Estimated total gas used for script: 2382032

Estimated amount required: 0.000000000855149488 ETH

==========================

##### 11011
✅  [Success]Hash: 0x34620212732576258e2ee77e30beb26e299d60da9eb4aaf1303b8ff8439f215c
Contract Address: 0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9
Block: 6617784
Paid: 0.000000000295158563 ETH (1833283 gas * 0.000000161 gwei)

✅ Sequence #1 on 11011 | Total Paid: 0.000000000295158563 ETH (1833283 gas * avg 0.000000161 gwei)
                                                                                                                                                                    
==========================

ONCHAIN EXECUTION COMPLETE & SUCCESSFUL.

Transactions saved to: /home/isk/Projects/risc0/post-quantum-wallet/packages/contracts/broadcast/DeployFactory.sol/11011/run-latest.json

Sensitive values saved to: /home/isk/Projects/risc0/post-quantum-wallet/packages/contracts/cache/DeployFactory.sol/11011/run-latest.json
```

### Run browser extension app locally
1. Change the configs of the browser extension to be using the new addresses of `EntryPoint` and `SimpleAccountFactory` contracts.
```shell
cd packages/app
vim src/excofig.ts
``` 
2. Run the following commands:
```shell
yarn 

yarn start
```


### Running
