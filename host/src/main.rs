use alloy::primitives::U256;
use rand::RngExt;
use rs_merkle::{algorithms::Sha256, Hasher, MerkleProof, MerkleTree};
pub struct Note {
    pub k: [u8; 31], // nullifier, 248 bits
    pub r: [u8; 31], // randomness, 248 bits
}

pub struct Commitment {
    hash: U256,
}

// input: note (preimage of nullifier and secret concatenated)
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let merkle_tree = build_merkle_tree(&["hurricane"; 20]);
    // let merkle_proof = build_merkle_proof(&merkle_tree);

    // let mut rng = rand::rng();

    // println!("Random UUID: 0x{:X}", rng.random::<u128>());

    let indices_to_prove = vec![3, 4];
}

fn build_merkle_tree(leaf_values: &[&str]) -> MerkleTree<Sha256> {
    let leaves: Vec<[u8; 32]> = leaf_values
        .iter()
        .map(|x| Sha256::hash(x.as_bytes()))
        .collect();

    MerkleTree::<Sha256>::from_leaves(&leaves)
}

// fn build_merkle_proof(
//     merkle_tree: &MerkleTree<Sha256>,
//     commitment: Commitment,
// ) -> MerkleProof<Sha256> {
// }

fn process_note(note: Note) {}

fn generate_commitment() {
    let mut rng = rand::rng();
    let k: [u8; 31] = rng.random();
}

// build Merkle Tree (ultiamtely from on-chain)
// take input as leaf to prove, remembering `processMerkleProof(leaf, MerkleProof) to get MerkleProof, so generateMerkleProof -> proof bytes
// these bytes get passed into the ZK guest, so that we can prove C is in fact built from their secrets in the guest and do the rest

/*
let leaf_values = ["a", "b", "c", "d", "e", "f"];
let leaves: Vec<[u8; 32]> = leaf_values
    .iter()
    .map(|x| Sha256::hash(x.as_bytes()))
    .collect();

let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
let indices_to_prove = vec![3, 4];
let leaves_to_prove = leaves.get(3..5).ok_or("can't get leaves to prove")?;
let merkle_proof = merkle_tree.proof(&indices_to_prove);
let merkle_root = merkle_tree.root().ok_or("couldn't get the merkle root")?;
// Serialize proof to pass it to the client
let proof_bytes = merkle_proof.to_bytes();

// Parse proof back on the client
let proof = MerkleProof::<Sha256>::try_from(proof_bytes)?;

assert!(proof.verify(merkle_root, &indices_to_prove, leaves_to_prove, leaves.len()));
*/

// get leaves
/*

let leaves = [
    Sha256::hash("a".as_bytes()),
    Sha256::hash("b".as_bytes()),
    Sha256::hash("c".as_bytes()),
];

let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
assert_eq!(merkle_tree.leaves(), Some(leaves.to_vec()));

*/
