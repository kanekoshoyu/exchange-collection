# \StreamApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_user_data_stream_delete**](StreamApi.md#api_v3_user_data_stream_delete) | **DELETE** /api/v3/userDataStream | Close a ListenKey (USER_STREAM)
[**api_v3_user_data_stream_post**](StreamApi.md#api_v3_user_data_stream_post) | **POST** /api/v3/userDataStream | Create a ListenKey (USER_STREAM)
[**api_v3_user_data_stream_put**](StreamApi.md#api_v3_user_data_stream_put) | **PUT** /api/v3/userDataStream | Ping/Keep-alive a ListenKey (USER_STREAM)



## api_v3_user_data_stream_delete

> serde_json::Value api_v3_user_data_stream_delete(listen_key)
Close a ListenKey (USER_STREAM)

Close out a user data stream.  Weight: 2

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


## api_v3_user_data_stream_post

> models::ApiV3UserDataStreamPost200Response api_v3_user_data_stream_post()
Create a ListenKey (USER_STREAM)

Start a new user data stream. The stream will close after 60 minutes unless a keepalive is sent. If the account has an active `listenKey`, that `listenKey` will be returned and its validity will be extended for 60 minutes.  Weight: 2

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiV3UserDataStreamPost200Response**](_api_v3_userDataStream_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_user_data_stream_put

> serde_json::Value api_v3_user_data_stream_put(listen_key)
Ping/Keep-alive a ListenKey (USER_STREAM)

Keepalive a user data stream to prevent a time out. User data streams will close after 60 minutes. It's recommended to send a ping about every 30 minutes.  Weight: 2

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

