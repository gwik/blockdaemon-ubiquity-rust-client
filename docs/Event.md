# Event

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The event identifier. | [optional]
**transaction_id** | Option<**String**> | The transaction identifer this event is presented. | [optional]
**_type** | Option<**String**> | The event type. | [optional]
**denomination** | Option<**String**> | The Symbol of currency, can be natibe currency or token currency. | [optional]
**destination** | Option<**String**> | The destination address of the event. | [optional]
**source** | Option<**String**> | The source address of the event. | [optional]
**meta** | Option<[**crate::models::EventMeta**](event_meta.md)> |  | [optional]
**date** | Option<**i64**> | The event date in unix timestamp format. | [optional]
**amount** | Option<[**bigdecimal::BigDecimal**](bigdecimal::BigDecimal.md)> | The coin amount transfered in the event. | [optional]
**decimals** | Option<**i32**> | The coin amount transfered in the event. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


