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
yarn run bundler --network sepoliafork --unsafe  
```

### Run browser extension app locally
1. 


### Running
