use rs_merkle::{MerkleTree, algorithms::Sha256, Hasher};

fn main() {
    let leaf_values: [&str; 20] = ["hurricane"; 20];
    let leaves: Vec<[u8; 32]> = leaf_values
        .iter()
        .map(|x| Sha256::hash(x.as_bytes()))
        .collect();
    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);

    println!("root {:?}", merkle_tree.root());
}
