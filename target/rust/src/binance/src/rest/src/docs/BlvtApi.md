# \BlvtApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_blvt_redeem_post**](BlvtApi.md#sapi_v1_blvt_redeem_post) | **POST** /sapi/v1/blvt/redeem | Redeem BLVT (USER_DATA)
[**sapi_v1_blvt_redeem_record_get**](BlvtApi.md#sapi_v1_blvt_redeem_record_get) | **GET** /sapi/v1/blvt/redeem/record | Redemption Record (USER_DATA)
[**sapi_v1_blvt_subscribe_post**](BlvtApi.md#sapi_v1_blvt_subscribe_post) | **POST** /sapi/v1/blvt/subscribe | Subscribe BLVT (USER_DATA)
[**sapi_v1_blvt_subscribe_record_get**](BlvtApi.md#sapi_v1_blvt_subscribe_record_get) | **GET** /sapi/v1/blvt/subscribe/record | Query Subscription Record (USER_DATA)
[**sapi_v1_blvt_token_info_get**](BlvtApi.md#sapi_v1_blvt_token_info_get) | **GET** /sapi/v1/blvt/tokenInfo | BLVT Info (MARKET_DATA)
[**sapi_v1_blvt_user_limit_get**](BlvtApi.md#sapi_v1_blvt_user_limit_get) | **GET** /sapi/v1/blvt/userLimit | BLVT User Limit Info (USER_DATA)



## sapi_v1_blvt_redeem_post

> models::SapiV1BlvtRedeemPost200Response sapi_v1_blvt_redeem_post(token_name, amount, timestamp, signature, recv_window)
Redeem BLVT (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_name** | **String** | BTCDOWN, BTCUP | [required] |
**amount** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1BlvtRedeemPost200Response**](_sapi_v1_blvt_redeem_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_blvt_redeem_record_get

> Vec<models::SapiV1BlvtRedeemRecordGet200ResponseInner> sapi_v1_blvt_redeem_record_get(timestamp, signature, token_name, id, start_time, end_time, limit, recv_window)
Redemption Record (USER_DATA)

- Only the data of the latest 90 days is available  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**token_name** | Option<**String**> | BTCDOWN, BTCUP |  |
**id** | Option<**i64**> |  |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i32**> | default 1000, max 1000 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1BlvtRedeemRecordGet200ResponseInner>**](_sapi_v1_blvt_redeem_record_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_blvt_subscribe_post

> models::SapiV1BlvtSubscribePost200Response sapi_v1_blvt_subscribe_post(token_name, cost, timestamp, signature, recv_window)
Subscribe BLVT (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_name** | **String** | BTCDOWN, BTCUP | [required] |
**cost** | **f64** | Spot balance | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1BlvtSubscribePost200Response**](_sapi_v1_blvt_subscribe_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_blvt_subscribe_record_get

> models::SapiV1BlvtSubscribeRecordGet200Response sapi_v1_blvt_subscribe_record_get(timestamp, signature, token_name, id, start_time, end_time, limit, recv_window)
Query Subscription Record (USER_DATA)

- Only the data of the latest 90 days is available  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**token_name** | Option<**String**> | BTCDOWN, BTCUP |  |
**id** | Option<**i64**> |  |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1BlvtSubscribeRecordGet200Response**](_sapi_v1_blvt_subscribe_record_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_blvt_token_info_get

> Vec<models::SapiV1BlvtTokenInfoGet200ResponseInner> sapi_v1_blvt_token_info_get(token_name)
BLVT Info (MARKET_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_name** | Option<**String**> | BTCDOWN, BTCUP |  |

### Return type

[**Vec<models::SapiV1BlvtTokenInfoGet200ResponseInner>**](_sapi_v1_blvt_tokenInfo_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_blvt_user_limit_get

> Vec<models::SapiV1BlvtUserLimitGet200ResponseInner> sapi_v1_blvt_user_limit_get(timestamp, signature, token_name, recv_window)
BLVT User Limit Info (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**token_name** | Option<**String**> | BTCDOWN, BTCUP |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1BlvtUserLimitGet200ResponseInner>**](_sapi_v1_blvt_userLimit_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

