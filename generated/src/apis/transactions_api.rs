/*
 * Ubiquity REST API
 *
 * Ubiquity provides a RESTful and uniform way to access blockchain resources, with a rich and reusable model across multiple cryptocurrencies.  [Documentation](https://app.blockdaemon.com/docs/ubiquity)  ### Protocols #### Mainnet The following protocols are currently supported: * bitcoin * ethereum * polkadot * xrp * algorand * stellar * dogecoin * oasis * stacks * near  #### Testnet * bitcoin/testnet * ethereum/ropsten * dogecoin/testnet  #### Native Ubiquity provides native access to all Blockchain nodes it supports. To access native functionality, use the protocol without the v2 prefix * bitcoin/(mainnet | testnet) - [RPC Documentation](https://developer.bitcoin.org/reference/rpc/) * ethereum/(mainnet | ropsten) - [RPC Documentation](https://ethereum.org/en/developers/docs/apis/json-rpc/) * polkadot/mainnet - [Sidecar API Documentation](https://paritytech.github.io/substrate-api-sidecar/dist/) * polkadot/mainnet/http-rpc - [Polkadot RPC Documentation](https://polkadot.js.org/docs/substrate/rpc/) * algorand/mainnet - [Algod API Documentation](https://developer.algorand.org/docs/reference/rest-apis/algod/v1/) * stellar/mainnet - [Stellar Horizon API Documentation](https://developers.stellar.org/api) * dogecoin/(mainnet | testnet) - [Dogecoin API Documentaion](https://developer.bitcoin.org/reference/rpc/) * oasis/mainnet - [Oasis Rosetta Gateway Documentation](https://www.rosetta-api.org/docs/api_identifiers.html#network-identifier) * stacks/mainnet - [Stacks API Documentation](https://blockstack.github.io/stacks-blockchain-api/) * near/mainnet - [NEAR RPC Documentation](https://docs.near.org/docs/api/rpc)  A full URL example: https://ubiquity.api.blockdaemon.com/_**bitcoin/mainnet**  ##### Pagination Certain resources contain a lot of data, more than what's practical to return for a single request. With the help of pagination, the data is split across multiple responses. Each response returns a subset of the items requested, and a continuation token.  To get the next batch of items, copy the returned continuation token to the continuation query parameter and repeat the request with the new URL. In case no continuation token is returned, there is no more data available. 
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@blockdaemon.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `estimate_fee`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EstimateFeeError {
    Status401(crate::models::Error),
    Status429(crate::models::Error),
    Status400(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_tx`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTxError {
    Status400(crate::models::Error),
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_txs`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTxsError {
    Status401(crate::models::Error),
    Status403(crate::models::Error),
    Status429(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `tx_send`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TxSendError {
    Status401(crate::models::Error),
    Status429(crate::models::Error),
    Status400(crate::models::Error),
    UnknownValue(serde_json::Value),
}


/// Get a fee estimation in decimals from the network. If supported by the platform, the number of blocks used to make the estimation can be customized by the confirmed_within_blocks query parameter. 
pub async fn estimate_fee(configuration: &configuration::Configuration, platform: &str, network: &str, confirmed_within_blocks: Option<i32>) -> Result<String, Error<EstimateFeeError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/{platform}/{network}/tx/estimate_fee", configuration.base_path, platform=crate::apis::urlencode(platform), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = confirmed_within_blocks {
        local_var_req_builder = local_var_req_builder.query(&[("confirmed_within_blocks", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<EstimateFeeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_tx(configuration: &configuration::Configuration, platform: &str, network: &str, id: &str) -> Result<crate::models::Tx, Error<GetTxError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/{platform}/{network}/tx/{id}", configuration.base_path, platform=crate::apis::urlencode(platform), network=crate::apis::urlencode(network), id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetTxError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get all transactions on the platform, starting with the lastest one. Each call returns a slice of the entire list. Use the returned continuation token to get the next part.
pub async fn get_txs(configuration: &configuration::Configuration, platform: &str, network: &str, order: Option<&str>, continuation: Option<&str>, limit: Option<i32>, assets: Option<&str>) -> Result<crate::models::TxPage, Error<GetTxsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/{platform}/{network}/txs", configuration.base_path, platform=crate::apis::urlencode(platform), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = order {
        local_var_req_builder = local_var_req_builder.query(&[("order", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = continuation {
        local_var_req_builder = local_var_req_builder.query(&[("continuation", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = assets {
        local_var_req_builder = local_var_req_builder.query(&[("assets", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetTxsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Submit a signed transaction to the network.  **Note**: A successful transaction may still be rejected on chain or not processed due to a too low fee. You can monitor successful transactions through Ubiquity websockets. 
pub async fn tx_send(configuration: &configuration::Configuration, platform: &str, network: &str, signed_tx: crate::models::SignedTx) -> Result<crate::models::TxReceipt, Error<TxSendError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/{platform}/{network}/tx/send", configuration.base_path, platform=crate::apis::urlencode(platform), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&signed_tx);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<TxSendError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

