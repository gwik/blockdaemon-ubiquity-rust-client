# TxCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from** | **String** | The source UTXO or account ID for the originating funds. | 
**to** | [**Vec<crate::models::TxDestination>**](tx_destination.md) | A list of recipients. | 
**contract** | Option<[**crate::models::TxCreateContract**](tx_create_contract.md)> |  | [optional]
**index** | Option<**i64**> | The UTXO index or the account Nonce. Required for Bitcoin, Bitcoincash, and Litecoin. | [optional]
**fee** | Option<**String**> | The fee you are willing to pay for the transaction. For Ethereum and Polygon see 'protocol.ethereum' and 'protocol.polygon'. | [optional]
**protocol** | Option<[**crate::models::TxCreateProtocol**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


