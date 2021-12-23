# \TransactionsApi

All URIs are relative to *https://ubiquity.api.blockdaemon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**estimate_fee**](TransactionsApi.md#estimate_fee) | **GET** /v2/{platform}/{network}/tx/estimate_fee | Get fee estimate
[**fee_estimate**](TransactionsApi.md#fee_estimate) | **GET** /v1/{platform}/{network}/tx/estimate_fee | Get fee estimate
[**get_tx**](TransactionsApi.md#get_tx) | **GET** /v2/{platform}/{network}/tx/{id} | Transaction By Hash
[**get_txs**](TransactionsApi.md#get_txs) | **GET** /v2/{platform}/{network}/txs | All Transactions
[**tx_send**](TransactionsApi.md#tx_send) | **POST** /v2/{platform}/{network}/tx/send | Submit a signed transaction



## estimate_fee

> String estimate_fee(platform, network, confirmed_within_blocks)
Get fee estimate

Get a fee estimation in decimals from the network. If supported by the platform, the number of blocks used to make the estimation can be customized by the confirmed_within_blocks query parameter. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** | Coin platform handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{platform} | [required] |
**confirmed_within_blocks** | Option<**i32**> | The number of blocks you would like the transaction to be processed within. Lower numbers produce higher fees.  |  |[default to 10]

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fee_estimate

> crate::models::FeeEstimate fee_estimate(platform, network)
Get fee estimate

Get a fee estimation in decimals from the ubiquity fee estimation service. Currently supported for Bitcoin and Ethereum. Endpoint will return 3 fee estimations fast, medium and slow 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** | Coin platform handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{platform} | [required] |

### Return type

[**crate::models::FeeEstimate**](fee_estimate.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tx

> crate::models::Tx get_tx(platform, network, id)
Transaction By Hash

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** | Coin platform handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{platform} | [required] |
**id** | **String** | Transaction ID/Hash | [required] |

### Return type

[**crate::models::Tx**](tx.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_txs

> crate::models::TxPage get_txs(platform, network, order, continuation, limit, assets)
All Transactions

Get all transactions on the platform, starting with the lastest one. Each call returns a slice of the entire list. Use the returned continuation token to get the next part.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** | Coin platform handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{platform} | [required] |
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


## tx_send

> crate::models::TxReceipt tx_send(platform, network, signed_tx)
Submit a signed transaction

Submit a signed transaction to the network.  **Note**: A successful transaction may still be rejected on chain or not processed due to a too low fee. You can monitor successful transactions through Ubiquity websockets. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** | Coin platform handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{platform} | [required] |
**signed_tx** | [**SignedTx**](SignedTx.md) |  | [required] |

### Return type

[**crate::models::TxReceipt**](tx_receipt.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

