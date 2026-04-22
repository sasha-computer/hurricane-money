use alloy::primitives::{keccak256, B256};

// generate two random U256 numbers:
// 1. k, the nullifer
// 2. r, randomness
pub fn generate_commitment() -> B256 {
    let mut k = [0u8; 32];
    let mut r = [0u8; 32];

    getrandom::fill(&mut k).unwrap();
    getrandom::fill(&mut r).unwrap();

    let mut concat = [0u8; 64];
    concat[0..32].copy_from_slice(&k);
    concat[32..64].copy_from_slice(&r);

    let commitment: B256 = keccak256(&concat);

    println!("k: 0x{}", hex::encode(k));
    println!("r: 0x{}", hex::encode(r));
    println!("c: 0x{}", hex::encode(commitment));

    commitment
}
