// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.34;

import {Script} from "forge-std/Script.sol";
import {Hurricane} from "../src/Hurricane.sol";

contract Deploy is Script {
    function run() public {
        vm.startBroadcast();

        Hurricane hurricane = new Hurricane();

        vm.stopBroadcast();
    }
}
