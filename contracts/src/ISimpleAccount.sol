// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.23;

interface ISimpleAccount {
    function entryPoint() external view returns (address);

    function execute(address dest, uint256 value, bytes calldata func) external;

    function executeBatch(
        address[] calldata dest,
        uint256[] calldata value,
        bytes[] calldata func
    ) external;

    function initialize(address anOwner) external;

    function getDeposit() external view returns (uint256);

    function addDeposit() external payable;

    function withdrawDepositTo(
        address payable withdrawAddress,
        uint256 amount
    ) external;
}
