// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.23;

import "forge-std/Script.sol";
import "../src/SimpleAccountFactory.sol";
import "../src/SimpleAccount.sol";

contract DeployFactory is Script {
    function run() external {
        // Set the deployer address (e.g., the first Anvil account)
      address deployer = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266; // Checksummed address // Replace with a default unlocked account

        // Start broadcasting transactions
        vm.startBroadcast(deployer);

        // Deploy the SimpleAccountFactory
        IEntryPoint entryPoint = IEntryPoint(
            0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789
        ); // Replace with actual entry point
        SimpleAccountFactory factory = new SimpleAccountFactory(entryPoint);

        console.log("SimpleAccountFactory deployed at:", address(factory));

        // Stop broadcasting
        vm.stopBroadcast();
    }
}
