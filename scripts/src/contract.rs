use boot_core::networks::{ChainInfo, NetworkInfo, NetworkKind};
use boot_core::prelude::*;
use std::sync::Arc;
// Traits for contract deployment
use interfaces::contract::ToDoContract;
// Select the chain to deploy to
const ARCHWAY_CHAIN: ChainInfo = ChainInfo {
    chain_id: "archway",
    pub_address_prefix: "archway",
    coin_type: 118u32,
};
const LOCAL_ARCHWAY: NetworkInfo = NetworkInfo {
    kind: NetworkKind::Local,
    id: "localarchway",
    gas_denom: "uarch",
    gas_price: 0.0,
    grpc_urls: &["http://0.0.0.0:9090"],
    chain_info: ARCHWAY_CHAIN,
    lcd_url: None,
    fcd_url: None,
};
const CONTRACT_NAME: &str = "to-do-dapp";

// Requires a running local archwayd with grpc enabled
pub fn deploy() -> anyhow::Result<()> {
    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());

    // use the cosmos chain registry for gRPC url sources.
    // let chain_data = rt.block_on( ChainData::fetch("juno".into(), None))?;
    // let (sender, chain) = instantiate_daemon_env(&rt,chain_data)?;

    // First we upload, instantiate and interact with a real chain
    let network = LOCAL_ARCHWAY;

    let options = DaemonOptionsBuilder::default()
        // or provide `chain_data`
        .network(network)
        .deployment_id("boot_showcase")
        .build()?;

    let (_sender, chain) = instantiate_daemon_env(&rt, options)?;
    let mut to_do_list = ToDoContract::new(CONTRACT_NAME, chain);
    to_do_list.upload()?;

    Ok(())
}

pub fn instantiate() -> anyhow::Result<()> {
    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());

    // use the cosmos chain registry for gRPC url sources.
    // let chain_data = rt.block_on( ChainData::fetch("juno".into(), None))?;
    // let (sender, chain) = instantiate_daemon_env(&rt,chain_data)?;

    // First we upload, instantiate and interact with a real chain
    let network = LOCAL_ARCHWAY;

    let options = DaemonOptionsBuilder::default()
        // or provide `chain_data`
        .network(network)
        .deployment_id("boot_showcase")
        .build()?;

    let (_sender, chain) = instantiate_daemon_env(&rt, options)?;
    let to_do_list = ToDoContract::new(CONTRACT_NAME, chain);
    to_do_list.create_new()?;

    Ok(())
}
