/*
 * Blockdaemon REST API
 *
 * Blockdaemon REST API provides a RESTful and uniform way to access blockchain resources, with a rich and reusable model across multiple protocols/cryptocurrencies.  [Documentation](https://docs.blockdaemon.com/reference/rest-api-overview)  ### Currently supported protocols:  * algorand   * mainnet * avalanche    * mainnet-c/testnet-c * bitcoin   * mainnet/testnet * bitcoincash   * mainnet/testnet * dogecoin   * mainnet/testnet * ethereum   * mainnet/holesky/sepolia * fantom   * mainnet/testnet * litecoin   * mainnet/testnet * near   * mainnet * optimism   * mainnet * polkadot   * mainnet/westend * polygon   * mainnet/amoy * solana   * mainnet/testnet * stellar   * mainnet/testnet * tezos   * mainnet * tron   * mainnet/nile * xrp   * mainnet  ### Pagination Certain resources contain a lot of data, more than what's practical to return for a single request. With the help of pagination, the data is split across multiple responses. Each response returns a subset of the items requested, and a continuation token.  To get the next batch of items, copy the returned continuation token to the continuation query parameter and repeat the request with the new URL. In case no continuation token is returned, there is no more data available. 
 *
 * The version of the OpenAPI document: 3.0.0
 * Contact: support@blockdaemon.com
 * Generated by: https://openapi-generator.tech
 */

/// Meta : The meta details.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Meta {
    #[serde(rename = "paging", skip_serializing_if = "Option::is_none")]
    pub paging: Option<Box<crate::models::Paging>>,
}

impl Meta {
    /// The meta details.
    pub fn new() -> Meta {
        Meta {
            paging: None,
        }
    }
}


