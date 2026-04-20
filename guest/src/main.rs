/* 
computation we're aiming for:
processMerkleProof(MerkleProof, leaf) == root 

where leaf = hash(concat(k, r))
*/

use openvm::io::{read, reveal_bytes32};
use openvm_sha2::sha256;

fn main() {
    let k: u8 = read();
    let r: u8 = read();

    let c: [u8; 32] = sha256(&[k, r]);

    reveal_bytes32(c)
}
