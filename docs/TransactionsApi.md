# \TransactionsApi

All URIs are relative to *https://ubiquity.api.blockdaemon.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fee_estimate**](TransactionsApi.md#fee_estimate) | **GET** /{protocol}/{network}/tx/estimate_fee | Get fee estimate
[**get_tx**](TransactionsApi.md#get_tx) | **GET** /{protocol}/{network}/tx/{id} | Transaction By Hash
[**get_tx_by_hash_and_index**](TransactionsApi.md#get_tx_by_hash_and_index) | **GET** /{protocol}/{network}/tx/{id}/{index} | Transaction output by hash and index
[**get_tx_confirmations**](TransactionsApi.md#get_tx_confirmations) | **GET** /{protocol}/{network}/tx/{id}/confirmations | Transaction confirmations By Hash
[**get_txs**](TransactionsApi.md#get_txs) | **GET** /{protocol}/{network}/txs | All Transactions
[**v1_tx_send**](TransactionsApi.md#v1_tx_send) | **POST** /{protocol}/{network}/tx/send | Submit a signed transaction



## fee_estimate

> crate::models::FeeEstimate fee_estimate(protocol, network)
Get fee estimate

Get a fee estimation in decimals from the ubiquity fee estimation service. Currently supported for Bitcoin and Ethereum. Endpoint will return 3 fee estimations fast, medium and slow 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{protocol} | [required] |

### Return type

[**crate::models::FeeEstimate**](fee_estimate.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tx

> crate::models::Tx get_tx(protocol, network, id)
Transaction By Hash

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{protocol} | [required] |
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

> crate::models::TxOutput get_tx_by_hash_and_index(protocol, network, id, index)
Transaction output by hash and index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{protocol} | [required] |
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

> crate::models::TxConfirmation get_tx_confirmations(protocol, network, id)
Transaction confirmations By Hash

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{protocol} | [required] |
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

> crate::models::TxPage get_txs(protocol, network, block_id, assets, order, continuation, limit)
All Transactions

Get all transactions on the protocol, starting with the lastest one. Each call returns a slice of the entire list. Use the returned continuation token to get the next part.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{protocol} | [required] |
**block_id** | Option<**String**> | Filter by block hash. You can specify only one block hash at a time. |  |
**assets** | Option<**String**> | Comma-separated list of asset paths to filter. If the list is empty, or all elements are empty, this filter has no effect. |  |
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


## v1_tx_send

> crate::models::TxReceipt v1_tx_send(protocol, network, signed_tx)
Submit a signed transaction

Submit a signed transaction to the network.  **Note**: A successful transaction may still be rejected on chain or not processed due to a too low fee. You can monitor successful transactions through Ubiquity websockets. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{protocol} | [required] |
**signed_tx** | [**SignedTx**](SignedTx.md) |  | [required] |

### Return type

[**crate::models::TxReceipt**](tx_receipt.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

