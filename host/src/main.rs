use alloy::{
    network::EthereumWallet,
    node_bindings::{Anvil, AnvilInstance},
    providers::{ext::AnvilApi, Provider, ProviderBuilder},
    rpc::types::anvil::NodeInfo,
    signers::local::PrivateKeySigner,
};
use eyre::Result;

mod contract;
use contract::Hurricane;
mod deposit;
mod withdraw;

#[tokio::main]
async fn main() -> Result<()> {
    let (_anvil, provider, _info) = spawn_anvil().await?;
    let hurricane = deploy_hurricane(provider).await?;
    let note = deposit::new()?;
    let _tx_receipt = deposit::submit(&hurricane, &note).await?;
    get_contract_root(&hurricane, "merkle root after deposit").await?;

    withdraw::run_program(&note)?;

    Ok(())
}

async fn spawn_anvil() -> Result<(AnvilInstance, impl Provider + Clone, NodeInfo)> {
    let anvil = Anvil::new().block_time(1).chain_id(1337).try_spawn()?;
    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    let wallet = EthereumWallet::from(signer);
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect_http(anvil.endpoint_url());
    let info = provider.anvil_node_info().await?;

    assert_eq!(info.environment.chain_id, 1337);
    assert_eq!(info.fork_config.fork_url, None);

    println! {"anvil spawned!"};
    println!("chain_id: {}", info.environment.chain_id);
    println!("rpc: {}", anvil.endpoint_url());
    println!("block: {}", info.current_block_number);
    println!("---");

    Ok((anvil, provider, info))
}

async fn deploy_hurricane(
    provider: impl Provider + Clone,
) -> Result<Hurricane::HurricaneInstance<impl Provider + Clone>> {
    let hurricane = Hurricane::deploy(provider.clone()).await?;
    println!("deployed at {}", hurricane.address());
    get_contract_root(&hurricane, "merkle root at deployment").await?;
    println!("---");
    Ok(hurricane)
}

async fn get_contract_root(
    hurricane: &Hurricane::HurricaneInstance<impl Provider + Clone>,
    label: &str,
) -> Result<()> {
    let root = hurricane.root().call().await?;
    println!("{label} : {root}");
    Ok(())
}
