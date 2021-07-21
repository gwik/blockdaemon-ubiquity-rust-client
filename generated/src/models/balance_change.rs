/*
 * Ubiquity REST API
 *
 * Ubiquity provides a RESTful and uniform way to access blockchain resources, with a rich and reusable model across multiple cryptocurrencies.  [Documentation](https://app.blockdaemon.com/docs/ubiquity)  ### Protocols #### Mainnet The following protocols are currently supported: * bitcoin * ethereum * polkadot * xrp * algorand * stellar * dogecoin  #### Testnet * bitcoin/testnet * ethereum/ropsten * dogecoin/testnet  ##### Pagination Certain resources contain a lot of data, more than what's practical to return for a single request. With the help of pagination, the data is split across multiple responses. Each response returns a subset of the items requested and a continuation token.  To get the next batch of items, copy the returned continuation token to the continuation query parameter and repeat the request with the new URL. In case no continuation token is returned, there is no more data available. 
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@blockdaemon.com
 * Generated by: https://openapi-generator.tech
 */

/// BalanceChange : Change of balance of a currency



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BalanceChange {
    /// Balance before transaction
    #[serde(rename = "old", skip_serializing_if = "Option::is_none")]
    pub old: Option<String>,
    /// Balance difference
    #[serde(rename = "delta", skip_serializing_if = "Option::is_none")]
    pub delta: Option<String>,
    /// Balance after transaction
    #[serde(rename = "new", skip_serializing_if = "Option::is_none")]
    pub new: Option<String>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<Box<crate::models::Currency>>,
}

impl BalanceChange {
    /// Change of balance of a currency
    pub fn new() -> BalanceChange {
        BalanceChange {
            old: None,
            delta: None,
            new: None,
            currency: None,
        }
    }
}


