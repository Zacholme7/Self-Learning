
// SPDX-License-Identifier: GPL-3.0-only
pragma solidity 0.8.20;

import {Base_Test, Store} from "./Base_Test.t.sol";
import {HuffDeployer} from "foundry-huff/HuffDeployer.sol";

contract StoreTestHuff is Base_Test {
  string public constant location = "Store";
  function setUp() public override {
    store = Store(HuffDeployer.config().deploy(location));
  }
}

