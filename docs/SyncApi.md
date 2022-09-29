# \SyncApi

All URIs are relative to *https://svc.blockdaemon.com/universal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**current_block_id**](SyncApi.md#current_block_id) | **GET** /{protocol}/{network}/sync/block_id | Get current block ID
[**current_block_number**](SyncApi.md#current_block_number) | **GET** /{protocol}/{network}/sync/block_number | Get current block number



## current_block_id

> String current_block_id(protocol, network)
Get current block ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{protocol} | [required] |

### Return type

**String**

### Authorization

[apiKeyAuthHeader](../README.md#apiKeyAuthHeader), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## current_block_number

> i64 current_block_number(protocol, network)
Get current block number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{protocol} | [required] |

### Return type

**i64**

### Authorization

[apiKeyAuthHeader](../README.md#apiKeyAuthHeader), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

