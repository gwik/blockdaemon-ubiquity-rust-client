use transactions_api::GetTxsError;
use ubiquity::api::transactions_api;
use ubiquity::configuration;
use ubiquity::models::TxPage;
use ubiquity::ubiquity_openapi_client::apis::Error;

use std::env;

fn format_get_tx_error(err: Error<GetTxsError>) -> String {
    return format!("{}", err).to_string();
}

async fn get_continuated_txs(
    conf: &configuration::Configuration,
    platform: &str,
    network: &str,
    order: Option<&str>,
    limit: Option<i32>,
    assets: Option<&str>,
    tx_page: &TxPage,
) -> Result<TxPage, String> {
    return transactions_api::get_txs(
        conf,
        platform,
        network,
        order,
        tx_page.continuation.as_deref(),
        limit,
        assets,
    )
    .await
    .map_err(format_get_tx_error);
}

fn print_tx_page_ids(tx_page: &TxPage) -> Result<(), String> {
    let items = tx_page.items.as_ref().ok_or("Could not get items!")?;
    return Ok(println!(
        "Transaction IDs: {:#?}",
        items.iter().map(|tx| tx.id.as_ref()).collect::<Vec<_>>()
    ));
}

async fn print_and_get_continuated_txs(
    conf: &configuration::Configuration,
    platform: &str,
    network: &str,
    order: Option<&str>,
    limit: Option<i32>,
    assets: Option<&str>,
    tx_page: &TxPage,
) -> Result<TxPage, String> {
    print_tx_page_ids(tx_page)?;
    return get_continuated_txs(&conf, platform, network, order, limit, assets, &tx_page).await;
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

    let order = Some("desc");
    let continuation = None;
    let limit = Some(10);
    let assets = None;

    // get last 10 transactions
    let tx_result =
        transactions_api::get_txs(&conf, platform, network, order, continuation, limit, assets)
            .await;

    match tx_result {
        Ok(tx_page) => {
            // gets next 10 transactions from continuation value
            match print_and_get_continuated_txs(
                &conf, platform, network, order, limit, assets, &tx_page,
            )
            .await
            .and_then(|tx_page| print_tx_page_ids(&tx_page))
            {
                Ok(_tx_page) => {}
                Err(e) => panic!("{}", e),
            }
        }
        Err(e) => panic!("{}", e),
    };
}
