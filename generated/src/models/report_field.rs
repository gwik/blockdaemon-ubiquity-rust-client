/*
 * Ubiquity REST API
 *
 * Ubiquity provides a RESTful and uniform way to access blockchain resources, with a rich and reusable model across multiple cryptocurrencies.  [Documentation](https://app.blockdaemon.com/docs/ubiquity)  ### Protocols #### Mainnet The following protocols are currently supported: * bitcoin * ethereum * polkadot * xrp * algorand * stellar * dogecoin * oasis * near * terra * litecoin * bitcoincash * tezos  #### Testnet * bitcoin/testnet * ethereum/ropsten * dogecoin/testnet * litecoin/testnet * bitcoincash/testnet  #### Native Ubiquity provides native access to all Blockchain nodes it supports. * bitcoin/(mainnet | testnet) - [RPC Documentation](https://developer.bitcoin.org/reference/rpc/) * ethereum/(mainnet | ropsten) - [RPC Documentation](https://ethereum.org/en/developers/docs/apis/json-rpc/) * polkadot/mainnet - [Sidecar API Documentation](https://paritytech.github.io/substrate-api-sidecar/dist/) * polkadot/mainnet/http-rpc - [Polkadot RPC Documentation](https://polkadot.js.org/docs/substrate/rpc/) * algorand/mainnet - [Algod API Documentation](https://developer.algorand.org/docs/reference/rest-apis/algod/) * stellar/mainnet - [Stellar Horizon API Documentation](https://developers.stellar.org/api) * dogecoin/(mainnet | testnet) - [Dogecoin API Documentaion](https://developer.bitcoin.org/reference/rpc/) * oasis/mainnet - [Oasis Rosetta Gateway Documentation](https://www.rosetta-api.org/docs/api_identifiers.html#network-identifier) * near/mainnet - [NEAR RPC Documentation](https://docs.near.org/docs/api/rpc) * terra/mainnet - [Terra RPC Documentation](https://docs.terra.money/docs/develop/how-to/endpoints.html) * litecoin/mainnet - [Litecoin RPC Documentation](https://litecoin.info/index.php/Litecoin_API) * bitcoincash/mainnet - [Bitcoin Cash RPC Documentation](https://docs.bitcoincashnode.org/doc/json-rpc/) * tezos/mainnet - [Tezos RPC Documentation](https://tezos.gitlab.io/developer/rpc.html)   A full URL example: https://ubiquity.api.blockdaemon.com/bitcoin/mainnet  ##### Pagination Certain resources contain a lot of data, more than what's practical to return for a single request. With the help of pagination, the data is split across multiple responses. Each response returns a subset of the items requested, and a continuation token.  To get the next batch of items, copy the returned continuation token to the continuation query parameter and repeat the request with the new URL. In case no continuation token is returned, there is no more data available. 
 *
 * The version of the OpenAPI document: 3.0.0
 * Contact: support@blockdaemon.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReportField {
    /// The protocol the address relates to
    #[serde(rename = "protocol")]
    pub protocol: String,
    /// The wallet/account the transaction occurred
    #[serde(rename = "address")]
    pub address: String,
    /// The currency symbol
    #[serde(rename = "currency")]
    pub currency: String,
    /// The ID of the event within a transaction
    #[serde(rename = "event_id")]
    pub event_id: String,
    /// The block number the transaction occurred on
    #[serde(rename = "block")]
    pub block: i64,
    /// The unix timestamp when the transaction was added to a block
    #[serde(rename = "timestamp")]
    pub timestamp: i32,
    /// The transaction ID
    #[serde(rename = "hash")]
    pub hash: String,
    /// The action type e.g. Transfer, Deposit, Staking Reward etc..
    #[serde(rename = "action")]
    pub action: String,
    /// The amount of currency involved in the transaction (smallest unit)
    #[serde(rename = "value")]
    pub value: String,
    /// The address where the funds originated
    #[serde(rename = "sender_address")]
    pub sender_address: String,
    /// How much was charged as a fee for processing the transaction
    #[serde(rename = "fee")]
    pub fee: String,
    /// The number of decimals in one coin, used to convert smallest unit to 1 whole coin if needed
    #[serde(rename = "decimals")]
    pub decimals: i32,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ReportFieldMeta>>,
}

impl ReportField {
    pub fn new(protocol: String, address: String, currency: String, event_id: String, block: i64, timestamp: i32, hash: String, action: String, value: String, sender_address: String, fee: String, decimals: i32) -> ReportField {
        ReportField {
            protocol,
            address,
            currency,
            event_id,
            block,
            timestamp,
            hash,
            action,
            value,
            sender_address,
            fee,
            decimals,
            meta: None,
        }
    }
}


