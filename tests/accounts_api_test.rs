use std::io;

use ubiquity::api::accounts_api;

pub mod utils;

struct Setup {
  url: String,
  mocks: Vec<mockito::Mock>,
}

fn setup_balances_endpoints(test_accounts_data: &[(&str, &str, &str)]) -> Result<Setup, io::Error> {
  let url = mockito::server_url();

  let mut mocks = vec![];
  for (platform, _network, address) in test_accounts_data {
    let mock = utils::create_mock_from_file(
      &format!(
        "./tests/mock_files/accounts_api/{}_account_{}.json",
        platform, address
      ),
      &format!("/{}/mainnet/account/{}", platform, address),
    )?;
    mocks.push(mock);
  }
  
  Ok(Setup {
    url,
    mocks,
  })
}

fn setup_txs_endpoints(test_accounts_data: &[(&str, &str, &str)]) -> Result<Setup, io::Error> {
  let url = mockito::server_url();

  let mut mocks = vec![];
  for (platform, _network, address) in test_accounts_data {
    let mock = utils::create_mock_from_file(
      &format!(
        "./tests/mock_files/accounts_api/{}_account_{}_txs.json",
        platform, address
      ),
      &format!("/{}/mainnet/account/{}/txs", platform, address),
    )?;
    mocks.push(mock);
  }
  
  Ok(Setup {
    url,
    mocks,
  })

}

#[tokio::test]
async fn account_balances() {
  let test_account_balances_data = vec![
    (
      "algorand",
      "mainnet",
      "5K6J3Z54656IR7YY65WNJT54UW6RBZZYL5LWQUTG4RWOTRTRBE2MR2AODQ",
    ),
    ("xrp", "mainnet", "rh3VLyj1GbQjX7eA15BwUagEhSrPHmLkSR"),
  ];

  match setup_balances_endpoints(&test_account_balances_data) {
    Ok(setup_data) => {
      let _m = setup_data.mocks;

      let config = utils::config_from_url(setup_data.url);

      for (platform, network, address) in test_account_balances_data {
        let res = accounts_api::get_balances_by_address(&config, platform, network, address, None).await;
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
async fn account_txs() {
  let test_account_txs_data = vec![
    (
      "bitcoin",
      "mainnet",
      "bc1q0e3vshzcp8kntj2ynqaxt32sqshqf36tl8xdmz",
    ),
    (
      "ethereum",
      "mainnet",
      "0x30B2f83e291f952850DFB13380F6c71b24FDF97c",
    ),
  ];


  match setup_txs_endpoints(&test_account_txs_data) {
    Ok(setup_data) => {
      let _m = setup_data.mocks;

      let config = utils::config_from_url(setup_data.url);

      for (platform, network, address) in test_account_txs_data {
        let res =
          accounts_api::get_txs_by_address(&config, platform, network, address, None, None, None, None)
            .await;
        match res {
          Ok(_) => {}
          Err(e) => panic!("{}", e),
        };
      }
    }
    Err(e) => panic!("{}", e),
  }
}
