/*
 * Ubiquity REST API
 *
 * Ubiquity provides a RESTful and uniform way to access blockchain resources, with a rich and reusable model across multiple cryptocurrencies.  [Documentation](https://app.blockdaemon.com/docs/ubiquity)  ### Protocols #### Mainnet The following protocols are currently supported: * bitcoin * ethereum * polkadot * xrp * algorand * stellar * dogecoin * oasis * stacks * near  #### Testnet * bitcoin/testnet * ethereum/ropsten * dogecoin/testnet  #### Native Ubiquity provides native access to all Blockchain nodes it supports. To access native functionality, use the protocol without the v2 prefix * bitcoin/(mainnet | testnet) - [RPC Documentation](https://developer.bitcoin.org/reference/rpc/) * ethereum/(mainnet | ropsten) - [RPC Documentation](https://ethereum.org/en/developers/docs/apis/json-rpc/) * polkadot/mainnet - [Sidecar API Documentation](https://paritytech.github.io/substrate-api-sidecar/dist/) * polkadot/mainnet/http-rpc - [Polkadot RPC Documentation](https://polkadot.js.org/docs/substrate/rpc/) * algorand/mainnet - [Algod API Documentation](https://developer.algorand.org/docs/reference/rest-apis/algod/v1/) * stellar/mainnet - [Stellar Horizon API Documentation](https://developers.stellar.org/api) * dogecoin/(mainnet | testnet) - [Dogecoin API Documentaion](https://developer.bitcoin.org/reference/rpc/) * oasis/mainnet - [Oasis Rosetta Gateway Documentation](https://www.rosetta-api.org/docs/api_identifiers.html#network-identifier) * stacks/mainnet - [Stacks API Documentation](https://blockstack.github.io/stacks-blockchain-api/) * near/mainnet - [NEAR RPC Documentation](https://docs.near.org/docs/api/rpc)  A full URL example: https://ubiquity.api.blockdaemon.com/_**bitcoin/mainnet**  ##### Pagination Certain resources contain a lot of data, more than what's practical to return for a single request. With the help of pagination, the data is split across multiple responses. Each response returns a subset of the items requested, and a continuation token.  To get the next batch of items, copy the returned continuation token to the continuation query parameter and repeat the request with the new URL. In case no continuation token is returned, there is no more data available. 
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@blockdaemon.com
 * Generated by: https://openapi-generator.tech
 */



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Currency {
    #[serde(rename="native")]
    NativeCurrency {
        /// Asset path of transferred currency
        #[serde(rename = "asset_path")]
        asset_path: String,
        /// Currency symbol
        #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
        symbol: Option<String>,
        /// Name of currency
        #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        /// Decimal places right to the comma
        #[serde(rename = "decimals", skip_serializing_if = "Option::is_none")]
        decimals: Option<i32>,
    },
    #[serde(rename="smart_token")]
    SmartTokenCurrency {
        /// Asset path of transferred currency
        #[serde(rename = "asset_path")]
        asset_path: String,
        /// Currency symbol
        #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
        symbol: Option<String>,
        /// Name of currency
        #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        /// Decimal places right to the comma
        #[serde(rename = "decimals", skip_serializing_if = "Option::is_none")]
        decimals: Option<i32>,
        #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
        detail: Option<Box<crate::models::SmartToken>>,
    },
    #[serde(rename="token")]
    TokenCurrency {
        /// Asset path of transferred currency
        #[serde(rename = "asset_path")]
        asset_path: String,
        /// Currency symbol
        #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
        symbol: Option<String>,
        /// Name of currency
        #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        /// Decimal places right to the comma
        #[serde(rename = "decimals", skip_serializing_if = "Option::is_none")]
        decimals: Option<i32>,
        #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
        detail: Option<Box<crate::models::Token>>,
    },
}




