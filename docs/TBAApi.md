# \TBAApi

All URIs are relative to *https://www.thebluealliance.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_status**](TBAApi.md#get_status) | **get** /status | 



## get_status

> crate::models::ApiStatus get_status(if_modified_since)


Returns API status, and TBA status information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**if_modified_since** | Option<**String**> | Value of the `Last-Modified` header in the most recently cached response by the client. |  |

### Return type

[**crate::models::ApiStatus**](API_Status.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

