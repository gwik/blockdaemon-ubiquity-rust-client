# TxOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**String**> | Result status of the transaction output. | [optional]
**is_spent** | Option<**bool**> | If the transaction output was spent or not, if the value is true the `spent` transaction object will be presented | [optional]
**value** | Option<**i64**> | Amount of transaction output | [optional]
**mined** | Option<[**crate::models::TxMinify**](tx_minify.md)> |  | [optional]
**spent** | Option<[**crate::models::TxMinify**](tx_minify.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


