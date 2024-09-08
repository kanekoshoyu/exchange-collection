# \C2CApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_c2c_order_match_list_user_order_history_get**](C2CApi.md#sapi_v1_c2c_order_match_list_user_order_history_get) | **GET** /sapi/v1/c2c/orderMatch/listUserOrderHistory | Get C2C Trade History (USER_DATA)



## sapi_v1_c2c_order_match_list_user_order_history_get

> models::SapiV1C2cOrderMatchListUserOrderHistoryGet200Response sapi_v1_c2c_order_match_list_user_order_history_get(trade_type, timestamp, signature, start_timestamp, end_timestamp, page, rows, recv_window)
Get C2C Trade History (USER_DATA)

- If startTimestamp and endTimestamp are not sent, the recent 30-day data will be returned. - The max interval between startTimestamp and endTimestamp is 30 days.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trade_type** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**start_timestamp** | Option<**i64**> | UTC timestamp in ms |  |
**end_timestamp** | Option<**i64**> | UTC timestamp in ms |  |
**page** | Option<**i32**> | Default 1 |  |
**rows** | Option<**i32**> | default 100, max 100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1C2cOrderMatchListUserOrderHistoryGet200Response**](_sapi_v1_c2c_orderMatch_listUserOrderHistory_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

