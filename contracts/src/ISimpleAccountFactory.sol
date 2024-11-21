// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.23;

interface ISimpleAccountFactory {
    function accountImplementation() external view returns (address);

    function createAccount(
        address owner,
        uint256 salt
    ) external returns (address);

    function simpleCreateAccount(
        address owner,
        uint256 salt
    ) external returns (address);

    function getAddress(
        address owner,
        uint256 salt
    ) external view returns (address);
}
