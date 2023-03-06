# Event

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | Option<[**bigdecimal::BigDecimal**](bigdecimal::BigDecimal.md)> | Coin amount transfered in the event | [optional]
**date** | Option<**i64**> | Event date in unix timestamp format | [optional]
**decimals** | Option<**i32**> | Coin amount transfered in the event | [optional]
**denomination** | Option<**String**> | Symbol of currency, can be natibe currency or token currency | [optional]
**destination** | Option<**String**> | Destination address of the event | [optional]
**id** | Option<**String**> | Event identifier | [optional]
**meta** | Option<[**serde_json::Value**](.md)> | Protocol specific data that doesn't fit into a standard model. | [optional]
**source** | Option<**String**> | Source address of the event | [optional]
**transaction_id** | Option<**String**> | Transaction identifer this event is presented | [optional]
**_type** | Option<**String**> | Event type | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


