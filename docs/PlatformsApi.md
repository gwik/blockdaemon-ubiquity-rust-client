# \PlatformsApi

All URIs are relative to *https://ubiquity.api.blockdaemon.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_platform**](PlatformsApi.md#get_platform) | **GET** /{platform}/{network} | Platform Info
[**get_platforms**](PlatformsApi.md#get_platforms) | **GET** / | Platforms overview



## get_platform

> crate::models::PlatformDetail get_platform(platform, network)
Platform Info

Provides information about supported endpoints and generic platform information. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** | Coin platform handle | [required] |
**network** | **String** | Which network to target. Available networks can be found with /{platform} | [required] |

### Return type

[**crate::models::PlatformDetail**](platform_detail.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_platforms

> crate::models::PlatformsOverview get_platforms()
Platforms overview

Provides a list of supported platforms and networks. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PlatformsOverview**](platforms_overview.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

