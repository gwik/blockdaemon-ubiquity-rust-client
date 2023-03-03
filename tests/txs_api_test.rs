use std::io;

use ubiquity::api::transactions_api;

pub mod utils;

fn setup_txs_by_id(test_txs_data: &[(&str, &str, &str)]) -> Result<utils::Setup, io::Error> {
  let url = mockito::server_url();

  let mut mocks = vec![];
  for (platform, network, id) in test_txs_data {
    let mock = utils::create_mock_from_file(
      &format!(
        "./tests/mock_files/transactions_api/{}_{}_{}.json",
        platform, network, id
      ),
      &format!("/{}/{}/tx/{}", platform, network, id),
    )?;
    mocks.push(mock);
  }

  Ok(utils::new_setup(url, mocks))
}

fn setup_txs(test_txs_data: &[(&str, &str)]) -> Result<utils::Setup, io::Error> {
  let url = mockito::server_url();

  let mut mocks = vec![];
  for (platform, network) in test_txs_data {
    let mock = utils::create_mock_from_file(
      &format!(
        "./tests/mock_files/transactions_api/get_txs_{}.json",
        platform
      ),
      &format!("/{}/{}/txs", platform, network),
    )?;
    mocks.push(mock);
  }

  Ok(utils::new_setup(url, mocks ))
}

#[tokio::test]
async fn tx_by_id() {
  let test_txs_data = vec![
    (
      "bitcoin",
      "mainnet",
      "d020dcbae21454aa6c042083cc21c35359f5d14a06162e29ac6ec82b9f81b5db",
    ),
    (
      "ethereum",
      "mainnet",
      "0xaadb85be12adcbfbe2ff84a1976d7de1d5fda1364ccf2b3cfa8ef046d73b4846",
    ),
  ];

  match setup_txs_by_id(&test_txs_data) {
    Ok(setup_data) => {
      let _m = setup_data.mocks;

      for (platform, network, id) in test_txs_data {
        let res = transactions_api::get_tx(&setup_data.config, platform, network, id).await;
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
async fn tx_by_id_with_large_amount() {
  let test_txs_data = vec![
    (
      "ethereum",
      "goerli",
      "0x9681659ba69b31a4ba55bb07b70ff7464cf065b43b60afd97e15b26a024a8e74",
    ),
  ];

  match setup_txs_by_id(&test_txs_data) {
    Ok(setup_data) => {
      let _m = setup_data.mocks;

      for (platform, network, id) in test_txs_data {
        let res = transactions_api::get_tx(&setup_data.config, platform, network, id).await;
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
async fn txs() {
  let test_txs_data = vec![("bitcoin", "mainnet"), ("ethereum", "mainnet")];

  match setup_txs(&test_txs_data) {
    Ok(setup_data) => {
      let _m = setup_data.mocks;

      for (platform, network) in test_txs_data {
        let res =
          transactions_api::get_txs(&setup_data.config, platform, network, None, None, None, None, None).await;
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
async fn estimate_fee() {
  let url = mockito::server_url();
  let config = utils::config_from_url(url);

  let _mock = utils::create_mock_from_file(
    "./tests/mock_files/transactions_api/estimate_fee.json",
    "/bitcoin/mainnet/tx/estimate_fee",
  );

  let res = transactions_api::fee_estimate(&config, "bitcoin", "mainnet").await;
  match res {
    Ok(_) => {}
    Err(e) => panic!("{}", e),
  };
}
