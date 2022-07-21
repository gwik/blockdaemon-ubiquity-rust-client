# \AccountsApi

All URIs are relative to *https://ubiquity.api.blockdaemon.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_list_of_balances_by_address**](AccountsApi.md#get_list_of_balances_by_address) | **GET** /{protocol}/{network}/account/{address} | Balances Of Address
[**get_list_of_balances_by_addresses**](AccountsApi.md#get_list_of_balances_by_addresses) | **POST** /{protocol}/{network}/accounts | Balances Of Addresses
[**get_report_by_address**](AccountsApi.md#get_report_by_address) | **GET** /{protocol}/{network}/account/{address}/report | A financial report for an address between a time period. Default timescale is within the last 30 days
[**get_txs_by_address**](AccountsApi.md#get_txs_by_address) | **GET** /{protocol}/{network}/account/{address}/txs | Transactions Of Address
[**get_utxoby_account**](AccountsApi.md#get_utxoby_account) | **GET** /{protocol}/{network}/account/{address}/utxo | Endpoint for getting transaction inputs and outputs for a given account.



## get_list_of_balances_by_address

> Vec<crate::models::Balance> get_list_of_balances_by_address(protocol, network, address, assets)
Balances Of Address

Returns the account balances for all supported currencies. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{protocol} | [required] |
**address** | **String** | Account address | [required] |
**assets** | Option<**String**> | Comma-separated list of asset paths to filter. If the list is empty, or all elements are empty, this filter has no effect. |  |

### Return type

[**Vec<crate::models::Balance>**](balance.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_list_of_balances_by_addresses

> ::std::collections::HashMap<String, Vec<crate::models::Balance>> get_list_of_balances_by_addresses(protocol, network, accounts_obj, assets)
Balances Of Addresses

Returns the balances of accounts for all supported currencies. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{protocol} | [required] |
**accounts_obj** | [**AccountsObj**](AccountsObj.md) |  | [required] |
**assets** | Option<**String**> | Comma-separated list of asset paths to filter. If the list is empty, or all elements are empty, this filter has no effect. |  |

### Return type

[**::std::collections::HashMap<String, Vec<crate::models::Balance>>**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_report_by_address

> crate::models::Report get_report_by_address(protocol, network, address, from, to, continuation, limit)
A financial report for an address between a time period. Default timescale is within the last 30 days

Returns account activity 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{protocol} | [required] |
**address** | **String** | Account address | [required] |
**from** | Option<**i32**> | Unix Timestamp from where to start |  |
**to** | Option<**i32**> | Unix Timestamp from where to end |  |
**continuation** | Option<**String**> | Continuation token from earlier response |  |
**limit** | Option<**i32**> | Max number of items to return in a response. Defaults to 50k and is capped at 100k.  |  |

### Return type

[**crate::models::Report**](report.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_txs_by_address

> crate::models::TxPage get_txs_by_address(protocol, network, address, assets, from, to, order, continuation, limit)
Transactions Of Address

Gets transactions that an address was involved with, from newest to oldest. This call uses pagination. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{protocol} | [required] |
**address** | **String** | Account address | [required] |
**assets** | Option<**String**> | Comma-separated list of asset paths to filter. If the list is empty, or all elements are empty, this filter has no effect. |  |
**from** | Option<**i32**> | Unix Timestamp from where to start |  |
**to** | Option<**i32**> | Unix Timestamp from where to end |  |
**order** | Option<**String**> | Pagination order |  |
**continuation** | Option<**String**> | Continuation token from earlier response |  |
**limit** | Option<**i32**> | Max number of items to return in a response. Defaults to 25 and is capped at 100.  |  |

### Return type

[**crate::models::TxPage**](tx_page.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_utxoby_account

> crate::models::TxOutputs get_utxoby_account(protocol, network, address, spent, from, to, order, continuation, limit)
Endpoint for getting transaction inputs and outputs for a given account.

Returns transactions outputs following the BTC's UTXO model definition. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{protocol} | [required] |
**address** | **String** | Account address | [required] |
**spent** | Option<**bool**> | Whether the transaction output was spent or not |  |
**from** | Option<**i32**> | Unix Timestamp from where to start |  |
**to** | Option<**i32**> | Unix Timestamp from where to end |  |
**order** | Option<**String**> | Pagination order |  |
**continuation** | Option<**String**> | Continuation token from earlier response |  |
**limit** | Option<**i32**> | Max number of items to return in a response. Defaults to 50k and is capped at 100k.  |  |

### Return type

[**crate::models::TxOutputs**](tx_outputs.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

