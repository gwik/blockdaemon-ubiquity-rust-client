# Block

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**number** | Option<**i64**> | Block number | [optional]
**id** | Option<**String**> | Block hash | [optional]
**parent_id** | Option<**String**> | Parent block hash | [optional]
**date** | Option<**i64**> | Block date in unix timestamp format | [optional]
**num_txs** | Option<**i32**> | Amount of transaction in the block | [optional]
**txs** | Option<[**Vec<crate::models::Tx>**](tx.md)> | List of normalized transactions presented in the block (not filtered or unknown model) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


