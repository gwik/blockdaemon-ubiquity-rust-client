use std::io;

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

#[tokio::test]
async fn sync_block_number() {
  match setup() {
    Ok(setup_data) => {
      let url = setup_data.url;
      let _m = setup_data.mocks;

      let config = configuration::Configuration {
        base_path: url,
        ..configuration::Configuration::new()
      };

      let test_platform_pairs = [("bitcoin", "mainnet"), ("ethereum", "mainnet")];

      for (platform, network) in test_platform_pairs {
        let res = sync_api::current_block_number(&config, platform, network).await;
        match res {
          Ok(_) => {}
          Err(e) => panic!("{}", e),
        };
      }
    }
    Err(e) => panic!("{}", e),
  }
}

#[tokio::test]
async fn sync_block_id() {
  match setup() {
    Ok(setup_data) => {
      let url = setup_data.url;
      let _m = setup_data.mocks;

      let config = configuration::Configuration {
        base_path: url,
        ..configuration::Configuration::new()
      };
      let platform = "bitcoin";
      let network = "mainnet";

      let res = sync_api::current_block_id(&config, platform, network).await;
      match res {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
      };
    }
    Err(e) => panic!("{}", e),
  }
}
