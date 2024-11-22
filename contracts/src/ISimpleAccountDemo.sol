// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.23;

interface ISimpleAccountDemo {
    /// @notice Event emitted when the account is initialized.
    /// @param owner The address of the account owner.
    event SimpleAccountInitialized(address indexed owner);

    /// @notice Event emitted when a transaction is executed.
    /// @param target The target address of the transaction.
    /// @param value The value sent in the transaction.
    /// @param data The calldata sent in the transaction.
    event Execute(address indexed target, uint256 value, bytes data);

    /// @notice Get the owner of the account.
    /// @return The address of the owner.
    function owner() external view returns (address);

    /**
     * @notice Execute a transaction if the proof is valid.
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
    ) external;

    /**
     * @notice Internal function to validate the proof and the journal.
     * Exposed for testing or external validation purposes.
     * @param seal The proof data.
     * @param checked The owner checked address.
     * @return Whether the proof is valid or not.
     */
    function _validateProof(
        bytes calldata seal,
        address checked
    ) external view returns (bool);
}
