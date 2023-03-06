# \AccountsApi

All URIs are relative to *https://svc.blockdaemon.com/universal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_list_of_balances_by_address**](AccountsApi.md#get_list_of_balances_by_address) | **GET** /{protocol}/{network}/account/{address} | Get a List of Balances for an Address
[**get_list_of_balances_by_addresses**](AccountsApi.md#get_list_of_balances_by_addresses) | **POST** /{protocol}/{network}/accounts | Get a List of Balances for Multiple Adresses
[**get_report_by_address**](AccountsApi.md#get_report_by_address) | **GET** /{protocol}/{network}/account/{address}/report | Get a Financial Report for an Address between a Time Period
[**get_txs_by_address**](AccountsApi.md#get_txs_by_address) | **GET** /{protocol}/{network}/account/{address}/txs | Get a List of Transactions
[**get_utxoby_account**](AccountsApi.md#get_utxoby_account) | **GET** /{protocol}/{network}/account/{address}/utxo | Get a List of Transaction Inputs and Outputs



## get_list_of_balances_by_address

> Vec<crate::models::Balance> get_list_of_balances_by_address(protocol, network, address, assets)
Get a List of Balances for an Address

Returns a list of account balances by a user-defined account address for the supported currencies.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle, one of:  `algorand`, `bitcoin`, `bitcoincash`, `dogecoin`, `ethereum`, `litecoin`, `near`, `oasis`, `optimism`, `polkadot`, `polygon`, `solana`, `stellar`, `tezos`, `xrp`.  | [required] |[default to bitcoin]
**network** | **String** | Which network to target. Available networks can be found in the list of supported protocols or with /{protocol}. | [required] |[default to mainnet]
**address** | **String** | The account address of the protocol. | [required] |[default to 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa]
**assets** | Option<**String**> | Comma-separated list of asset paths to filter. If the list is empty, or all elements are empty, this filter has no effect. |  |

### Return type

[**Vec<crate::models::Balance>**](balance.md)

### Authorization

[apiKeyAuthHeader](../README.md#apiKeyAuthHeader), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_list_of_balances_by_addresses

> ::std::collections::HashMap<String, Vec<crate::models::Balance>> get_list_of_balances_by_addresses(protocol, network, accounts_obj, assets)
Get a List of Balances for Multiple Adresses

Returns a list of account balances by a user-defined list of account addresses for the supported currencies. The maximum of account addresses allowed in the filter is 10. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle, one of:  `algorand`, `bitcoin`, `bitcoincash`, `dogecoin`, `ethereum`, `litecoin`, `near`, `oasis`, `optimism`, `polkadot`, `polygon`, `solana`, `stellar`, `tezos`, `xrp`.  | [required] |[default to bitcoin]
**network** | **String** | Which network to target. Available networks can be found in the list of supported protocols or with /{protocol}. | [required] |[default to mainnet]
**accounts_obj** | [**AccountsObj**](AccountsObj.md) |  | [required] |
**assets** | Option<**String**> | Comma-separated list of asset paths to filter. If the list is empty, or all elements are empty, this filter has no effect. |  |

### Return type

[**::std::collections::HashMap<String, Vec<crate::models::Balance>>**](array.md)

### Authorization

[apiKeyAuthHeader](../README.md#apiKeyAuthHeader), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_report_by_address

> crate::models::Report get_report_by_address(protocol, network, address, from, to, page_token, page_size)
Get a Financial Report for an Address between a Time Period

Returns a financial report by a user-defined account address between a time period. Default timescale is within the last 30 days. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle, one of: `algorand`, `polkadot`, `tezos`.  | [required] |[default to polkadot]
**network** | **String** | Which network to target. Available networks can be found in the list of supported protocols or with /{protocol}. | [required] |[default to mainnet]
**address** | **String** | The account address of the protocol. | [required] |[default to 12yi4uHFbnSUryffXT7Xq92fbGC3iXvCs3vz9HjVgpb4sBvL]
**from** | Option<**i32**> | Unix Timestamp from where to start |  |
**to** | Option<**i32**> | Unix Timestamp from where to end |  |
**page_token** | Option<**String**> | Continuation token from earlier response |  |
**page_size** | Option<**i32**> | Max number of items to return in a response. Defaults to 50k and is capped at 100k.  |  |

### Return type

[**crate::models::Report**](report.md)

### Authorization

[apiKeyAuthHeader](../README.md#apiKeyAuthHeader), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_txs_by_address

> crate::models::TxPage get_txs_by_address(protocol, network, address, assets, from, to, order, page_token, page_size)
Get a List of Transactions

Returns the transactions that an address was involved with, from newest to oldest.  The response is paginated: use the returned `next_page_token` token as a query parameter to get the next page. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle, one of:  `algorand`, `bitcoin`, `bitcoincash`, `dogecoin`, `ethereum`, `litecoin`, `near`, `oasis`, `optimism`, `polkadot`, `polygon`, `solana`, `stellar`, `tezos`, `xrp`.  | [required] |[default to bitcoin]
**network** | **String** | Which network to target. Available networks can be found in the list of supported protocols or with /{protocol}. | [required] |[default to mainnet]
**address** | **String** | The account address of the protocol. | [required] |[default to 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa]
**assets** | Option<**String**> | Comma-separated list of asset paths to filter. If the list is empty, or all elements are empty, this filter has no effect. |  |
**from** | Option<**i32**> | Unix Timestamp from where to start |  |
**to** | Option<**i32**> | Unix Timestamp from where to end |  |
**order** | Option<**String**> | The pagination order. |  |[default to desc]
**page_token** | Option<**String**> | The continuation token from earlier response. |  |
**page_size** | Option<**i32**> | Max number of items to return in a response. Defaults to 25 and is capped at 100.  |  |

### Return type

[**crate::models::TxPage**](tx_page.md)

### Authorization

[apiKeyAuthHeader](../README.md#apiKeyAuthHeader), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_utxoby_account

> crate::models::TxOutputs get_utxoby_account(protocol, network, address, spent, from, to, order, page_token, page_size)
Get a List of Transaction Inputs and Outputs

Returns the transaction inputs and outputs following the BTC's UTXO model definition by a user-definied account address.  The response is paginated: use the returned `next_page_token` token as a query parameter to get the next page. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle, one of: `bitcoin`, `bitcoincash`, `dogecoin`, `litecoin`.  | [required] |[default to bitcoin]
**network** | **String** | Which network to target. Available networks can be found in the list of supported protocols or with /{protocol}. | [required] |[default to mainnet]
**address** | **String** | The account address of the protocol. | [required] |[default to 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa]
**spent** | Option<**bool**> | Whether the transaction output was spent or not |  |
**from** | Option<**i32**> | Unix Timestamp from where to start |  |
**to** | Option<**i32**> | Unix Timestamp from where to end |  |
**order** | Option<**String**> | The pagination order. |  |[default to desc]
**page_token** | Option<**String**> | The continuation token from earlier response. |  |
**page_size** | Option<**i32**> | Max number of items to return in a response. Defaults to 50k and is capped at 100k.  |  |

### Return type

[**crate::models::TxOutputs**](tx_outputs.md)

### Authorization

[apiKeyAuthHeader](../README.md#apiKeyAuthHeader), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

