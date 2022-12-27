// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.0;

contract TestContract {
    uint256 public counter = 0;

    constructor() payable {}

    function increment() public {
        counter = counter + 1;
    }
}
