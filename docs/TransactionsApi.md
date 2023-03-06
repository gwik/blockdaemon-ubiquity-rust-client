# \TransactionsApi

All URIs are relative to *https://svc.blockdaemon.com/universal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_tx_by_hash**](TransactionsApi.md#get_tx_by_hash) | **GET** /{protocol}/{network}/tx/{hash} | Get a Transaction
[**get_tx_confirmations**](TransactionsApi.md#get_tx_confirmations) | **GET** /{protocol}/{network}/tx/{hash}/confirmations | Get the Transaction Confirmations
[**get_tx_output_by_hash_and_index**](TransactionsApi.md#get_tx_output_by_hash_and_index) | **GET** /{protocol}/{network}/tx/{hash}/{index} | Get a Transaction Output by Hash and Index
[**get_txs**](TransactionsApi.md#get_txs) | **GET** /{protocol}/{network}/txs | Get a List of Transactions
[**tx_create**](TransactionsApi.md#tx_create) | **POST** /{protocol}/{network}/tx/create | Create an unsigned transaction
[**tx_send**](TransactionsApi.md#tx_send) | **POST** /{protocol}/{network}/tx/send | Submit a Signed Transaction



## get_tx_by_hash

> crate::models::Tx get_tx_by_hash(protocol, network, hash)
Get a Transaction

Returns a transaction by a user-defined transaction hash.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle, one of:  `algorand`, `bitcoin`, `bitcoincash`, `dogecoin`, `ethereum`, `litecoin`, `near`, `oasis`, `optimism`, `polkadot`, `polygon`, `solana`, `stellar`, `tezos`, `xrp`.  | [required] |[default to bitcoin]
**network** | **String** | Which network to target. Available networks can be found in the list of supported protocols or with /{protocol}. | [required] |[default to mainnet]
**hash** | **String** | The transaction Hash. | [required] |[default to 71d4f3412ec11128bbd9ce988d5bff2ec3bb6ea3953c8faf189d88ae49de9f7a]

### Return type

[**crate::models::Tx**](tx.md)

### Authorization

[apiKeyAuthHeader](../README.md#apiKeyAuthHeader), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tx_confirmations

> crate::models::TxConfirmation get_tx_confirmations(protocol, network, hash)
Get the Transaction Confirmations

Returns the number of transaction confirmations by a user-defined transaction hash. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle, one of:  `algorand`, `bitcoin`, `bitcoincash`, `dogecoin`, `ethereum`, `litecoin`, `near`, `oasis`, `optimism`, `polkadot`, `polygon`, `solana`, `stellar`, `tezos`, `xrp`.  | [required] |[default to bitcoin]
**network** | **String** | Which network to target. Available networks can be found in the list of supported protocols or with /{protocol}. | [required] |[default to mainnet]
**hash** | **String** | The transaction Hash. | [required] |[default to 71d4f3412ec11128bbd9ce988d5bff2ec3bb6ea3953c8faf189d88ae49de9f7a]

### Return type

[**crate::models::TxConfirmation**](tx_confirmation.md)

### Authorization

[apiKeyAuthHeader](../README.md#apiKeyAuthHeader), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tx_output_by_hash_and_index

> crate::models::TxOutput get_tx_output_by_hash_and_index(protocol, network, hash, index)
Get a Transaction Output by Hash and Index

Get a transaction output by a user-defined transaction hash and the transaction output index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle, one of: `bitcoin`, `bitcoincash`, `dogecoin`, `litecoin`.  | [required] |[default to bitcoin]
**network** | **String** | Which network to target. Available networks can be found in the list of supported protocols or with /{protocol}. | [required] |[default to mainnet]
**hash** | **String** | The transaction Hash. | [required] |[default to 71d4f3412ec11128bbd9ce988d5bff2ec3bb6ea3953c8faf189d88ae49de9f7a]
**index** | **i32** | Transaction output index. | [required] |[default to 0]

### Return type

[**crate::models::TxOutput**](tx_output.md)

### Authorization

[apiKeyAuthHeader](../README.md#apiKeyAuthHeader), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_txs

> crate::models::TxPage get_txs(protocol, network, block_hash, assets, order, page_token, page_size)
Get a List of Transactions

Get a List of transactions, starting with the lastest one. Each call returns an array of the entire list.  The response is paginated: use the returned `next_page_token` token as a query parameter to get the next page. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle, one of:  `algorand`, `bitcoin`, `bitcoincash`, `dogecoin`, `ethereum`, `litecoin`, `near`, `oasis`, `optimism`, `polkadot`, `polygon`, `solana`, `stellar`, `tezos`, `xrp`.  | [required] |[default to bitcoin]
**network** | **String** | Which network to target. Available networks can be found in the list of supported protocols or with /{protocol}. | [required] |[default to mainnet]
**block_hash** | Option<**String**> | Filter by block hash. You can specify only one block hash at a time. |  |
**assets** | Option<**String**> | Comma-separated list of asset paths to filter. If the list is empty, or all elements are empty, this filter has no effect. |  |
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


## tx_create

> crate::models::UnsignedTx tx_create(protocol, network, tx_create)
Create an unsigned transaction

Creates an unsigned transaction for BTC, DOT and ETH.  **Note** that Ethereum currently only supports singular transaction destinations 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle, one of:  `algorand`, `bitcoin`, `bitcoincash`, `dogecoin`, `ethereum`, `litecoin`, `near`, `oasis`, `optimism`, `polkadot`, `polygon`, `solana`, `stellar`, `tezos`, `xrp`.  | [required] |[default to bitcoin]
**network** | **String** | Which network to target. Available networks can be found in the list of supported protocols or with /{protocol}. | [required] |[default to mainnet]
**tx_create** | [**TxCreate**](TxCreate.md) |  | [required] |

### Return type

[**crate::models::UnsignedTx**](unsigned_tx.md)

### Authorization

[apiKeyAuthHeader](../README.md#apiKeyAuthHeader), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tx_send

> crate::models::TxReceipt tx_send(protocol, network, signed_tx)
Submit a Signed Transaction

Submit a signed transaction to the network.  **Note**: A successful transaction may still be rejected on chain or not processed due to a too low fee. You can monitor successful transactions through Universal websockets. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle, one of:  `algorand`, `bitcoin`, `bitcoincash`, `dogecoin`, `ethereum`, `litecoin`, `near`, `oasis`, `optimism`, `polkadot`, `polygon`, `solana`, `stellar`, `tezos`, `xrp`.  | [required] |[default to bitcoin]
**network** | **String** | Which network to target. Available networks can be found in the list of supported protocols or with /{protocol}. | [required] |[default to mainnet]
**signed_tx** | [**SignedTx**](SignedTx.md) |  | [required] |

### Return type

[**crate::models::TxReceipt**](tx_receipt.md)

### Authorization

[apiKeyAuthHeader](../README.md#apiKeyAuthHeader), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

