# ReportField

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**protocol** | **String** | The protocol the address relates to. | 
**address** | **String** | The wallet/account the transaction occurred | 
**currency** | **String** | The currency symbol. | 
**event_id** | **String** | The ID of the event within a transaction. | 
**block** | **i64** | The block number the transaction occurred on. | 
**timestamp** | **i32** | The unix timestamp when the transaction was added to a block. | 
**hash** | **String** | The transaction ID. | 
**action** | **String** | The action type e.g. Transfer, Deposit, Staking Reward etc.. | 
**value** | **String** | The amount of currency involved in the transaction (smallest unit). | 
**fee** | **String** | How much was charged as a fee for processing the transaction. | 
**sender_address** | **String** | The address where the funds originated. | 
**decimals** | **i32** | The number of decimals in one coin, used to convert smallest unit to 1 whole coin if needed. | 
**meta** | Option<[**crate::models::ReportFieldMeta**](report_field_meta.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


