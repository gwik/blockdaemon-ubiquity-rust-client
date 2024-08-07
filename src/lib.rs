pub use ubiquity_openapi_client::apis::configuration;
pub use ubiquity_openapi_client::models;

pub use ubiquity_openapi_client;

/// Example of getting latest block's information
/// ```no_run
/// use ubiquity::api::blocks_api;
/// use ubiquity::configuration;
/// use ubiquity::models::block;
///
/// async fn get_block(token: String) -> Option<()>{
///     let conf = configuration::Configuration {
///         bearer_access_token: Some(token),
///         ..configuration::Configuration::new()
///     };
///
///     let protocol = "ethereum";
///     let network = "mainnet";
///     let ident = "current";
///     let option = "";
///
///     let block_result = blocks_api::get_block_by_number(&conf, protocol, network, ident).await.ok()?;
///
///     let block_id = block_result.id.as_ref()?;
///     let block_txs = block_result.txs.as_ref()?;
///
///     Some(println!(
///         "Latest block has hash {} and {} transactions",
///         block_id,
///         block_txs.len()
///     ))
///
/// }
/// ```
///
/// Example of getting paginated transactions
/// ```no_run
/// use transactions_api::GetTxsError;
/// use ubiquity::api::transactions_api;
/// use ubiquity::configuration;
/// use ubiquity::models::TxPage;
/// use ubiquity::ubiquity_openapi_client::apis::Error;
///
/// fn format_get_tx_error(err: Error<GetTxsError>) -> String {
///     return format!("{}", err).to_string();
/// }
///
/// async fn get_continuated_txs(
///     conf: &configuration::Configuration,
///     protocol: &str,
///     network: &str,
///     order: Option<&str>,
///     limit: Option<i32>,
///     tx_page: &TxPage,
/// ) -> Result<TxPage, String> {
///     return transactions_api::get_txs(
///         conf,
///         protocol,
///         network,
///         None,
///         None,
///         None,
///         order,
///         None,
///         limit,
///     )
///     .await
///     .map_err(format_get_tx_error);
/// }
///
/// fn print_tx_page_ids(tx_page: &TxPage) -> Result<(), String> {
///     let items = tx_page.data.as_ref().ok_or("Could not get items!")?;
///     return Ok(println!(
///         "Transaction IDs: {:#?}",
///         items.iter().map(|tx| tx.id.as_ref()).collect::<Vec<_>>()
///     ));
/// }
///
/// async fn print_and_get_continuated_txs(
///     conf: &configuration::Configuration,
///     protocol: &str,
///     network: &str,
///     order: Option<&str>,
///     limit: Option<i32>,
///     tx_page: &TxPage,
/// ) -> Result<TxPage, String> {
///     print_tx_page_ids(tx_page)?;
///     return get_continuated_txs(&conf, protocol, network, order, limit, &tx_page).await;
/// }
///
/// async fn get_paginated_txs(token: String) -> Result<(), String> {
///     let conf = configuration::Configuration {
///         bearer_access_token: Some(token),
///         ..configuration::Configuration::new()
///     };
///
///     let protocol = "ethereum";
///     let network = "mainnet";
///
///     let order = Some("desc");
///     let continuation = None;
///     let limit = Some(10);
///
///     // get last 10 transactions
///     let tx_result =
///         transactions_api::get_txs(&conf, protocol, network, None, None, None, order, continuation, limit).await.map_err(format_get_tx_error)?;
///
///     let tx_page = print_and_get_continuated_txs(
///         &conf, protocol, network, order, limit, &tx_result,
///     ).await?;
///
///     print_tx_page_ids(&tx_page)
///
/// }
/// ```
pub mod api;
