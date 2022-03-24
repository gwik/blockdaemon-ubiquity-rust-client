/*
 * Ubiquity REST API
 *
 * Ubiquity provides a RESTful and uniform way to access blockchain resources, with a rich and reusable model across multiple cryptocurrencies.  [Documentation](https://app.blockdaemon.com/docs/ubiquity)  ### Protocols #### Mainnet The following protocols are currently supported: * bitcoin * ethereum * polkadot * xrp * algorand * stellar * dogecoin * oasis * near * terra * litecoin * bitcoincash * solana  #### Testnet * bitcoin/testnet * ethereum/ropsten * dogecoin/testnet * litecoin/testnet * bitcoincash/testnet  #### Native Ubiquity provides native access to all Blockchain nodes it supports. To access native functionality, use the protocol without the v2 prefix * bitcoin/(mainnet | testnet) - [RPC Documentation](https://developer.bitcoin.org/reference/rpc/) * ethereum/(mainnet | ropsten) - [RPC Documentation](https://ethereum.org/en/developers/docs/apis/json-rpc/) * polkadot/mainnet - [Sidecar API Documentation](https://paritytech.github.io/substrate-api-sidecar/dist/) * polkadot/mainnet/http-rpc - [Polkadot RPC Documentation](https://polkadot.js.org/docs/substrate/rpc/) * algorand/mainnet - [Algod API Documentation](https://developer.algorand.org/docs/reference/rest-apis/algod/v1/) * stellar/mainnet - [Stellar Horizon API Documentation](https://developers.stellar.org/api) * dogecoin/(mainnet | testnet) - [Dogecoin API Documentaion](https://developer.bitcoin.org/reference/rpc/) * oasis/mainnet - [Oasis Rosetta Gateway Documentation](https://www.rosetta-api.org/docs/api_identifiers.html#network-identifier) * near/mainnet - [NEAR RPC Documentation](https://docs.near.org/docs/api/rpc) * terra/mainnet - [Terra RPC Documentation](https://docs.terra.money/docs/develop/how-to/endpoints.html) * litecoin/mainnet - [Litecoin RPC Documentation](https://litecoin.info/index.php/Litecoin_API) * bitcoincash/mainnet - [Bitcoin Cash RPC Documentation](https://docs.bitcoincashnode.org/doc/json-rpc/) * solana/mainnet - [Solana RPC Documentation](https://docs.solana.com/developing/clients/jsonrpc-api)  A full URL example: https://ubiquity.api.blockdaemon.com/bitcoin/mainnet  ##### Pagination Certain resources contain a lot of data, more than what's practical to return for a single request. With the help of pagination, the data is split across multiple responses. Each response returns a subset of the items requested, and a continuation token.  To get the next batch of items, copy the returned continuation token to the continuation query parameter and repeat the request with the new URL. In case no continuation token is returned, there is no more data available. 
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@blockdaemon.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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


