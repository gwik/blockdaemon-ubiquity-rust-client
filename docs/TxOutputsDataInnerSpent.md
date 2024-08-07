# TxOutputsDataInnerSpent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**index** | Option<**i32**> | The output index within a given transaction. | [optional]
**tx_id** | Option<**String**> | The transaction identifier. | [optional]
**date** | Option<**i32**> | The transaction creation unix timestamp. | [optional]
**block_id** | Option<**String**> | The hash identifier of the block which the transaction was mined. | [optional]
**block_number** | Option<**i32**> | The number of the block which the transaction was mined. | [optional]
**confirmations** | Option<**i32**> | The number of confirmations the transaction took in order to be mined. | [optional]
**meta** | Option<[**crate::models::TxOutputResponseMeta**](tx_output_response_meta.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


