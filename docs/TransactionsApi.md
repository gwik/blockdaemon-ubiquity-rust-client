# \TransactionsApi

All URIs are relative to *https://ubiquity.api.blockdaemon.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fee_estimate**](TransactionsApi.md#fee_estimate) | **get** /{platform}/{network}/tx/estimate_fee | Get fee estimate
[**get_tx**](TransactionsApi.md#get_tx) | **get** /{platform}/{network}/tx/{id} | Transaction By Hash
[**get_tx_by_hash_and_index**](TransactionsApi.md#get_tx_by_hash_and_index) | **get** /{platform}/{network}/tx/{id}/{index} | Transaction output by hash and index
[**get_tx_confirmations**](TransactionsApi.md#get_tx_confirmations) | **get** /{platform}/{network}/tx/{id}/confirmations | Transaction confirmations By Hash
[**get_txs**](TransactionsApi.md#get_txs) | **get** /{platform}/{network}/txs | Latest transactions of a protocol
[**tx_send**](TransactionsApi.md#tx_send) | **post** /{platform}/{network}/tx/send | Submit a signed transaction



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


## get_tx_by_hash_and_index

> crate::models::TxOutput get_tx_by_hash_and_index(platform, network, id, index)
Transaction output by hash and index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** | Coin platform handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{platform} | [required] |
**id** | **String** | Transaction ID/Hash | [required] |
**index** | **i32** | Transaction output index | [required] |

### Return type

[**crate::models::TxOutput**](tx_output.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tx_confirmations

> crate::models::TxConfirmation get_tx_confirmations(platform, network, id)
Transaction confirmations By Hash

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** | Coin platform handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{platform} | [required] |
**id** | **String** | Transaction ID/Hash | [required] |

### Return type

[**crate::models::TxConfirmation**](tx_confirmation.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_txs

> crate::models::TxPage get_txs(platform, network, order, continuation, limit, assets)
Latest transactions of a protocol

Gets transactions from oldest to newest. This call uses pagination. 

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

