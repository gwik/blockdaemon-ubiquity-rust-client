/*
 * Blockdaemon REST API
 *
 * Blockdaemon REST API provides a RESTful and uniform way to access blockchain resources, with a rich and reusable model across multiple protocols/cryptocurrencies.  [Documentation](https://docs.blockdaemon.com/reference/rest-api-overview)  ### Currently supported protocols:  * algorand   * mainnet * avalanche    * mainnet-c/testnet-c * bitcoin   * mainnet/testnet * bitcoincash   * mainnet/testnet * dogecoin   * mainnet/testnet * ethereum   * mainnet/holesky/sepolia * fantom   * mainnet/testnet * litecoin   * mainnet/testnet * near   * mainnet * optimism   * mainnet * polkadot   * mainnet/westend * polygon   * mainnet/amoy * solana   * mainnet/testnet * stellar   * mainnet/testnet * tezos   * mainnet * tron   * mainnet/nile * xrp   * mainnet  ### Pagination Certain resources contain a lot of data, more than what's practical to return for a single request. With the help of pagination, the data is split across multiple responses. Each response returns a subset of the items requested, and a continuation token.  To get the next batch of items, copy the returned continuation token to the continuation query parameter and repeat the request with the new URL. In case no continuation token is returned, there is no more data available. 
 *
 * The version of the OpenAPI document: 3.0.0
 * Contact: support@blockdaemon.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TxOutput {
    /// The result status of the transaction output.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// If the transaction output was spent or not, if the value is true the `spent` transaction object will be presented.
    #[serde(rename = "is_spent", skip_serializing_if = "Option::is_none")]
    pub is_spent: Option<bool>,
    #[serde(rename = "spent", skip_serializing_if = "Option::is_none")]
    pub spent: Option<Box<crate::models::TxMinify>>,
    /// The amount of transaction output.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    #[serde(rename = "mined", skip_serializing_if = "Option::is_none")]
    pub mined: Option<Box<crate::models::TxMinify>>,
}

impl TxOutput {
    pub fn new() -> TxOutput {
        TxOutput {
            status: None,
            is_spent: None,
            spent: None,
            value: None,
            mined: None,
        }
    }
}

/// The result status of the transaction output.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "mined")]
    Mined,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for Status {
    fn default() -> Status {
        Self::Mined
    }
}

