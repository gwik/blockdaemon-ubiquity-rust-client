/*
 * Universal REST API
 *
 * Universal API provides a RESTful and uniform way to access blockchain resources, with a rich and reusable model across multiple protocols/cryptocurrencies.  [Documentation](https://app.blockdaemon.com/docs/ubiquity)  ### Currently supported protocols:  * algorand   * mainnet * bitcoin   * mainnet/testnet * bitcoincash   * mainnet/testnet * dogecoin   * mainnet/testnet * ethereum   * mainnet/goerli * litecoin   * mainnet/testnet * near   * mainnet/testnet * oasis   * mainnet * optimism   * mainnet * polkadot   * mainnet/westend * polygon   * mainnet/amoy * solana   * mainnet/testnet * stellar   * mainnet/testnet * tezos   * mainnet * xrp   * mainnet  ##### Pagination Certain resources contain a lot of data, more than what's practical to return for a single request. With the help of pagination, the data is split across multiple responses. Each response returns a subset of the items requested, and a continuation token.  To get the next batch of items, copy the returned continuation token to the continuation query parameter and repeat the request with the new URL. In case no continuation token is returned, there is no more data available. 
 *
 * The version of the OpenAPI document: 3.0.0
 * Contact: support@blockdaemon.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`get_protocol_endpoints`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetProtocolEndpointsError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    Status503(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_protocols_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetProtocolsListError {
    Status401(crate::models::Error),
    Status429(crate::models::Error),
    Status500(crate::models::Error),
    Status503(crate::models::Error),
    UnknownValue(serde_json::Value),
}


/// Provides information about supported endpoints and generic protocol information. 
pub async fn get_protocol_endpoints(configuration: &configuration::Configuration, protocol: &str, network: &str) -> Result<crate::models::ProtocolDetail, Error<GetProtocolEndpointsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/{protocol}/{network}", local_var_configuration.base_path, protocol=crate::apis::urlencode(protocol), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Key", local_var_value);
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetProtocolEndpointsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Provides a list of supported protocols and networks. 
pub async fn get_protocols_list(configuration: &configuration::Configuration, ) -> Result<crate::models::ProtocolsOverview, Error<GetProtocolsListError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Key", local_var_value);
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetProtocolsListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

