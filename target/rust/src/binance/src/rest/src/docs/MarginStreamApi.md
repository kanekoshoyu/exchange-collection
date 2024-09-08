# \MarginStreamApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_user_data_stream_delete**](MarginStreamApi.md#sapi_v1_user_data_stream_delete) | **DELETE** /sapi/v1/userDataStream | Close a ListenKey (USER_STREAM)
[**sapi_v1_user_data_stream_post**](MarginStreamApi.md#sapi_v1_user_data_stream_post) | **POST** /sapi/v1/userDataStream | Create a ListenKey (USER_STREAM)
[**sapi_v1_user_data_stream_put**](MarginStreamApi.md#sapi_v1_user_data_stream_put) | **PUT** /sapi/v1/userDataStream | Ping/Keep-alive a ListenKey (USER_STREAM)



## sapi_v1_user_data_stream_delete

> serde_json::Value sapi_v1_user_data_stream_delete(listen_key)
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


## sapi_v1_user_data_stream_post

> models::ApiV3UserDataStreamPost200Response sapi_v1_user_data_stream_post()
Create a ListenKey (USER_STREAM)

Start a new user data stream. The stream will close after 60 minutes unless a keepalive is sent. If the account has an active `listenKey`, that `listenKey` will be returned and its validity will be extended for 60 minutes.  Weight: 1

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


## sapi_v1_user_data_stream_put

> serde_json::Value sapi_v1_user_data_stream_put(listen_key)
Ping/Keep-alive a ListenKey (USER_STREAM)

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

