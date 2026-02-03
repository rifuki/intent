// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script, console} from "forge-std/Script.sol";
import {IntentGateway} from "../src/IntentGateway.sol";

contract DeployIntentGateway is Script {
    function run() external returns (IntentGateway) {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        
        vm.startBroadcast(deployerPrivateKey);
        
        IntentGateway gateway = new IntentGateway();
        
        console.log("IntentGateway deployed at:", address(gateway));
        
        vm.stopBroadcast();
        
        return gateway;
    }
}
