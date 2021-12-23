use ubiquity::api::platforms_api;

pub mod utils;

#[tokio::test]
async fn test_get_platform() {
  let url = mockito::server_url();

  let get_platform_algorand_mock = (
    "algorand",
    "testnet",
    utils::create_mock_from_file(
      "./tests/mock_files/platforms_api/platform_algorand_testnet.json",
      "/v2/algorand/testnet",
    ),
  );

  let get_platform_polkadot_mock = (
    "polkadot",
    "mainnet",
    utils::create_mock_from_file(
      "./tests/mock_files/platforms_api/platform_polkadot_mainnet.json",
      "/v2/polkadot/mainnet",
    ),
  );

  let config = utils::config_from_url(url);

  let mocks = vec![get_platform_algorand_mock, get_platform_polkadot_mock];

  for (platform, network, mock_result) in mocks {
    match mock_result {
      Ok(_) => {
        let res = platforms_api::get_platform(&config, platform, network).await;
        match res {
          Ok(_) => {}
          Err(e) => panic!("{}", e),
        };
      }
      Err(e) => panic!("{}", e),
    }
  }
}

#[tokio::test]
async fn test_get_platforms_overview() {
  let url = mockito::server_url();

  let get_platforms_overview_mock = utils::create_mock_from_file(
    "./tests/mock_files/platforms_api/platforms_overview.json",
    &format!("/v2/"),
  );

  let config = utils::config_from_url(url);

  match get_platforms_overview_mock {
    Ok(_) => {
      let res = platforms_api::get_platforms(&config).await;
      match res {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
      };
    }
    Err(e) => panic!("{}", e),
  }
}
