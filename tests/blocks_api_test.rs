use std::{future::Future, io};

use ubiquity::{api::blocks_api, configuration};

pub mod utils;

fn setup_blocks_by_identifier(
    test_blocks_data: &[(&str, &str, &str)],
) -> Result<utils::Setup, io::Error> {
    let url = mockito::server_url();

    let mut mocks = vec![];
    for (platform, network, id) in test_blocks_data {
        let mock = utils::create_mock_from_file(
            &format!("./tests/mock_files/blocks_api/{}.json", platform),
            &format!("/{}/{}/block/{}", platform, network, id),
        )?;
        mocks.push(mock);
    }

    Ok(utils::new_setup(url, mocks))
}

fn setup_sync() -> Result<utils::Setup, io::Error> {
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

    Ok(utils::new_setup(
        url,
        vec![
            block_number_bitcoin_mock,
            block_number_ethereum_mock,
            block_id_bitcoin_mock,
            block_id_ethereum_mock,
        ],
    ))
}

fn setup_block_ids_by_identifier(
    test_blocks_data: &[(&str, &str, &str)],
) -> Result<utils::Setup, io::Error> {
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

    Ok(utils::new_setup(url, mocks))
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

            for (platform, network, ident) in test_blocks_data {
                let res =
                    blocks_api::get_block_by_number(&setup_data.config, platform, network, ident)
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
async fn blocks_by_number() {
    let test_blocks_data = vec![
        ("bitcoin", "mainnet", "686702"),
        ("ethereum", "mainnet", "12589661"),
    ];

    match setup_blocks_by_identifier(&test_blocks_data) {
        Ok(setup_data) => {
            let _m = setup_data.mocks;

            for (platform, network, ident) in test_blocks_data {
                let res = blocks_api::get_block_by_number(&setup_data.config, platform, network, ident).await;
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

            for (platform, network, ident) in test_blocks_data {
                let res = blocks_api::get_block_identifier_by_number(
                    &setup_data.config,
                    platform,
                    network,
                    ident,
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
async fn block_ids_by_number() {
    let test_blocks_data = vec![
        ("bitcoin", "mainnet", "686702"),
        ("ethereum", "mainnet", "12589661"),
    ];

    match setup_block_ids_by_identifier(&test_blocks_data) {
        Ok(setup_data) => {
            let _m = setup_data.mocks;

            for (platform, network, ident) in test_blocks_data {
                let res = blocks_api::get_block_identifier_by_number(
                    &setup_data.config,
                    platform,
                    network,
                    ident,
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

async fn test_current_block_id_with_config(config: configuration::Configuration) {
    let test_platform_pairs = [("bitcoin", "mainnet"), ("ethereum", "mainnet")];

    for (platform, network) in test_platform_pairs {
        let res = blocks_api::get_current_block_hash(&config, platform, network).await;
        match res {
            Ok(_) => {}
            Err(e) => panic!("{}", e),
        };
    }
}

async fn test_current_block_number_with_config(config: configuration::Configuration) {
    let test_platform_pairs = [("bitcoin", "mainnet"), ("ethereum", "mainnet")];

    for (platform, network) in test_platform_pairs {
        let res = blocks_api::get_current_block_number(&config, platform, network).await;
        match res {
            Ok(_) => {}
            Err(e) => panic!("{}", e),
        };
    }
}
async fn sync_test<F, Fut>(func: F)
where
    F: Fn(configuration::Configuration) -> Fut,
    Fut: Future<Output = ()>,
{
    match setup_sync() {
        Ok(setup_data) => {
            let _m = setup_data.mocks;

            func(setup_data.config).await;
        }
        Err(e) => panic!("{}", e),
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
