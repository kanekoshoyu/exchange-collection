# \SpotAlgoApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_algo_spot_historical_orders_get**](SpotAlgoApi.md#sapi_v1_algo_spot_historical_orders_get) | **GET** /sapi/v1/algo/spot/historicalOrders | Query Historical Algo Orders
[**sapi_v1_algo_spot_new_order_twap_post**](SpotAlgoApi.md#sapi_v1_algo_spot_new_order_twap_post) | **POST** /sapi/v1/algo/spot/newOrderTwap | Time-Weighted Average Price (Twap) New Order
[**sapi_v1_algo_spot_open_orders_get**](SpotAlgoApi.md#sapi_v1_algo_spot_open_orders_get) | **GET** /sapi/v1/algo/spot/openOrders | Query Current Algo Open Orders
[**sapi_v1_algo_spot_order_delete**](SpotAlgoApi.md#sapi_v1_algo_spot_order_delete) | **DELETE** /sapi/v1/algo/spot/order | Cancel Algo Order
[**sapi_v1_algo_spot_sub_orders_get**](SpotAlgoApi.md#sapi_v1_algo_spot_sub_orders_get) | **GET** /sapi/v1/algo/spot/subOrders | Query Sub Orders



## sapi_v1_algo_spot_historical_orders_get

> models::SapiV1AlgoSpotHistoricalOrdersGet200Response sapi_v1_algo_spot_historical_orders_get(symbol, side, timestamp, signature, start_time, end_time, page, page_size, recv_window)
Query Historical Algo Orders

Get all historical SPOT TWAP orders  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**side** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**page** | Option<**i32**> | Default 1 |  |
**page_size** | Option<**String**> | MIN 1, MAX 100; Default 100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1AlgoSpotHistoricalOrdersGet200Response**](_sapi_v1_algo_spot_historicalOrders_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_algo_spot_new_order_twap_post

> models::SapiV1AlgoSpotNewOrderTwapPost200Response sapi_v1_algo_spot_new_order_twap_post(symbol, side, quantity, duration, timestamp, signature, client_algo_id, limit_price, recv_window)
Time-Weighted Average Price (Twap) New Order

Place a new spot TWAP order with Algo service.  Weight(UID): 3000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**side** | **String** |  | [required] |
**quantity** | **f64** |  | [required] |
**duration** | **i32** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**client_algo_id** | Option<**String**> |  |  |
**limit_price** | Option<**f32**> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1AlgoSpotNewOrderTwapPost200Response**](_sapi_v1_algo_spot_newOrderTwap_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_algo_spot_open_orders_get

> models::SapiV1AlgoSpotOpenOrdersGet200Response sapi_v1_algo_spot_open_orders_get(timestamp, signature, recv_window)
Query Current Algo Open Orders

Get all open SPOT TWAP orders  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1AlgoSpotOpenOrdersGet200Response**](_sapi_v1_algo_spot_openOrders_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_algo_spot_order_delete

> models::SapiV1AlgoSpotOrderDelete200Response sapi_v1_algo_spot_order_delete(algo_id, timestamp, signature, recv_window)
Cancel Algo Order

Cancel an open TWAP order  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**algo_id** | **i64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1AlgoSpotOrderDelete200Response**](_sapi_v1_algo_spot_order_delete_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_algo_spot_sub_orders_get

> models::SapiV1AlgoSpotSubOrdersGet200Response sapi_v1_algo_spot_sub_orders_get(algo_id, timestamp, signature, page, page_size, recv_window)
Query Sub Orders

Get respective sub orders for a specified algoId  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**algo_id** | **i64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**page** | Option<**i32**> | Default 1 |  |
**page_size** | Option<**String**> | MIN 1, MAX 100; Default 100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1AlgoSpotSubOrdersGet200Response**](_sapi_v1_algo_spot_subOrders_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

