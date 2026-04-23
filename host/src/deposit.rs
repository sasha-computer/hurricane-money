use crate::contract::Hurricane;
use alloy::{
    primitives::{keccak256, utils::parse_ether, B256},
    providers::Provider,
    rpc::types::TransactionReceipt,
};
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize)]
pub struct Note {
    #[serde(with = "hex::serde")]
    pub k: [u8; 32],
    #[serde(with = "hex::serde")]
    pub r: [u8; 32],
    #[serde(with = "hex::serde")]
    pub commitment: [u8; 32],
}

pub fn new() -> Result<Note> {
    println!("generating secrets k and r");
    let mut k = [0u8; 32];
    let mut r = [0u8; 32];

    getrandom::fill(&mut k)?;
    getrandom::fill(&mut r)?;

    println!("creating commitment");
    let commitment: B256 = keccak256([k, r].concat());

    let note = Note {
        k,
        r,
        commitment: commitment.into(),
    };
    save_note(&note)?;
    Ok(note)
}

pub async fn submit(
    hurricane: &Hurricane::HurricaneInstance<impl Provider + Clone>,
    note: &Note,
) -> Result<TransactionReceipt> {
    let receipt = hurricane
        .deposit(B256::from(&note.commitment))
        .value(parse_ether("1")?)
        .send()
        .await?
        .get_receipt()
        .await?;

    println!("deposit tx: {}", receipt.transaction_hash);
    println!(
        "block: {:?}",
        receipt
            .block_number
            .expect("anvil receipt should have block number")
    );
    Ok(receipt)
}

fn save_note(note: &Note) -> Result<()> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../notes");
    fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{}.json", hex::encode(&note.commitment[..8])));
    let json = serde_json::to_string_pretty(note)?;
    fs::write(&path, json)?;
    println!("saved k, r, commitment to note: {}", path.display());
    println!("---");
    Ok(())
}
