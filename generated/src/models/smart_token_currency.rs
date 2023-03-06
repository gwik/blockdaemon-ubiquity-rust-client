/*
 * Universal REST API
 *
 * Universal API provides a RESTful and uniform way to access blockchain resources, with a rich and reusable model across multiple protocols/cryptocurrencies.  [Documentation](https://app.blockdaemon.com/docs/ubiquity)  ### Currently supported protocols:  * algorand   * mainnet * bitcoin   * mainnet/testnet * bitcoincash   * mainnet/testnet * dogecoin   * mainnet/testnet * ethereum   * mainnet/goerli * litecoin   * mainnet/testnet * near   * mainnet/testnet * oasis   * mainnet * optimism   * mainnet * polkadot   * mainnet/westend * polygon   * mainnet * solana   * mainnet/testnet * stellar   * mainnet/testnet * tezos   * mainnet * xrp   * mainnet  ##### Pagination Certain resources contain a lot of data, more than what's practical to return for a single request. With the help of pagination, the data is split across multiple responses. Each response returns a subset of the items requested, and a continuation token.  To get the next batch of items, copy the returned continuation token to the continuation query parameter and repeat the request with the new URL. In case no continuation token is returned, there is no more data available. 
 *
 * The version of the OpenAPI document: 3.0.0
 * Contact: support@blockdaemon.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SmartTokenCurrency {
    /// Asset path of transferred currency
    #[serde(rename = "asset_path")]
    pub asset_path: String,
    /// Currency symbol
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Name of currency
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Decimal places right to the comma
    #[serde(rename = "decimals", skip_serializing_if = "Option::is_none")]
    pub decimals: Option<i32>,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<Box<crate::models::SmartToken>>,
}

impl SmartTokenCurrency {
    pub fn new(asset_path: String, _type: String) -> SmartTokenCurrency {
        SmartTokenCurrency {
            asset_path,
            symbol: None,
            name: None,
            decimals: None,
            _type,
            detail: None,
        }
    }
}


