# \BlocksApi

All URIs are relative to *https://svc.blockdaemon.com/universal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_block_by_number**](BlocksApi.md#get_block_by_number) | **GET** /{protocol}/{network}/block/{block_identifier} | Get a Block by Number or Hash
[**get_block_identifier_by_number**](BlocksApi.md#get_block_identifier_by_number) | **GET** /{protocol}/{network}/block_identifier/{block_identifier} | Get a Block Identifier by Number
[**get_block_identifiers**](BlocksApi.md#get_block_identifiers) | **GET** /{protocol}/{network}/block_identifiers | Get a List of Block Identifiers
[**get_current_block_hash**](BlocksApi.md#get_current_block_hash) | **GET** /{protocol}/{network}/sync/block_id | Get the Current Block Hash
[**get_current_block_number**](BlocksApi.md#get_current_block_number) | **GET** /{protocol}/{network}/sync/block_number | Get the Current Block Number



## get_block_by_number

> crate::models::Block get_block_by_number(protocol, network, block_identifier)
Get a Block by Number or Hash

Get a block and all its transactions by a user-defined block number or block hash.   Use `-1` or `current` parameter to return the current block. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle, one of:  `algorand`, `bitcoin`, `bitcoincash`, `dogecoin`, `ethereum`, `litecoin`, `near`, `oasis`, `optimism`, `polkadot`, `polygon`, `solana`, `stellar`, `tezos`, `xrp`.  | [required] |[default to bitcoin]
**network** | **String** | Which network to target. Available networks can be found in the list of supported protocols or with /{protocol}. | [required] |[default to mainnet]
**block_identifier** | **String** | The block identifier, hash or number. | [required] |[default to 0000000000000000000590fc0f3eba193a278534220b2b37e9849e1a770ca959]

### Return type

[**crate::models::Block**](block.md)

### Authorization

[apiKeyAuthHeader](../README.md#apiKeyAuthHeader), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_block_identifier_by_number

> crate::models::BlockIdentifier get_block_identifier_by_number(protocol, network, block_identifier)
Get a Block Identifier by Number

Get the minimal block data, block header, by a user-defined block number or block hash.   Use `-1` or `current` parameter to return the current block. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle, one of:  `algorand`, `bitcoin`, `bitcoincash`, `dogecoin`, `ethereum`, `litecoin`, `near`, `oasis`, `optimism`, `polkadot`, `polygon`, `solana`, `stellar`, `tezos`, `xrp`.  | [required] |[default to bitcoin]
**network** | **String** | Which network to target. Available networks can be found in the list of supported protocols or with /{protocol}. | [required] |[default to mainnet]
**block_identifier** | **String** | The block identifier, hash or number. | [required] |[default to 0000000000000000000590fc0f3eba193a278534220b2b37e9849e1a770ca959]

### Return type

[**crate::models::BlockIdentifier**](block_identifier.md)

### Authorization

[apiKeyAuthHeader](../README.md#apiKeyAuthHeader), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_block_identifiers

> crate::models::BlockIdentifiers get_block_identifiers(protocol, network, order, page_token, page_size)
Get a List of Block Identifiers

Returns a list of minimal block data, block headers such as block hash and block number.  The response is paginated: use the returned `next_page_token` token as a query parameter to get the next page. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle, one of:  `algorand`, `bitcoin`, `bitcoincash`, `dogecoin`, `ethereum`, `litecoin`, `near`, `oasis`, `optimism`, `polkadot`, `polygon`, `solana`, `stellar`, `tezos`, `xrp`.  | [required] |[default to bitcoin]
**network** | **String** | Which network to target. Available networks can be found in the list of supported protocols or with /{protocol}. | [required] |[default to mainnet]
**order** | Option<**String**> | The pagination order. |  |[default to desc]
**page_token** | Option<**String**> | The continuation token from earlier response. |  |
**page_size** | Option<**i32**> | Max number of items to return in a response. Defaults to 25 and is capped at 100.  |  |

### Return type

[**crate::models::BlockIdentifiers**](block_identifiers.md)

### Authorization

[apiKeyAuthHeader](../README.md#apiKeyAuthHeader), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/png, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_block_hash

> String get_current_block_hash(protocol, network)
Get the Current Block Hash

Get the current block id (hash) of the protocol.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle, one of:  `algorand`, `bitcoin`, `bitcoincash`, `dogecoin`, `ethereum`, `litecoin`, `near`, `oasis`, `optimism`, `polkadot`, `polygon`, `solana`, `stellar`, `tezos`, `xrp`.  | [required] |[default to bitcoin]
**network** | **String** | Which network to target. Available networks can be found in the list of supported protocols or with /{protocol}. | [required] |[default to mainnet]

### Return type

**String**

### Authorization

[apiKeyAuthHeader](../README.md#apiKeyAuthHeader), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_block_number

> i64 get_current_block_number(protocol, network)
Get the Current Block Number

Returns the current block number/height of the protocol.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle, one of:  `algorand`, `bitcoin`, `bitcoincash`, `dogecoin`, `ethereum`, `litecoin`, `near`, `oasis`, `optimism`, `polkadot`, `polygon`, `solana`, `stellar`, `tezos`, `xrp`.  | [required] |[default to bitcoin]
**network** | **String** | Which network to target. Available networks can be found in the list of supported protocols or with /{protocol}. | [required] |[default to mainnet]

### Return type

**i64**

### Authorization

[apiKeyAuthHeader](../README.md#apiKeyAuthHeader), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

