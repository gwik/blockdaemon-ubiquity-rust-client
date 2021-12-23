# \PlatformsApi

All URIs are relative to *https://ubiquity.api.blockdaemon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_platform**](PlatformsApi.md#get_platform) | **GET** /v2/{platform}/{network} | Platform Info
[**get_platform_endpoints**](PlatformsApi.md#get_platform_endpoints) | **GET** /v1/{platform}/{network}/ | Platform Info
[**get_platforms**](PlatformsApi.md#get_platforms) | **GET** /v2/ | Platforms overview
[**get_platforms_list**](PlatformsApi.md#get_platforms_list) | **GET** /v1/ | Platforms overview



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


## get_platform_endpoints

> crate::models::PlatformDetail get_platform_endpoints(platform, network)
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


## get_platforms_list

> crate::models::PlatformsOverview get_platforms_list()
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

