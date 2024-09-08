# \FuturesApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_futures_hist_data_link_get**](FuturesApi.md#sapi_v1_futures_hist_data_link_get) | **GET** /sapi/v1/futures/histDataLink | Get Future TickLevel Orderbook Historical Data Download Link (USER_DATA)
[**sapi_v1_futures_transfer_get**](FuturesApi.md#sapi_v1_futures_transfer_get) | **GET** /sapi/v1/futures/transfer | Get Future Account Transaction History List (USER_DATA)
[**sapi_v1_futures_transfer_post**](FuturesApi.md#sapi_v1_futures_transfer_post) | **POST** /sapi/v1/futures/transfer | New Future Account Transfer (USER_DATA)



## sapi_v1_futures_hist_data_link_get

> models::SapiV1FuturesHistDataLinkGet200Response sapi_v1_futures_hist_data_link_get(symbol, data_type, timestamp, signature, start_time, end_time, recv_window)
Get Future TickLevel Orderbook Historical Data Download Link (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** |  | [required] |
**data_type** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1FuturesHistDataLinkGet200Response**](_sapi_v1_futures_histDataLink_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_futures_transfer_get

> models::SapiV1FuturesTransferGet200Response sapi_v1_futures_transfer_get(asset, start_time, timestamp, signature, end_time, current, size, recv_window)
Get Future Account Transaction History List (USER_DATA)

Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**start_time** | **i64** | UTC timestamp in ms | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1FuturesTransferGet200Response**](_sapi_v1_futures_transfer_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_futures_transfer_post

> models::SapiV1MarginBorrowRepayPost200Response sapi_v1_futures_transfer_post(asset, amount, r#type, timestamp, signature, recv_window)
New Future Account Transfer (USER_DATA)

Execute transfer between spot account and futures account.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**r#type** | **i64** | 1: transfer from spot account to USDT-Ⓜ futures account. 2: transfer from USDT-Ⓜ futures account to spot account. 3: transfer from spot account to COIN-Ⓜ futures account. 4: transfer from COIN-Ⓜ futures account to spot account. | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MarginBorrowRepayPost200Response**](_sapi_v1_margin_borrow_repay_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

