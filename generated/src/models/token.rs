/*
 * Ubiquity REST API
 *
 * Ubiquity provides a RESTful and uniform way to access blockchain resources, with a rich and reusable model across multiple cryptocurrencies.  [Documentation](https://app.blockdaemon.com/docs/ubiquity)  ### Protocols #### Mainnet The following protocols are currently supported: * bitcoin * ethereum * polkadot * xrp * algorand * stellar * dogecoin * oasis * near * litecoin * bitcoincash * tezos  #### Testnet * bitcoin/testnet * ethereum/ropsten * dogecoin/testnet * litecoin/testnet * bitcoincash/testnet  #### Native Ubiquity provides native access to all Blockchain nodes it supports. * bitcoin/(mainnet | testnet) - [RPC Documentation](https://developer.bitcoin.org/reference/rpc/) * ethereum/(mainnet | ropsten) - [RPC Documentation](https://ethereum.org/en/developers/docs/apis/json-rpc/) * polkadot/mainnet - [Sidecar API Documentation](https://paritytech.github.io/substrate-api-sidecar/dist/) * polkadot/mainnet/http-rpc - [Polkadot RPC Documentation](https://polkadot.js.org/docs/substrate/rpc/) * algorand/mainnet - [Algod API Documentation](https://developer.algorand.org/docs/reference/rest-apis/algod/) * stellar/mainnet - [Stellar Horizon API Documentation](https://developers.stellar.org/api) * dogecoin/(mainnet | testnet) - [Dogecoin API Documentaion](https://developer.bitcoin.org/reference/rpc/) * oasis/mainnet - [Oasis Rosetta Gateway Documentation](https://www.rosetta-api.org/docs/api_identifiers.html#network-identifier) * near/mainnet - [NEAR RPC Documentation](https://docs.near.org/docs/api/rpc) * litecoin/mainnet - [Litecoin RPC Documentation](https://litecoin.info/index.php/Litecoin_API) * bitcoincash/mainnet - [Bitcoin Cash RPC Documentation](https://docs.bitcoincashnode.org/doc/json-rpc/) * tezos/mainnet - [Tezos RPC Documentation](https://tezos.gitlab.io/developer/rpc.html)   A full URL example: https://ubiquity.api.blockdaemon.com/v1/bitcoin/mainnet  ##### Pagination Certain resources contain a lot of data, more than what's practical to return for a single request. With the help of pagination, the data is split across multiple responses. Each response returns a subset of the items requested, and a continuation token.  To get the next batch of items, copy the returned continuation token to the continuation query parameter and repeat the request with the new URL. In case no continuation token is returned, there is no more data available. 
 *
 * The version of the OpenAPI document: 3.0.0
 * Contact: support@blockdaemon.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Token {
    /// Name of token mechanism
    #[serde(rename = "type")]
    pub _type: String,
    /// Token identifier
    #[serde(rename = "id")]
    pub id: String,
    /// Address that created token
    #[serde(rename = "creator", skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
}

impl Token {
    pub fn new(_type: String, id: String) -> Token {
        Token {
            _type,
            id,
            creator: None,
        }
    }
}


