# \GasEstimatorApi

All URIs are relative to *https://svc.blockdaemon.com/universal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_gas_fee_estimate**](GasEstimatorApi.md#get_gas_fee_estimate) | **GET** /{protocol}/{network}/tx/estimate_fee | Get the Gas Fee Estimation



## get_gas_fee_estimate

> crate::models::FeeEstimate get_gas_fee_estimate(protocol, network)
Get the Gas Fee Estimation

Return gas fee estimation in decimals.  Currently supported for **Bitcoin**, **Bitcoincash**, **Ethereum** and **Litecoin**. Endpoint will return 3 gas fee estimations: fast, medium and slow. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle, one of: `bitcoin`, `bitcoincash`, `ethereum`, `litecoin`.  | [required] |[default to bitcoin]
**network** | **String** | Which network to target. Available networks can be found in the list of supported protocols or with /{protocol}. | [required] |[default to mainnet]

### Return type

[**crate::models::FeeEstimate**](fee_estimate.md)

### Authorization

[apiKeyAuthHeader](../README.md#apiKeyAuthHeader), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

