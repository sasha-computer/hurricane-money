// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.34;

import {MerkleTree} from "@openzeppelin/contracts/utils/structs/MerkleTree.sol";
import {MerkleProof} from "@openzeppelin/contracts/utils/cryptography/MerkleProof.sol";

contract Hurricane {
    using MerkleTree for MerkleTree.Bytes32PushTree;
    MerkleTree.Bytes32PushTree private tree;
    bytes32 public constant HURRICANE_SALT = keccak256("hurricane");
    bytes32 public root;

    event Deposit(
        bytes32 indexed commitment,
        uint256 amount,
        uint256 leafIndex,
        bytes32 previousRoot,
        bytes32 newRoot
    );

    constructor() {
        root = tree.setup(20, HURRICANE_SALT);
    }

    function deposit(bytes32 commitment) external payable {
        require(
            msg.value == 0.1 ether || msg.value == 1 ether,
            "Invalid amount: must be 0.1 or 1 ETH"
        );
        require(
            commitment != HURRICANE_SALT,
            "Invalid commitment: must not be keccak256('hurricane')"
        );

        (uint256 newLeafIndex, bytes32 newRoot) = tree.push(commitment);

        emit Deposit(commitment, msg.value, newLeafIndex, root, newRoot);

        root = newRoot;
    }

    // accepts openVM proof of `processMerkleProof(MerkleProof, leaf(hash(concat(k,r)))`
    // verifies it
    // also should accept
    // function withdraw() public {}
}
