# \BlocksApi

All URIs are relative to *https://ubiquity.api.blockdaemon.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_block**](BlocksApi.md#get_block) | **GET** /{platform}/{network}/block/{key} | Block By Number/Hash
[**get_block_identifier**](BlocksApi.md#get_block_identifier) | **GET** /{platform}/{network}/block_identifier/{key} | Block Identifier By Number/Hash
[**get_block_identifiers**](BlocksApi.md#get_block_identifiers) | **GET** /{platform}/{network}/block_identifiers | Block Identifiers



## get_block

> crate::models::Block get_block(platform, network, key)
Block By Number/Hash

Get a block and all its transactions by the block number or hash

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** | Coin platform handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{platform} | [required] |
**key** | **String** | Block number or block hash/ID or Special identifier | [required] |

### Return type

[**crate::models::Block**](block.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_block_identifier

> crate::models::BlockIdentifier get_block_identifier(platform, network, key)
Block Identifier By Number/Hash

Get minimal block identifier by block number or hash

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** | Coin platform handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{platform} | [required] |
**key** | **String** | Block number or block hash/ID or Special identifier | [required] |

### Return type

[**crate::models::BlockIdentifier**](block_identifier.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_block_identifiers

> crate::models::BlockIdentifierPage get_block_identifiers(platform, network)
Block Identifiers

Get minimal block identifiers from oldest to newest. This call uses pagination. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** | Coin platform handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{platform} | [required] |

### Return type

[**crate::models::BlockIdentifierPage**](block_identifier_page.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

