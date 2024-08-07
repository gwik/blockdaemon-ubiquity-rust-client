# Block

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**number** | Option<**i64**> | The block number. | [optional]
**id** | Option<**String**> | The block hash. | [optional]
**parent_id** | Option<**String**> | The parent block hash. | [optional]
**date** | Option<**i64**> | The block date in unix timestamp format. | [optional]
**num_txs** | Option<**i32**> | The amount of transaction in the block. | [optional]
**txs** | Option<[**Vec<crate::models::Tx>**](tx.md)> | A list of normalized transactions presented in the block (not filtered or unknown model). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


