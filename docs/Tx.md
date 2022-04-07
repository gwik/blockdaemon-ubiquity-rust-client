# Tx

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique transaction identifier | [optional]
**date** | Option<**i64**> | Unix timestamp | [optional]
**block_id** | Option<**String**> | ID of block if mined, otherwise omitted. | [optional]
**status** | Option<**String**> | Result status of the transaction. | [optional]
**nonce** | Option<**i32**> |  | [optional]
**num_events** | Option<**i32**> |  | [optional]
**meta** | Option<[**serde_json::Value**](.md)> |  | [optional]
**events** | Option<[**Vec<crate::models::Event>**](Event.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


