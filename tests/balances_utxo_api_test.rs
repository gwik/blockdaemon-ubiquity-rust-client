use std::io;

use ubiquity::api::balances_utxo_api;
use ubiquity::api::transactions_api;

pub mod utils;

fn setup_balances_endpoints(
    test_accounts_data: &[(&str, &str, &str)],
) -> Result<utils::Setup, io::Error> {
    let url = mockito::server_url();

    let mut mocks = vec![];
    for (protocol, network, address) in test_accounts_data {
        let filepath = &format!(
            "./tests/mock_files/balances_utxo_api/{}_account_{}.json",
            protocol, address
        );

        let mock = utils::create_mock_from_file(
            &filepath,
            &format!("/{}/{}/account/{}", protocol, network, address),
        )?;
        mocks.push(mock);
    }

    Ok(utils::new_setup(url, mocks))
}

fn setup_txs_endpoints(
    test_accounts_data: &[(&str, &str, &str)],
) -> Result<utils::Setup, io::Error> {
    let url = mockito::server_url();

    let mut mocks = vec![];
    for (protocol, _network, address) in test_accounts_data {
        let mock = utils::create_mock_from_file(
            &format!(
                "./tests/mock_files/balances_utxo_api/{}_account_{}_txs.json",
                protocol, address
            ),
            &format!("/{}/{}/account/{}/txs", protocol, _network, address),
        )?;
        mocks.push(mock);
    }

    Ok(utils::new_setup(url, mocks))
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

            for (protocol, network, address) in test_account_balances_data {
                let res = balances_utxo_api::get_list_of_balances_by_address(
                    &setup_data.config,
                    protocol,
                    network,
                    address,
                    None,
                )
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

            for (protocol, network, address) in test_account_txs_data {
                let res = transactions_api::get_txs_by_address(
                    &setup_data.config,
                    protocol,
                    network,
                    address,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                )
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
