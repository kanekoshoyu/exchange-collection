# \FiatApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_fiat_orders_get**](FiatApi.md#sapi_v1_fiat_orders_get) | **GET** /sapi/v1/fiat/orders | Fiat Deposit/Withdraw History (USER_DATA)
[**sapi_v1_fiat_payments_get**](FiatApi.md#sapi_v1_fiat_payments_get) | **GET** /sapi/v1/fiat/payments | Fiat Payments History (USER_DATA)



## sapi_v1_fiat_orders_get

> models::SapiV1FiatOrdersGet200Response sapi_v1_fiat_orders_get(transaction_type, timestamp, signature, begin_time, end_time, page, rows, recv_window)
Fiat Deposit/Withdraw History (USER_DATA)

- If beginTime and endTime are not sent, the recent 30-day data will be returned.  Weight(UID): 90000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_type** | **i32** | * `0` - deposit * `1` - withdraw | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**begin_time** | Option<**i64**> |  |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**page** | Option<**i32**> | Default 1 |  |
**rows** | Option<**i32**> | Default 100, max 500 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1FiatOrdersGet200Response**](_sapi_v1_fiat_orders_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_fiat_payments_get

> models::SapiV1FiatPaymentsGet200Response sapi_v1_fiat_payments_get(transaction_type, timestamp, signature, begin_time, end_time, page, rows, recv_window)
Fiat Payments History (USER_DATA)

- If beginTime and endTime are not sent, the recent 30-day data will be returned.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_type** | **i32** | * `0` - deposit * `1` - withdraw | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**begin_time** | Option<**i64**> |  |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**page** | Option<**i32**> | Default 1 |  |
**rows** | Option<**i32**> | Default 100, max 500 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1FiatPaymentsGet200Response**](_sapi_v1_fiat_payments_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

