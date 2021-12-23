# \AccountsApi

All URIs are relative to *https://ubiquity.api.blockdaemon.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_balances_by_address**](AccountsApi.md#get_balances_by_address) | **GET** /{platform}/{network}/account/{address} | Balances Of Address
[**get_balances_by_addresses**](AccountsApi.md#get_balances_by_addresses) | **POST** /{platform}/{network}/accounts | Balances Of Addresses
[**get_report_by_address**](AccountsApi.md#get_report_by_address) | **GET** /{platform}/{network}/account/{address}/report | A financial report for an address between a time period. Default timescale is within the last 30 days
[**get_txs_by_address**](AccountsApi.md#get_txs_by_address) | **GET** /{platform}/{network}/account/{address}/txs | Transactions Of Address



## get_balances_by_address

> ::std::collections::HashMap<String, serde_json::Value> get_balances_by_address(platform, network, address, assets)
Balances Of Address

Returns the account balances for all supported currencies. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** | Coin platform handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{platform} | [required] |
**address** | **String** | Account address | [required] |
**assets** | Option<**String**> | Comma-separated list of asset paths to filter. If the list is empty, or all elements are empty, this filter has no effect. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_balances_by_addresses

> ::std::collections::HashMap<String, ::std::collections::HashMap<String, serde_json::Value>> get_balances_by_addresses(platform, network, accounts_obj, assets)
Balances Of Addresses

Returns the balances of accounts for all supported currencies. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** | Coin platform handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{platform} | [required] |
**accounts_obj** | [**AccountsObj**](AccountsObj.md) |  | [required] |
**assets** | Option<**String**> | Comma-separated list of asset paths to filter. If the list is empty, or all elements are empty, this filter has no effect. |  |

### Return type

[**::std::collections::HashMap<String, ::std::collections::HashMap<String, serde_json::Value>>**](map.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_report_by_address

> crate::models::Report get_report_by_address(platform, network, address, from, to)
A financial report for an address between a time period. Default timescale is within the last 30 days

Returns account activity 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** | Coin platform handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{platform} | [required] |
**address** | **String** | Account address | [required] |
**from** | Option<**i32**> | Unix Timestamp from where to start |  |
**to** | Option<**i32**> | Unix Timestamp from where to end |  |

### Return type

[**crate::models::Report**](report.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_txs_by_address

> crate::models::TxPage get_txs_by_address(platform, network, address, order, continuation, limit, assets)
Transactions Of Address

Gets transactions that an address was involved with, from newest to oldest. This call uses pagination. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** | Coin platform handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{platform} | [required] |
**address** | **String** | Account address | [required] |
**order** | Option<**String**> | Pagination order |  |
**continuation** | Option<**String**> | Continuation token from earlier response |  |
**limit** | Option<**i32**> | Max number of items to return in a response. Defaults to 25 and is capped at 100.  |  |
**assets** | Option<**String**> | Comma-separated list of asset paths to filter. If the list is empty, or all elements are empty, this filter has no effect. |  |

### Return type

[**crate::models::TxPage**](tx_page.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

