# \ProtocolsApi

All URIs are relative to *https://ubiquity.api.blockdaemon.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_protocol_endpoints**](ProtocolsApi.md#get_protocol_endpoints) | **GET** /{protocol}/{network} | Protocol Info
[**get_protocols_list**](ProtocolsApi.md#get_protocols_list) | **GET** / | Protocols overview



## get_protocol_endpoints

> crate::models::ProtocolDetail get_protocol_endpoints(protocol, network)
Protocol Info

Provides information about supported endpoints and generic protocol information. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | **String** | Protocol handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{protocol} | [required] |

### Return type

[**crate::models::ProtocolDetail**](protocol_detail.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_protocols_list

> crate::models::ProtocolsOverview get_protocols_list()
Protocols overview

Provides a list of supported protocols and networks. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ProtocolsOverview**](protocols_overview.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

