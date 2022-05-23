# \NFTApi

All URIs are relative to *https://ubiquity.api.blockdaemon.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**explorer_get_collection**](NFTApi.md#explorer_get_collection) | **get** /nft/{protocol}/{network}/collection/{id} | 
[**explorer_list_assets**](NFTApi.md#explorer_list_assets) | **get** /nft/{protocol}/{network}/assets | 
[**explorer_list_collections**](NFTApi.md#explorer_list_collections) | **get** /nft/{protocol}/{network}/collections | 
[**explorer_list_events**](NFTApi.md#explorer_list_events) | **get** /nft/{protocol}/{network}/events | 



## explorer_get_collection

> crate::models::GetCollectionResponse explorer_get_collection(protocol, network, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Coin platform handle | [required] |
**network** | **String** | Which network to target | [required] |
**id** | **String** | Mapped to URL query parameter 'uuid' | [required] |

### Return type

[**crate::models::GetCollectionResponse**](GetCollectionResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## explorer_list_assets

> crate::models::ListAssetsResponse explorer_list_assets(protocol, network, wallet_address, contract_address, token_id_value, collection_name, sort_by, order, page_size, page_token, attributes)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Coin platform handle | [required] |
**network** | **String** | Which network to target | [required] |
**wallet_address** | Option<**String**> | Mapped to URL query parameter `wallet_address` |  |
**contract_address** | Option<**String**> | Mapped to URL query parameter `contract_address` |  |
**token_id_value** | Option<**i64**> | The int64 value. |  |
**collection_name** | Option<**String**> | Mapped to URL query parameter `collection_name` |  |
**sort_by** | Option<**String**> | One of: name, token_id, mint_date |  |
**order** | Option<**String**> | Mapped to URL query parameter `order` One of: asc, desc |  |
**page_size** | Option<**i32**> | Mapped to URL query parameter `page_size` |  |
**page_token** | Option<**String**> | Mapped to URL query parameter `page_token` base64 encoded cursor |  |
**attributes** | Option<[**Vec<String>**](String.md)> | Mapped to URL query parameter `attributes` |  |

### Return type

[**crate::models::ListAssetsResponse**](ListAssetsResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## explorer_list_collections

> crate::models::ListCollectionResponse explorer_list_collections(protocol, network, contract_address, collection_name, sort_by, order, page_size, page_token)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Coin platform handle | [required] |
**network** | **String** | Which network to target | [required] |
**contract_address** | Option<[**Vec<String>**](String.md)> | Mapped to URL query parameter 'contract_address' |  |
**collection_name** | Option<[**Vec<String>**](String.md)> | Mapped to URL query parameter 'collection_name' |  |
**sort_by** | Option<**String**> | Sort by one of: name |  |
**order** | Option<**String**> | Mapped to URL query parameter `order` One of: asc, desc |  |
**page_size** | Option<**i32**> | Mapped to URL query parameter `page_size` |  |
**page_token** | Option<**String**> | Mapped to URL query parameter `page_token` base64 encoded cursor |  |

### Return type

[**crate::models::ListCollectionResponse**](ListCollectionResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## explorer_list_events

> crate::models::ListEventResponse explorer_list_events(protocol, network, contract_address, wallet_address, token_id, event_type, sort_by, order, page_size, page_token)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Coin platform handle | [required] |
**network** | **String** | Which network to target | [required] |
**contract_address** | Option<**String**> | mapped to URL query parameter 'contract_address' |  |
**wallet_address** | Option<**String**> | mapped to URL query parameter 'wallet_address' |  |
**token_id** | Option<**i64**> | mapped to URL query parameter 'token_id' |  |
**event_type** | Option<**String**> | mapped to URL query parameter 'event_type' |  |
**sort_by** | Option<**String**> | Sort by one of: timestamp |  |
**order** | Option<**String**> | Mapped to URL query parameter `order` One of: asc, desc |  |
**page_size** | Option<**i32**> | Mapped to URL query parameter `page_size` |  |
**page_token** | Option<**String**> | Mapped to URL query parameter `page_token` base64 encoded cursor |  |

### Return type

[**crate::models::ListEventResponse**](ListEventResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

