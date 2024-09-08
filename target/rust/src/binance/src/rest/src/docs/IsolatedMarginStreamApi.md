# \IsolatedMarginStreamApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_user_data_stream_isolated_delete**](IsolatedMarginStreamApi.md#sapi_v1_user_data_stream_isolated_delete) | **DELETE** /sapi/v1/userDataStream/isolated | Close a ListenKey (USER_STREAM)
[**sapi_v1_user_data_stream_isolated_post**](IsolatedMarginStreamApi.md#sapi_v1_user_data_stream_isolated_post) | **POST** /sapi/v1/userDataStream/isolated | Generate a Listen Key (USER_STREAM)
[**sapi_v1_user_data_stream_isolated_put**](IsolatedMarginStreamApi.md#sapi_v1_user_data_stream_isolated_put) | **PUT** /sapi/v1/userDataStream/isolated | Ping/Keep-alive a Listen Key (USER_STREAM)



## sapi_v1_user_data_stream_isolated_delete

> serde_json::Value sapi_v1_user_data_stream_isolated_delete(listen_key)
Close a ListenKey (USER_STREAM)

Close out a user data stream.  Weight: 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**listen_key** | Option<**String**> | User websocket listen key |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_user_data_stream_isolated_post

> models::SapiV1UserDataStreamIsolatedPost200Response sapi_v1_user_data_stream_isolated_post()
Generate a Listen Key (USER_STREAM)

Start a new user data stream. The stream will close after 60 minutes unless a keepalive is sent. If the account has an active `listenKey`, that `listenKey` will be returned and its validity will be extended for 60 minutes.  Weight: 1

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SapiV1UserDataStreamIsolatedPost200Response**](_sapi_v1_userDataStream_isolated_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_user_data_stream_isolated_put

> serde_json::Value sapi_v1_user_data_stream_isolated_put(listen_key)
Ping/Keep-alive a Listen Key (USER_STREAM)

Keepalive a user data stream to prevent a time out. User data streams will close after 60 minutes. It's recommended to send a ping about every 30 minutes.  Weight: 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**listen_key** | Option<**String**> | User websocket listen key |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

