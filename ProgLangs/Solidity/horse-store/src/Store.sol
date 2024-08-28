// SPDX-License-Identifier: GPL-3.0-only
pragma solidity 0.8.20;


contract Store {
  uint256 number;

  function setNumber(uint256 newNumber) external {
    number = newNumber;
  }

  function readNumber() external view returns (uint256) {
    return number;
  }
}
