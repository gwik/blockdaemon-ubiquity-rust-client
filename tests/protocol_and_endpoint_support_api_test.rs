use ubiquity::api::protocol_and_endpoint_support_api as protocols_api;

pub mod utils;

#[tokio::test]
async fn test_get_protocol() {
  let url = mockito::server_url();

  let get_protocol_algorand_mock = (
    "algorand",
    "mainnet",
    utils::create_mock_from_file(
      "./tests/mock_files/protocol_and_endpoint_support_api/protocol_algorand_mainnet.json",
      "/algorand/mainnet",
    ),
  );

  let get_protocol_polkadot_mock = (
    "polkadot",
    "mainnet",
    utils::create_mock_from_file(
      "./tests/mock_files/protocol_and_endpoint_support_api/protocol_polkadot_mainnet.json",
      "/polkadot/mainnet",
    ),
  );

  let config = utils::config_from_url(url);

  let mocks = vec![get_protocol_algorand_mock, get_protocol_polkadot_mock];  

  for (protocol, network, mock_result) in mocks {
    match mock_result {
      Ok(_) => {
        let res = protocols_api::get_protocol_endpoints(&config, protocol, network).await;
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
async fn test_get_protocols_overview() {
  let url = mockito::server_url();

  let get_protocols_overview_mock = utils::create_mock_from_file(
    "./tests/mock_files/protocol_and_endpoint_support_api/protocols_overview.json",
    &format!("/"),
  );

  let config = utils::config_from_url(url);

  match get_protocols_overview_mock {
    Ok(_) => {
      let res = protocols_api::get_protocols_list(&config).await;
      match res {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
      };
    }
    Err(e) => panic!("{}", e),
  }
}
