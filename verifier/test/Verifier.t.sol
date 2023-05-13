// SPDX-License-Identifier: GPL-3
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/BabyItsMe.sol";

contract VerifierTest is Test {
    BabyItsMe public verifier;

    address constant me = address(0);
    uint256 constant myMsg = 0;

    function setUp() public {
        verifier = new BabyItsMe();
    }

    function testGenerate() public {
        assertEq(verifier.generate(me), myMsg);
    }

    function testProof() public {
        Verifier.Proof memory proof;
        proof.a.X = 0;
        proof.a.Y = 0;
        proof.b.X = [uint256(0), 0];
        proof.b.Y = [uint256(0), 0];
        proof.c.X = 0;
        proof.c.Y = 0;

        vm.startPrank(me);
        assertTrue(verifier.verifyProof(proof));
        assertTrue(verifier.verify(myMsg, uint256(uint160(me))));
    }
}
