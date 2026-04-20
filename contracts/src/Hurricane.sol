// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.34;

import {MerkleTree} from "@openzeppelin/contracts/utils/structs/MerkleTree.sol";
import {MerkleProof} from "@openzeppelin/contracts/utils/cryptography/MerkleProof.sol";

contract Hurricane {
    using MerkleTree for MerkleTree.Bytes32PushTree;

    MerkleTree.Bytes32PushTree private tree;
    bytes32 public root;

    constructor() {
        root = tree.setup(20, keccak256("hurricane"));
    }

    function deposit() public {}

    function withdraw() public {}
}
