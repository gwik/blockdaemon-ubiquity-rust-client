# Tx

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique transaction identifier | [optional]
**date** | Option<**i64**> | The transaction creation unix timestamp | [optional]
**block_id** | Option<**String**> | Block hash if mined, otherwise omitted. | [optional]
**status** | Option<**String**> | Result status of the transaction | [optional]
**assets** | Option<**Vec<String>**> | List of moved assets by asset path | [optional]
**nonce** | Option<**i32**> |  | [optional]
**num_events** | Option<**i32**> | List of transaction events | [optional]
**meta** | Option<[**serde_json::Value**](.md)> | Protocol specific data that doesn't fit into a standard model | [optional]
**events** | Option<[**Vec<crate::models::Event>**](event.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


