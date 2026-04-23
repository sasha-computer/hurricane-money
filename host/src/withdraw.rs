// process_merkle_proof(merkle_proof, hash(concat(k,r))) == root

// run above computation in zkVM guest, get proof + public outputs, send onchain to withdraw function

use crate::deposit::Note;
use eyre::Result;
use openvm_build::GuestOptions;
use openvm_sdk::{config::SdkVmConfig, Sdk, StdIn};
use std::path::PathBuf;

pub fn run_program(note: &Note) -> Result<()> {
    let sdk = Sdk::new(SdkVmConfig::from_toml(include_str!(
        "../../guest/openvm.toml"
    ))?)?;

    let guest_opts = GuestOptions::default();
    let guest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../guest");

    let elf = sdk.build(guest_opts, &guest_path, &None, None)?;

    let mut stdin = StdIn::default();
    stdin.write(&note.k);
    stdin.write(&note.r);

    let output = sdk.execute(elf.clone(), stdin.clone())?;
    println!("public values output: {output:?}");

    Ok(())
}
