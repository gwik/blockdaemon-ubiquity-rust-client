use std::io;

use ubiquity::api::blocks_api;

pub mod utils;

struct Setup {
  url: String,
  mocks: Vec<mockito::Mock>,
}

fn setup_blocks_by_identifier(test_blocks_data: &[(&str, &str, &str)]) -> Result<Setup, io::Error> {
  let url = mockito::server_url();

  let mut mocks = vec![];
  for (platform, network, id) in test_blocks_data {
    let mock = utils::create_mock_from_file(
      &format!(
        "./tests/mock_files/blocks_api/{}.json",
        platform
      ),
      &format!("/{}/{}/block/{}", platform, network, id),
    )?;
    mocks.push(mock);
  }
  
  Ok(Setup {
    url,
    mocks,
  })
}

fn setup_block_ids_by_identifier(test_blocks_data: &[(&str, &str, &str)]) -> Result<Setup, io::Error> {
  let url = mockito::server_url();

  let mut mocks = vec![];
  for (platform, network, id) in test_blocks_data {
    let mock = utils::create_mock_from_file(
      &format!(
        "./tests/mock_files/blocks_api/block_identifiers/{}.json",
        platform
      ),
      &format!("/{}/{}/block_identifier/{}", platform, network, id),
    )?;
    mocks.push(mock);
  }
  
  Ok(Setup {
    url,
    mocks,
  })
}

#[tokio::test]
async fn blocks_by_id() {
  let test_blocks_data = vec![
    (
      "bitcoin",
      "mainnet",
      "0000000000000000000bfdd964f1771093434027c9d68f60a74dac6fb80ac705",
    ),
    (
      "ethereum",
      "mainnet",
      "0xed9e9e89c3d48f5a7bd749f5c20327eaf286b54ede82da001d49e6cf50d8c372",
    ),
  ];

  match setup_blocks_by_identifier(&test_blocks_data) {
    Ok(setup_data) => {
      let _m = setup_data.mocks;

      let config = utils::config_from_url(setup_data.url);

      for (platform, network, ident) in test_blocks_data {
        let res = blocks_api::get_block(&config, platform, network, ident).await;
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
async fn blocks_by_number() {
  let test_blocks_data = vec![
    (
      "bitcoin",
      "mainnet",
      "686702",
    ),
    (
      "ethereum",
      "mainnet",
      "12589661",
    ),
  ];

  match setup_blocks_by_identifier(&test_blocks_data) {
    Ok(setup_data) => {
      let _m = setup_data.mocks;

      let config = utils::config_from_url(setup_data.url);

      for (platform, network, ident) in test_blocks_data {
        let res = blocks_api::get_block(&config, platform, network, ident).await;
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
async fn block_ids_by_id() {
  let test_blocks_data = vec![
    (
      "bitcoin",
      "mainnet",
      "0000000000000000000bfdd964f1771093434027c9d68f60a74dac6fb80ac705",
    ),
    (
      "ethereum",
      "mainnet",
      "0xed9e9e89c3d48f5a7bd749f5c20327eaf286b54ede82da001d49e6cf50d8c372",
    ),
  ];

  match setup_block_ids_by_identifier(&test_blocks_data) {
    Ok(setup_data) => {
      let _m = setup_data.mocks;

      let config = utils::config_from_url(setup_data.url);

      for (platform, network, ident) in test_blocks_data {
        let res = blocks_api::get_block_identifier(&config, platform, network, ident).await;
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
async fn block_ids_by_number() {
  let test_blocks_data = vec![
    (
      "bitcoin",
      "mainnet",
      "686702",
    ),
    (
      "ethereum",
      "mainnet",
      "12589661",
    ),
  ];

  match setup_block_ids_by_identifier(&test_blocks_data) {
    Ok(setup_data) => {
      let _m = setup_data.mocks;

      let config = utils::config_from_url(setup_data.url);

      for (platform, network, ident) in test_blocks_data {
        let res = blocks_api::get_block_identifier(&config, platform, network, ident).await;
        match res {
          Ok(_) => {}
          Err(e) => panic!("{}", e),
        };
      }

    }
    Err(e) => panic!("{}", e),
  }
}
