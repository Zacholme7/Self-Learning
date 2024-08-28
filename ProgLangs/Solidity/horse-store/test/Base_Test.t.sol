// SPDX-License-Identifier: GPL-3.0-only
pragma solidity 0.8.20;

import {Store} from "../src/Store.sol";
import {Test, console2} from "forge-std/Test.sol";

abstract contract Base_Test is Test {
    Store public store;

    function setUp() public virtual {
        store = new Store();
    }

    function testReadValue() public view {
        uint256 initialValue = store.readNumber();
        assertEq(initialValue, 0);
    }

    function testWriteValue(uint256 number) public {
      store.setNumber(number);
      assertEq(store.readNumber(), number);
    }
}


