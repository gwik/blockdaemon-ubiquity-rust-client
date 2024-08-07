# \BalancesUTXOApi

All URIs are relative to *https://svc.blockdaemon.com/universal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_list_of_balances_by_address**](BalancesUTXOApi.md#get_list_of_balances_by_address) | **GET** /{protocol}/{network}/account/{address} | Get a List of Balances for an Address
[**get_list_of_balances_by_addresses**](BalancesUTXOApi.md#get_list_of_balances_by_addresses) | **POST** /{protocol}/{network}/accounts/ | Get a List of Balances for Multiple Adresses
[**get_report_by_address**](BalancesUTXOApi.md#get_report_by_address) | **GET** /{protocol}/{network}/account/{address}/report | Get a Financial Report for an Address Between a Time Period



## get_list_of_balances_by_address

> Vec<crate::models::Balance> get_list_of_balances_by_address(protocol, network, address, assets)
Get a List of Balances for an Address

Returns a list of account balances by a user-defined account address for the supported currencies.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle, one of:  `algorand`, `avalanche`, `bitcoin`, `bitcoincash`, `dogecoin`, `ethereum`, `fantom`, `litecoin`, `near`, `optimism`, `polkadot`, `polygon`, `solana`, `stellar`, `tezos`, `xrp`.  | [required] |[default to bitcoin]
**network** | **String** | Which network to target. Available networks can be found in the list of supported protocols or with /{protocol}. | [required] |[default to mainnet]
**address** | **String** | The account address of the protocol. | [required] |[default to 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa]
**assets** | Option<**String**> | Comma-separated list of asset paths to filter. If the list is empty, or all elements are empty, this filter has no effect. Find all the asset paths on this [page](https://docs.blockdaemon.com/reference/available-currencies-and-tokens). |  |

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
**protocol** | **String** | Protocol handle, one of:  `algorand`, `avalanche`, `bitcoin`, `bitcoincash`, `dogecoin`, `ethereum`, `fantom`, `litecoin`, `near`, `optimism`, `polkadot`, `polygon`, `solana`, `stellar`, `tezos`, `xrp`.  | [required] |[default to bitcoin]
**network** | **String** | Which network to target. Available networks can be found in the list of supported protocols or with /{protocol}. | [required] |[default to mainnet]
**accounts_obj** | [**AccountsObj**](AccountsObj.md) |  | [required] |
**assets** | Option<**String**> | Comma-separated list of asset paths to filter. If the list is empty, or all elements are empty, this filter has no effect. Find all the asset paths on this [page](https://docs.blockdaemon.com/reference/available-currencies-and-tokens). |  |

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
Get a Financial Report for an Address Between a Time Period

Returns a financial report by a user-defined account address between a time period. Default timescale is within the last 30 days. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle, one of: `algorand`, `fantom`, `polkadot`, `polygon`, `stellar` `tezos`, `xrp`.  | [required] |[default to polkadot]
**network** | **String** | Which network to target. Available networks can be found in the list of supported protocols or with /{protocol}. | [required] |[default to mainnet]
**address** | **String** | The account address of the protocol. | [required] |[default to 12yi4uHFbnSUryffXT7Xq92fbGC3iXvCs3vz9HjVgpb4sBvL]
**from** | Option<**i32**> | The Unix Timestamp from where to start. |  |
**to** | Option<**i32**> | The Unix Timestamp from where to end. |  |
**page_token** | Option<**String**> | The token to retrieve more items in the next request. Use the `next_page_token` returned from the previous response for this parameter. |  |
**page_size** | Option<**i32**> | The max number of items to return in a response. Defaults to 50k and is capped at 100k.  |  |

### Return type

[**crate::models::Report**](report.md)

### Authorization

[apiKeyAuthHeader](../README.md#apiKeyAuthHeader), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

