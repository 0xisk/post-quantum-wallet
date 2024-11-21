// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.23;

import {Script} from "forge-std/Script.sol";
import "forge-std/Test.sol";
import {RiscZeroCheats} from "../lib/risc0-ethereum/contracts/src/test/RiscZeroCheats.sol";
import {IRiscZeroVerifier} from "../lib/risc0-ethereum/contracts/src/IRiscZeroVerifier.sol";
import {RiscZeroGroth16Verifier} from "../lib/risc0-ethereum/contracts/src/groth16/RiscZeroGroth16Verifier.sol";

import {SimpleAccountDemo} from "../src/SimpleAccountDemo.sol";

/// @notice Deployment script for the SimpleAccountDemo contract with local verifier deployment.
contract SimpleAccountDemoDeploy is Script, RiscZeroCheats {
    IRiscZeroVerifier verifier;

    function run() external {
        // Start deployment
        vm.startBroadcast();

        // Read the chainID to determine if this is a local deployment
        uint256 chainId = block.chainid;
        console2.log("You are deploying on ChainID %d", chainId);

        // Check if we are deploying locally (e.g., Hardhat or Foundry forked chains usually have low chain IDs like 31337)
        bool isLocalDeployment = (chainId == 31337 || chainId == 1337);
        console2.log("Local deployment condition:", isLocalDeployment);

        // Deploy the verifier locally if no address is set and this is a local deployment
        if (isLocalDeployment || address(verifier) == address(0)) {
            verifier = deployRiscZeroVerifier();
            console2.log(
                "Deployed local RiscZeroGroth16Verifier to:",
                address(verifier)
            );
        } else {
            console2.log(
                "Using existing IRiscZeroVerifier contract at:",
                address(verifier)
            );
        }

        // Define the owner for the SimpleAccountDemo contract
        address owner = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266;
        require(
            owner != address(0),
            "OWNER_ADDRESS must be set in environment variables"
        );

        // Deploy the SimpleAccountDemo contract
        SimpleAccountDemo demo = new SimpleAccountDemo(owner, verifier);
        console2.log("Deployed SimpleAccountDemo to:", address(demo));

        // Stop broadcasting transactions
        vm.stopBroadcast();
    }
}
