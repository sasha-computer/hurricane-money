use alloy::{
    node_bindings::{Anvil, AnvilInstance},
    providers::{ext::AnvilApi, Provider, ProviderBuilder},
};
use eyre::{Ok, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let (_anvil, provider) = spawn_anvil()?;
    let info = provider.anvil_node_info().await?;

    println!("Node info: {info:#?}");

    assert_eq!(info.environment.chain_id, 1337);
    assert_eq!(info.fork_config.fork_url, None);

    Ok(())
}

fn spawn_anvil() -> Result<(AnvilInstance, impl Provider)> {
    let anvil = Anvil::new().block_time(1).chain_id(1337).try_spawn()?;
    let provider = ProviderBuilder::new().connect_http(anvil.endpoint_url());
    Ok((anvil, provider))
}
