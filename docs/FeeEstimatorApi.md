# \FeeEstimatorApi

All URIs are relative to *https://svc.blockdaemon.com/universal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_fee_estimate**](FeeEstimatorApi.md#get_fee_estimate) | **GET** /{protocol}/{network}/tx/estimate_fee | Get the Fee Estimation



## get_fee_estimate

> crate::models::InlineResponse200 get_fee_estimate(protocol, network)
Get the Fee Estimation

Returns fee estimation in decimals.This endpoint will return 3 fee estimations: fast, medium and slow. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle, one of: `avalanche`, `bitcoin`, `bitcoincash`, `ethereum`, `fantom`, `litecoin`, `polkadot`, `polygon` and `solana`.  | [required] |[default to bitcoin]
**network** | **String** | Which network to target. Available networks can be found in the list of supported protocols or with /{protocol}. | [required] |[default to mainnet]

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

[apiKeyAuthHeader](../README.md#apiKeyAuthHeader), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

