# \ProtocolAndEndpointSupportApi

All URIs are relative to *https://svc.blockdaemon.com/universal/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_protocol_endpoints**](ProtocolAndEndpointSupportApi.md#get_protocol_endpoints) | **GET** /{protocol}/{network} | Get the Protocol Info
[**get_protocols_list**](ProtocolAndEndpointSupportApi.md#get_protocols_list) | **GET** / | Get the Protocols Overview



## get_protocol_endpoints

> crate::models::ProtocolDetail get_protocol_endpoints(protocol, network)
Get the Protocol Info

Provides information about supported endpoints and generic protocol information. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle, one of:  `algorand`, `avalanche`, `bitcoin`, `bitcoincash`, `dogecoin`, `ethereum`, `fantom`, `litecoin`, `near`, `optimism`, `polkadot`, `polygon`, `solana`, `stellar`, `tezos`, `xrp`.  | [required] |[default to bitcoin]
**network** | **String** | Which network to target. Available networks can be found in the list of supported protocols or with /{protocol}. | [required] |[default to mainnet]

### Return type

[**crate::models::ProtocolDetail**](protocol_detail.md)

### Authorization

[apiKeyAuthHeader](../README.md#apiKeyAuthHeader), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_protocols_list

> crate::models::ProtocolsOverview get_protocols_list()
Get the Protocols Overview

Provides a list of supported protocols and networks. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ProtocolsOverview**](protocols_overview.md)

### Authorization

[apiKeyAuthHeader](../README.md#apiKeyAuthHeader), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

