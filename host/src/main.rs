use alloy::{
    network::EthereumWallet,
    node_bindings::{Anvil, AnvilInstance},
    primitives::B256,
    providers::{ext::AnvilApi, Provider, ProviderBuilder},
    rpc::types::anvil::NodeInfo,
    signers::local::PrivateKeySigner,
    sol,
};
use eyre::Result;

mod deposit;

sol!(
    #[sol(rpc)]
    Hurricane,
    "../contracts/out/Hurricane.sol/Hurricane.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    let (_anvil, provider, _info) = spawn_anvil().await?;
    let _commitment: B256 = deposit::generate_commitment();
    deploy_hurricane(&provider).await?;

    Ok(())
}

async fn spawn_anvil() -> Result<(AnvilInstance, impl Provider, NodeInfo)> {
    let anvil = Anvil::new().block_time(1).chain_id(1337).try_spawn()?;
    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    let wallet = EthereumWallet::from(signer);
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect_http(anvil.endpoint_url());
    let info = provider.anvil_node_info().await?;

    assert_eq!(info.environment.chain_id, 1337);
    assert_eq!(info.fork_config.fork_url, None);

    println!("Node info: {info:#?}");

    Ok((anvil, provider, info))
}

async fn deploy_hurricane(provider: &impl Provider) -> Result<()> {
    let hurricane = Hurricane::deploy(&provider).await?;
    println!("deployed at {}", hurricane.address());
    Ok(())
}
