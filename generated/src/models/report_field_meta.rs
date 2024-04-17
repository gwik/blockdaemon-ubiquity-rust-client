/*
 * Universal REST API
 *
 * Universal API provides a RESTful and uniform way to access blockchain resources, with a rich and reusable model across multiple protocols/cryptocurrencies.  [Documentation](https://app.blockdaemon.com/docs/ubiquity)  ### Currently supported protocols:  * algorand   * mainnet * bitcoin   * mainnet/testnet * bitcoincash   * mainnet/testnet * dogecoin   * mainnet/testnet * ethereum   * mainnet/goerli * litecoin   * mainnet/testnet * near   * mainnet/testnet * oasis   * mainnet * optimism   * mainnet * polkadot   * mainnet/westend * polygon   * mainnet/amoy * solana   * mainnet/testnet * stellar   * mainnet/testnet * tezos   * mainnet * xrp   * mainnet  ##### Pagination Certain resources contain a lot of data, more than what's practical to return for a single request. With the help of pagination, the data is split across multiple responses. Each response returns a subset of the items requested, and a continuation token.  To get the next batch of items, copy the returned continuation token to the continuation query parameter and repeat the request with the new URL. In case no continuation token is returned, there is no more data available. 
 *
 * The version of the OpenAPI document: 3.0.0
 * Contact: support@blockdaemon.com
 * Generated by: https://openapi-generator.tech
 */

/// ReportFieldMeta : Additional metadata bespoke to specific protocols


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ReportFieldMeta {
    #[serde(rename="algorand_meta")]
    AlgorandMeta {
        #[serde(rename = "sender_reward", skip_serializing_if = "Option::is_none")]
        sender_reward: Option<String>,
        #[serde(rename = "recipient_reward", skip_serializing_if = "Option::is_none")]
        recipient_reward: Option<String>,
        #[serde(rename = "close", skip_serializing_if = "Option::is_none")]
        close: Option<String>,
        #[serde(rename = "close_amount", skip_serializing_if = "Option::is_none")]
        close_amount: Option<String>,
        #[serde(rename = "close_reward", skip_serializing_if = "Option::is_none")]
        close_reward: Option<String>,
    },
}




