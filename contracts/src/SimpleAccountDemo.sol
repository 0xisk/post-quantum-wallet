// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.23;

import "forge-std/console.sol";
import {IRiscZeroVerifier} from "../lib/risc0-ethereum/contracts/src/IRiscZeroVerifier.sol";
import {ImageID} from "./ImageID.sol";

/**
 * A simplified account contract.
 * Validates ownership and proof before executing transactions.
 */
contract SimpleAccountDemo {
    /// @notice RISC Zero verifier contract address.
    IRiscZeroVerifier public immutable verifier;

    bytes32 public constant imageId = ImageID.OWNER_VERIFY_ID;

    address public owner;

    event SimpleAccountInitialized(address indexed owner);
    event Executed(address indexed target, uint256 value, bytes data);

    modifier onlyOwner() {
        require(msg.sender == owner, "Not the owner");
        _;
    }

    /**
     * Initialize the account with an owner.
     * Can only be called once.
     */
    constructor(address anOwner, IRiscZeroVerifier _verifier) {
        require(anOwner != address(0), "Invalid owner");
        owner = anOwner;
        verifier = _verifier;
        console.log("==> SimpleAccount initialized with owner:", owner);
        emit SimpleAccountInitialized(owner);
    }

    /**
     * Executed a transaction if the proof is valid.
     * @param target The target address for the call.
     * @param value The value to transfer in the call.
     * @param data The calldata to send in the call.
     * @param seal The proof to validate the transaction.
     * @param checked The owner checked address.
     */
    function execute(
        address target,
        uint256 value,
        bytes calldata data,
        bytes calldata seal,
        address checked
    ) external onlyOwner {
        require(_validateProof(seal, checked), "Invalid proof");
        console.log("==> SimpleAccount execute called with target:", target);
        emit Executed(target, value, data);
    }

    /**
     * Internal function to validate the proof and the journal.
     * @param seal The proof data.
     * @param checked The owner checked address.
     * @return Whether the proof is valid or not.
     */
    function _validateProof(
        bytes memory seal,
        address checked
    ) internal view returns (bool) {
        require(checked == owner, "NOT RIGHT OWNER!");
        bytes memory journal = abi.encode(checked);

        // Verify the proof using the verifier contract
        verifier.verify(seal, imageId, sha256(journal));

        console.log("Proof validated successfully");
        return true;
    }

    /**
     * Internal function to execute a call.
     * @param target The target address.
     * @param value The value to send.
     * @param data The calldata to send.
     */
    function _call(address target, uint256 value, bytes memory data) internal {
        (bool success, bytes memory result) = target.call{value: value}(data);
        require(success, string(result));
    }
}
