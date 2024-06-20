// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {SP1Verifier} from "../src/SP1Verifier.sol";

contract SP1VerifierTest is Test {
    SP1Verifier public verifier;

    function setUp() public {
        verifier = new SP1Verifier();
    }
}
