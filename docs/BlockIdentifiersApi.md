# \BlockIdentifiersApi

All URIs are relative to *https://svc.blockdaemon.com/universal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_block_identifier**](BlockIdentifiersApi.md#get_block_identifier) | **GET** /{protocol}/{network}/block_identifier/{block_identifier} | Block Identifier By Hash
[**get_block_identifiers**](BlockIdentifiersApi.md#get_block_identifiers) | **GET** /{protocol}/{network}/block_identifiers | Block Identifiers



## get_block_identifier

> crate::models::BlockIdentifier get_block_identifier(protocol, network, block_identifier)
Block Identifier By Hash

Get minimal block identifier by block hash

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{protocol} | [required] |
**block_identifier** | **String** | Block ID or number | [required] |

### Return type

[**crate::models::BlockIdentifier**](block_identifier.md)

### Authorization

[apiKeyAuthHeader](../README.md#apiKeyAuthHeader), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_block_identifiers

> crate::models::BlockIdentifiers get_block_identifiers(protocol, network, order, continuation, limit)
Block Identifiers

Get minimal block identifiers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{protocol} | [required] |
**order** | Option<**String**> | Pagination order |  |
**continuation** | Option<**String**> | Continuation token from earlier response |  |
**limit** | Option<**i32**> | Max number of items to return in a response. Defaults to 25 and is capped at 100.  |  |

### Return type

[**crate::models::BlockIdentifiers**](block_identifiers.md)

### Authorization

[apiKeyAuthHeader](../README.md#apiKeyAuthHeader), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

