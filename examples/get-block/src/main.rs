use ubiquity::api::blocks_api;
use ubiquity::configuration;
use ubiquity::models::block;

use std::env;

fn print_block_result(block: &block::Block) -> Option<()> {
    let block_id = block.id.as_ref()?;
    let block_txs = block.txs.as_ref()?;

    return Some(println!(
        "Latest block has hash {} and {} transactions",
        block_id,
        block_txs.len()
    ));
}

#[tokio::main]
async fn main() {
    let token = env::var("ACCESS_TOKEN").unwrap_or("".to_string());
    let conf = configuration::Configuration {
        bearer_access_token: Some(token),
        ..configuration::Configuration::new()
    };

    let platform = "ethereum";
    let network = "mainnet";
    let ident = "current";

    let block_result = blocks_api::get_block(&conf, platform, network, ident).await;

    match block_result {
        Ok(block) => print_block_result(&block),
        Err(e) => panic!("{}", e),
    };
}
