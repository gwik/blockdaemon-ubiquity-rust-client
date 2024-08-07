/*
 * Blockdaemon REST API
 *
 * Blockdaemon REST API provides a RESTful and uniform way to access blockchain resources, with a rich and reusable model across multiple protocols/cryptocurrencies.  [Documentation](https://docs.blockdaemon.com/reference/rest-api-overview)  ### Currently supported protocols:  * algorand   * mainnet * avalanche    * mainnet-c/testnet-c * bitcoin   * mainnet/testnet * bitcoincash   * mainnet/testnet * dogecoin   * mainnet/testnet * ethereum   * mainnet/holesky/sepolia * fantom   * mainnet/testnet * litecoin   * mainnet/testnet * near   * mainnet * optimism   * mainnet * polkadot   * mainnet/westend * polygon   * mainnet/amoy * solana   * mainnet/testnet * stellar   * mainnet/testnet * tezos   * mainnet * tron   * mainnet/nile * xrp   * mainnet  ### Pagination Certain resources contain a lot of data, more than what's practical to return for a single request. With the help of pagination, the data is split across multiple responses. Each response returns a subset of the items requested, and a continuation token.  To get the next batch of items, copy the returned continuation token to the continuation query parameter and repeat the request with the new URL. In case no continuation token is returned, there is no more data available. 
 *
 * The version of the OpenAPI document: 3.0.0
 * Contact: support@blockdaemon.com
 * Generated by: https://openapi-generator.tech
 */

/// ReportFieldMeta : Additional metadata bespoke to specific protocols.


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ReportFieldMeta {
    #[serde(rename="algorand_meta")]
    AlgorandMeta {
        /// The sender reward amount.
        #[serde(rename = "sender_reward", skip_serializing_if = "Option::is_none")]
        sender_reward: Option<String>,
        /// The recipient reward amount.
        #[serde(rename = "recipient_reward", skip_serializing_if = "Option::is_none")]
        recipient_reward: Option<String>,
        /// The closing account.
        #[serde(rename = "close", skip_serializing_if = "Option::is_none")]
        close: Option<String>,
        ///  The closing amount.
        #[serde(rename = "close_amount", skip_serializing_if = "Option::is_none")]
        close_amount: Option<String>,
        /// The closing reward.
        #[serde(rename = "close_reward", skip_serializing_if = "Option::is_none")]
        close_reward: Option<String>,
    },
}




