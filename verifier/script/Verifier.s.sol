// SPDX-License-Identifier: GPL-v3
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
import "src/BabyItsMe.sol";

contract BabyItsMeScript is Script {
    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        BabyItsMe ctf = new BabyItsMe();

        vm.stopBroadcast();
    }
}
