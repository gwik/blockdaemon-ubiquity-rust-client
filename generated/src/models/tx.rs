/*
 * Ubiquity REST API
 *
 * Ubiquity provides a RESTful and uniform way to access blockchain resources, with a rich and reusable model across multiple cryptocurrencies.  [Documentation](https://app.blockdaemon.com/docs/ubiquity)  ### Protocols #### Mainnet The following protocols are currently supported: * bitcoin * ethereum * polkadot * xrp * algorand * stellar * dogecoin  #### Testnet * bitcoin/testnet * ethereum/ropsten * dogecoin/testnet  ##### Pagination Certain resources contain a lot of data, more than what's practical to return for a single request. With the help of pagination, the data is split across multiple responses. Each response returns a subset of the items requested and a continuation token.  To get the next batch of items, copy the returned continuation token to the continuation query parameter and repeat the request with the new URL. In case no continuation token is returned, there is no more data available. 
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@blockdaemon.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tx {
    /// Unique transaction identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// SLIP-44 coin ID
    #[serde(rename = "slip44", skip_serializing_if = "Option::is_none")]
    pub slip44: Option<i32>,
    /// List of involved addresses (excluding contracts)
    #[serde(rename = "addresses", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    /// List of moved assets by asset path
    #[serde(rename = "assets", skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<String>>,
    /// Unix timestamp
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<i64>,
    /// Number of block if mined, otherwise omitted.
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// ID of block if mined, otherwise omitted.
    #[serde(rename = "block_id", skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    /// Result status of the transaction.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// List of tags for this transaction
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Operations in this transaction (opaque keys).
    #[serde(rename = "operations", skip_serializing_if = "Option::is_none")]
    pub operations: Option<::std::collections::HashMap<String, crate::models::Operation>>,
    /// Effects by address, if supported
    #[serde(rename = "effects", skip_serializing_if = "Option::is_none")]
    pub effects: Option<::std::collections::HashMap<String, crate::models::Effect>>,
}

impl Tx {
    pub fn new() -> Tx {
        Tx {
            id: None,
            slip44: None,
            addresses: None,
            assets: None,
            date: None,
            height: None,
            block_id: None,
            status: None,
            tags: None,
            operations: None,
            effects: None,
        }
    }
}

/// Result status of the transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}

