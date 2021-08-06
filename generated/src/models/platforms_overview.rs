/*
 * Ubiquity REST API
 *
 * Ubiquity provides a RESTful and uniform way to access blockchain resources, with a rich and reusable model across multiple cryptocurrencies.  [Documentation](https://app.blockdaemon.com/docs/ubiquity)  ### Protocols #### Mainnet The following protocols are currently supported: * bitcoin * ethereum * polkadot * xrp * algorand * stellar * dogecoin * oasis * stacks  #### Testnet * bitcoin/testnet * ethereum/ropsten * dogecoin/testnet  ##### Pagination Certain resources contain a lot of data, more than what's practical to return for a single request. With the help of pagination, the data is split across multiple responses. Each response returns a subset of the items requested and a continuation token.  To get the next batch of items, copy the returned continuation token to the continuation query parameter and repeat the request with the new URL. In case no continuation token is returned, there is no more data available. 
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@blockdaemon.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlatformsOverview {
    /// List of items each describing a pair of supported platform and network.
    #[serde(rename = "platforms", skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<crate::models::PlatformsOverviewPlatforms>>,
}

impl PlatformsOverview {
    pub fn new() -> PlatformsOverview {
        PlatformsOverview {
            platforms: None,
        }
    }
}


