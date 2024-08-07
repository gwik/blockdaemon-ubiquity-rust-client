# Tx

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique transaction identifier. | [optional]
**block_id** | Option<**String**> | Block hash if mined, otherwise omitted. | [optional]
**date** | Option<**i64**> | The transaction creation unix timestamp. | [optional]
**status** | Option<**String**> | Result status of the transaction. | [optional]
**num_events** | Option<**i32**> | List of transaction events. | [optional]
**meta** | Option<[**serde_json::Value**](.md)> | Protocol specific data that doesn't fit into a standard model. | [optional]
**block_number** | Option<**i32**> | Block number if mined, otherwise omitted. | [optional]
**confirmations** | Option<**i32**> | Total transaction confirmations. | [optional]
**events** | Option<[**Vec<crate::models::Event>**](event.md)> |  | [optional]
**assets** | Option<**Vec<String>**> | List of moved assets by asset path. Find all the asset paths on this [page](https://docs.blockdaemon.com/reference/available-currencies-and-tokens). | [optional]
**nonce** | Option<**i32**> | The nonce of the transaction. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


