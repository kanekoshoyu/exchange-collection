# \RebateApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_rebate_tax_query_get**](RebateApi.md#sapi_v1_rebate_tax_query_get) | **GET** /sapi/v1/rebate/taxQuery | Get Spot Rebate History Records (USER_DATA)



## sapi_v1_rebate_tax_query_get

> models::SapiV1RebateTaxQueryGet200Response sapi_v1_rebate_tax_query_get(timestamp, signature, start_time, end_time, page, recv_window)
Get Spot Rebate History Records (USER_DATA)

- The max interval between startTime and endTime is 90 days. - If startTime and endTime are not sent, the recent 7 days' data will be returned. - The earliest startTime is supported on June 10, 2020  Weight(UID): 3000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**page** | Option<**i32**> | default 1 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1RebateTaxQueryGet200Response**](_sapi_v1_rebate_taxQuery_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

