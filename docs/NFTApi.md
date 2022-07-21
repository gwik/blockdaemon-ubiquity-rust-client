# \NFTApi

All URIs are relative to *https://ubiquity.api.blockdaemon.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_asset**](NFTApi.md#get_asset) | **GET** /nft/{protocol}/{network}/asset/{id} | 
[**get_collection**](NFTApi.md#get_collection) | **GET** /nft/{protocol}/{network}/collection/{id} | 
[**get_event**](NFTApi.md#get_event) | **GET** /nft/{protocol}/{network}/event/{id} | 
[**list_assets**](NFTApi.md#list_assets) | **GET** /nft/{protocol}/{network}/assets | 
[**list_collections**](NFTApi.md#list_collections) | **GET** /nft/{protocol}/{network}/collections | 
[**list_events**](NFTApi.md#list_events) | **GET** /nft/{protocol}/{network}/events | 
[**search_collections**](NFTApi.md#search_collections) | **GET** /nft/{protocol}/{network}/collections/search | 



## get_asset

> crate::models::GetAssetResponse get_asset(protocol, network, id, contract_address, token_id, show_wallets)


Returns detailed information about an NFT asset by a given unique asset ID or  by a given contact address and token ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **i32** | Protocol handle, example: ethereum | [required] |
**network** | **i32** | Which network to target, example: mainnet | [required] |
**id** | **String** | Gets the Asset with matching `id` | [required] |
**contract_address** | Option<**String**> | Used in conjunction with `token_id` to get an Asset |  |
**token_id** | Option<**String**> | Used in conjunction with `contract_address` to get an Asset |  |
**show_wallets** | Option<**bool**> | Shows associated wallets when set to true |  |

### Return type

[**crate::models::GetAssetResponse**](GetAssetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collection

> crate::models::GetCollectionResponse get_collection(protocol, network, id, contract_address)


Returns detailed information about an NFT collection by a given unique collection ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **i32** | Protocol handle, example: ethereum | [required] |
**network** | **i32** | Which network to target, example: mainnet | [required] |
**id** | **String** | Gets the Collection with matching `id` | [required] |
**contract_address** | Option<**String**> | Gets the Collection with matching `contract_address` |  |

### Return type

[**crate::models::GetCollectionResponse**](GetCollectionResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event

> crate::models::GetEventResponse get_event(protocol, network, id)


Returns detailed information about an NFT event by a given event ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **i32** | Protocol handle, example: ethereum | [required] |
**network** | **i32** | Which network to target, example: mainnet | [required] |
**id** | **String** | Gets the Event with matching `id` | [required] |

### Return type

[**crate::models::GetEventResponse**](GetEventResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_assets

> crate::models::ListAssetsResponse list_assets(protocol, network, wallet_address, contract_address, token_id, collection_name, sort_by, order, page_size, page_token, attributes, token_type, show_wallets)


Returns NFT assets by a given collection, contract, or wallet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **i32** | Protocol handle, example: ethereum | [required] |
**network** | **i32** | Which network to target, example: mainnet | [required] |
**wallet_address** | Option<**String**> | Lists Assets by `wallet_address` |  |
**contract_address** | Option<**String**> | Lists Assets by `contract_address` |  |
**token_id** | Option<**String**> | Filters results by `token_id` requires `contract_address` |  |
**collection_name** | Option<**String**> | Lists Assets by `collection_name` |  |
**sort_by** | Option<**String**> | One of: name, token_id, mint_date |  |
**order** | Option<**i32**> | One of: asc, desc |  |
**page_size** | Option<**i32**> | Limit results number of results by `page_size` where maximum is 50 |  |
**page_token** | Option<**String**> | Base64 encoded cursor used to retrieve next page of results |  |
**attributes** | Option<[**Vec<String>**](String.md)> | Filters results by attribute pairs in the format key:value |  |
**token_type** | Option<[**Vec<i32>**](i32.md)> | Filters by `token_type`, one of: ERC721, ERC1155, CRYPTOPUNKS or ERC20 |  |
**show_wallets** | Option<**bool**> | Shows associated wallets when set to true |  |

### Return type

[**crate::models::ListAssetsResponse**](ListAssetsResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_collections

> crate::models::ListCollectionResponse list_collections(protocol, network, contract_address, collection_name, sort_by, order, page_size, page_token, token_type, verified)


Returns a list of all Collections in the network, which can be filtered by a given  collection name, contract address or token type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **i32** | Protocol handle, example: ethereum | [required] |
**network** | **i32** | Which network to target, example: mainnet | [required] |
**contract_address** | Option<[**Vec<String>**](String.md)> | Lists collections with matching `contract_address`, repeatable field |  |
**collection_name** | Option<[**Vec<String>**](String.md)> | Lists Collections matching provided `collection_name`, repeatable field |  |
**sort_by** | Option<**String**> | Sort by one of: name |  |
**order** | Option<**i32**> | One of: asc, desc |  |
**page_size** | Option<**i32**> | Limit results number of results by `page_size` where maximum is 500 |  |
**page_token** | Option<**String**> | Base64 encoded cursor used to retrieve next page of results |  |
**token_type** | Option<[**Vec<i32>**](i32.md)> | Filters by `token_type`, one of: ERC721, ERC1155, CRYPTOPUNKS or ERC20, repeatable field |  |
**verified** | Option<**bool**> | Only verified collections will be returned when set to true |  |

### Return type

[**crate::models::ListCollectionResponse**](ListCollectionResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_events

> crate::models::ListEventResponse list_events(protocol, network, contract_address, wallet_address, token_id, event_type, sort_by, order, page_size, page_token)


Returns NFT events by a given contract or wallet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **i32** | Protocol handle, example: ethereum | [required] |
**network** | **i32** | Which network to target, example: mainnet | [required] |
**contract_address** | Option<**String**> | Lists Events by `contract_address` |  |
**wallet_address** | Option<**String**> | Lists Events by `wallet_address` |  |
**token_id** | Option<**String**> | Filters Events by `token_id`, requires `contract_address` |  |
**event_type** | Option<**String**> | Filters Events by `event_type`, one of: mint, transfer, sale |  |
**sort_by** | Option<**String**> | Sort by one of: timestamp |  |
**order** | Option<**i32**> | One of: asc, desc |  |
**page_size** | Option<**i32**> | Limit results number of results by `page_size` where maximum is 500 |  |
**page_token** | Option<**String**> | Base64 encoded cursor used to retrieve next page of results |  |

### Return type

[**crate::models::ListEventResponse**](ListEventResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_collections

> crate::models::SearchCollectionResponse search_collections(protocol, network, name)


Returns NFT collections with names matching a given search string.  The response includes top 50 most relevant results, sorted in descending order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **i32** | Protocol handle, example: ethereum | [required] |
**network** | **i32** | Which network to target, example: mainnet | [required] |
**name** | Option<**String**> | Free text search on collection name, returning closest matching results |  |

### Return type

[**crate::models::SearchCollectionResponse**](SearchCollectionResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

