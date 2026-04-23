/*
computation we're aiming for:
processMerkleProof(MerkleProof, leaf) == root

where leaf = hash(concat(k, r))
*/

use openvm::io::{read, reveal_bytes32};
use openvm_keccak256::keccak256;

fn main() {
    let k: [u8; 32] = read();
    let r: [u8; 32] = read();

    let c: [u8; 32] = keccak256(&[k, r].concat());

    reveal_bytes32(c)
}
