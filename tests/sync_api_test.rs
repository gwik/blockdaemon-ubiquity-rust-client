use std::{future::Future, io};

use ubiquity::api::sync_api;
use ubiquity::configuration;

use mockito;

pub mod utils;

struct Setup {
  url: String,
  mocks: Vec<mockito::Mock>,
}

fn setup() -> Result<Setup, io::Error> {
  let url = mockito::server_url();

  let block_number_bitcoin_mock = utils::create_mock_from_file(
    "./tests/mock_files/sync_api/sync_block_number_bitcoin.json",
    "/bitcoin/mainnet/sync/block_number",
  )?;
  let block_number_ethereum_mock = utils::create_mock_from_file(
    "./tests/mock_files/sync_api/sync_block_number_ethereum.json",
    "/ethereum/mainnet/sync/block_number",
  )?;

  let block_id_bitcoin_mock = utils::create_mock_from_file(
    "./tests/mock_files/sync_api/sync_block_id_bitcoin.json",
    "/bitcoin/mainnet/sync/block_id",
  )?;
  let block_id_ethereum_mock = utils::create_mock_from_file(
    "./tests/mock_files/sync_api/sync_block_id_ethereum.json",
    "/ethereum/mainnet/sync/block_id",
  )?;

  return Ok(Setup {
    url,
    mocks: vec![
      block_number_bitcoin_mock,
      block_number_ethereum_mock,
      block_id_bitcoin_mock,
      block_id_ethereum_mock,
    ],
  });
}

async fn sync_test<F, Fut>(func: F)
where
  F: Fn(configuration::Configuration) -> Fut,
  Fut: Future<Output = ()>,
{
  match setup() {
    Ok(setup_data) => {
      let _m = setup_data.mocks;

      let config = utils::config_from_url(setup_data.url);

      func(config).await;
    }
    Err(e) => panic!("{}", e),
  }
}

async fn test_current_block_id_with_config(config: configuration::Configuration) {
  let test_platform_pairs = [("bitcoin", "mainnet"), ("ethereum", "mainnet")];

  for (platform, network) in test_platform_pairs {
    let res = sync_api::current_block_id(&config, platform, network).await;
    match res {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    };
  }
}

async fn test_current_block_number_with_config(config: configuration::Configuration) {
  let test_platform_pairs = [("bitcoin", "mainnet"), ("ethereum", "mainnet")];

  for (platform, network) in test_platform_pairs {
    let res = sync_api::current_block_number(&config, platform, network).await;
    match res {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    };
  }
}

#[tokio::test]
async fn sync_block_id() {
  sync_test(test_current_block_id_with_config).await;
}

#[tokio::test]
async fn sync_block_number() {
  sync_test(test_current_block_number_with_config).await;
}
