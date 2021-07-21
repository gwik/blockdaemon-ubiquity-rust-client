# Tx

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique transaction identifier | [optional]
**slip44** | Option<**i32**> | SLIP-44 coin ID | [optional]
**addresses** | Option<**Vec<String>**> | List of involved addresses (excluding contracts) | [optional]
**assets** | Option<**Vec<String>**> | List of moved assets by asset path | [optional]
**date** | Option<**i64**> | Unix timestamp | [optional]
**height** | Option<**i64**> | Number of block if mined, otherwise omitted. | [optional]
**block_id** | Option<**String**> | ID of block if mined, otherwise omitted. | [optional]
**status** | Option<**String**> | Result status of the transaction. | [optional]
**tags** | Option<**Vec<String>**> | List of tags for this transaction | [optional]
**operations** | Option<[**::std::collections::HashMap<String, crate::models::Operation>**](operation.md)> | Operations in this transaction (opaque keys). | [optional]
**effects** | Option<[**::std::collections::HashMap<String, crate::models::Effect>**](effect.md)> | Effects by address, if supported | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


