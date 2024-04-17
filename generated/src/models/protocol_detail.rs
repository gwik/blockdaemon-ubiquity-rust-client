/*
 * Universal REST API
 *
 * Universal API provides a RESTful and uniform way to access blockchain resources, with a rich and reusable model across multiple protocols/cryptocurrencies.  [Documentation](https://app.blockdaemon.com/docs/ubiquity)  ### Currently supported protocols:  * algorand   * mainnet * bitcoin   * mainnet/testnet * bitcoincash   * mainnet/testnet * dogecoin   * mainnet/testnet * ethereum   * mainnet/goerli * litecoin   * mainnet/testnet * near   * mainnet/testnet * oasis   * mainnet * optimism   * mainnet * polkadot   * mainnet/westend * polygon   * mainnet/amoy * solana   * mainnet/testnet * stellar   * mainnet/testnet * tezos   * mainnet * xrp   * mainnet  ##### Pagination Certain resources contain a lot of data, more than what's practical to return for a single request. With the help of pagination, the data is split across multiple responses. Each response returns a subset of the items requested, and a continuation token.  To get the next batch of items, copy the returned continuation token to the continuation query parameter and repeat the request with the new URL. In case no continuation token is returned, there is no more data available. 
 *
 * The version of the OpenAPI document: 3.0.0
 * Contact: support@blockdaemon.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProtocolDetail {
    /// Backend API Type
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "handle", skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    #[serde(rename = "genesis_number", skip_serializing_if = "Option::is_none")]
    pub genesis_number: Option<i64>,
    #[serde(rename = "endpoints", skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<String>>,
}

impl ProtocolDetail {
    pub fn new() -> ProtocolDetail {
        ProtocolDetail {
            source: None,
            handle: None,
            genesis_number: None,
            endpoints: None,
        }
    }
}


