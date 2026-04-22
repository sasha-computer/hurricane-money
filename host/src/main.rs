use alloy::{
    network::EthereumWallet,
    node_bindings::{Anvil, AnvilInstance},
    primitives::{utils::parse_ether, B256},
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

    let hurricane = Hurricane::deploy(provider.clone()).await?;
    println!("deployed at {}", hurricane.address());

    get_contract_root(&hurricane).await?;
    // TODO: want to assert that root at deployment is correct via rs-merkle crate

    let commitment: B256 = deposit::generate_commitment();

    let receipt = hurricane
        .deposit(commitment)
        .value(parse_ether("1")?)
        .send()
        .await?
        .get_receipt()
        .await?;

    println!("deposit tx: {}", receipt.transaction_hash);
    println!("block: {:?}", receipt.block_number.unwrap());

    get_contract_root(&hurricane).await?;

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

    println!("Node info: {info:#?}");

    Ok((anvil, provider, info))
}

async fn get_contract_root(
    hurricane: &Hurricane::HurricaneInstance<impl Provider + Clone>,
) -> Result<()> {
    let root = hurricane.root().call().await?;
    println!("current root: {root}");
    Ok(())
}
