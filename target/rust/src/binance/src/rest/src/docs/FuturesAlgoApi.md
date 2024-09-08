# \FuturesAlgoApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_algo_futures_historical_orders_get**](FuturesAlgoApi.md#sapi_v1_algo_futures_historical_orders_get) | **GET** /sapi/v1/algo/futures/historicalOrders | Query Historical Algo Orders (USER_DATA)
[**sapi_v1_algo_futures_new_order_twap_post**](FuturesAlgoApi.md#sapi_v1_algo_futures_new_order_twap_post) | **POST** /sapi/v1/algo/futures/newOrderTwap | Time-Weighted Average Price(Twap) New Order (TRADE)
[**sapi_v1_algo_futures_new_order_vp_post**](FuturesAlgoApi.md#sapi_v1_algo_futures_new_order_vp_post) | **POST** /sapi/v1/algo/futures/newOrderVp | Volume Participation(VP) New Order (TRADE)
[**sapi_v1_algo_futures_open_orders_get**](FuturesAlgoApi.md#sapi_v1_algo_futures_open_orders_get) | **GET** /sapi/v1/algo/futures/openOrders | Query Current Algo Open Orders (USER_DATA)
[**sapi_v1_algo_futures_order_delete**](FuturesAlgoApi.md#sapi_v1_algo_futures_order_delete) | **DELETE** /sapi/v1/algo/futures/order | Cancel Algo Order(TRADE)
[**sapi_v1_algo_futures_sub_orders_get**](FuturesAlgoApi.md#sapi_v1_algo_futures_sub_orders_get) | **GET** /sapi/v1/algo/futures/subOrders | Query Sub Orders (USER_DATA)



## sapi_v1_algo_futures_historical_orders_get

> models::SapiV1AlgoFuturesHistoricalOrdersGet200Response sapi_v1_algo_futures_historical_orders_get(timestamp, signature, symbol, side, start_time, end_time, page, page_size, recv_window)
Query Historical Algo Orders (USER_DATA)

- You need to enable Futures Trading Permission for the api key which requests this endpoint. - Base URL: https://api.binance.com  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**symbol** | Option<**String**> | Trading symbol, e.g. BNBUSDT |  |
**side** | Option<**String**> |  |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**page** | Option<**i32**> | Default 1 |  |
**page_size** | Option<**String**> | MIN 1, MAX 100; Default 100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1AlgoFuturesHistoricalOrdersGet200Response**](_sapi_v1_algo_futures_historicalOrders_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_algo_futures_new_order_twap_post

> models::SapiV1AlgoFuturesNewOrderVpPost200Response sapi_v1_algo_futures_new_order_twap_post(symbol, side, quantity, duration, timestamp, signature, position_side, client_algo_id, reduce_only, limit_price, recv_window)
Time-Weighted Average Price(Twap) New Order (TRADE)

Send in a Twap new order. Only support on USDⓈ-M Contracts.  You need to enable Futures Trading Permission for the api key which requests this endpoint. Base URL: https://api.binance.com  - Total Algo open orders max allowed: 10 orders. - Leverage of symbols and position mode will be the same as your futures account settings. You can set up through the trading page or fapi. - Receiving \"success\": true does not mean that your order will be executed. Please use the query order endpoints(GET sapi/v1/algo/futures/openOrders or GET sapi/v1/algo/futures/historicalOrders) to check the order status. For example: Your futures balance is insufficient, or open position with reduce only or position side is inconsistent with your own setting. In these cases you will receive \"success\": true, but the order status will be expired after we check it. - quantity * 60 / duration should be larger than minQty - duration cannot be less than 5 mins or more than 24 hours. - For delivery contracts, TWAP end time should be one hour earlier than the delivery time of the symbol.  Weight(UID): 3000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**side** | **String** |  | [required] |
**quantity** | **f64** | Quantity of base asset; The notional (quantity * mark price(base asset)) must be more than the equivalent of 10,000 USDT and less than the equivalent of 1,000,000 USDT | [required] |
**duration** | **i64** | Duration for TWAP orders in seconds. [300, 86400];Less than 5min => defaults to 5 min; Greater than 24h => defaults to 24h | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**position_side** | Option<**String**> | Default BOTH for One-way Mode ; LONG or SHORT for Hedge Mode. It must be sent in Hedge Mode. |  |
**client_algo_id** | Option<**String**> | A unique id among Algo orders (length should be 32 characters)， If it is not sent, we will give default value |  |
**reduce_only** | Option<**bool**> | 'true' or 'false'. Default 'false'; Cannot be sent in Hedge Mode; Cannot be sent when you open a position |  |
**limit_price** | Option<**f64**> | Limit price of the order; If it is not sent, will place order by market price by default |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1AlgoFuturesNewOrderVpPost200Response**](_sapi_v1_algo_futures_newOrderVp_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_algo_futures_new_order_vp_post

> models::SapiV1AlgoFuturesNewOrderVpPost200Response sapi_v1_algo_futures_new_order_vp_post(symbol, side, quantity, urgency, timestamp, signature, position_side, client_algo_id, reduce_only, limit_price, recv_window)
Volume Participation(VP) New Order (TRADE)

Send in a VP new order. Only support on USDⓈ-M Contracts.  - You need to enable `Futures Trading Permission` for the api key which requests this endpoint. - Base URL: https://api.binance.com  - Total Algo open orders max allowed: 10 orders. - Leverage of symbols and position mode will be the same as your futures account settings. You can set up through the trading page or fapi. - Receiving \"success\": true does not mean that your order will be executed. Please use the query order endpoints(GET sapi/v1/algo/futures/openOrders or GET sapi/v1/algo/futures/historicalOrders) to check the order status. For example: Your futures balance is insufficient, or open position with reduce only or position side is inconsistent with your own setting. In these cases you will receive \"success\": true, but the order status will be expired after we check it.  Weight(UID): 3000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**side** | **String** |  | [required] |
**quantity** | **f64** | Quantity of base asset; The notional (quantity * mark price(base asset)) must be more than the equivalent of 10,000 USDT and less than the equivalent of 1,000,000 USDT | [required] |
**urgency** | **String** | Represent the relative speed of the current execution; ENUM: LOW, MEDIUM, HIGH | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**position_side** | Option<**String**> | Default BOTH for One-way Mode ; LONG or SHORT for Hedge Mode. It must be sent in Hedge Mode. |  |
**client_algo_id** | Option<**String**> | A unique id among Algo orders (length should be 32 characters)， If it is not sent, we will give default value |  |
**reduce_only** | Option<**bool**> | 'true' or 'false'. Default 'false'; Cannot be sent in Hedge Mode; Cannot be sent when you open a position |  |
**limit_price** | Option<**f64**> | Limit price of the order; If it is not sent, will place order by market price by default |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1AlgoFuturesNewOrderVpPost200Response**](_sapi_v1_algo_futures_newOrderVp_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_algo_futures_open_orders_get

> models::SapiV1AlgoFuturesOpenOrdersGet200Response sapi_v1_algo_futures_open_orders_get(timestamp, signature, recv_window)
Query Current Algo Open Orders (USER_DATA)

- You need to enable Futures Trading Permission for the api key which requests this endpoint. - Base URL: https://api.binance.com  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1AlgoFuturesOpenOrdersGet200Response**](_sapi_v1_algo_futures_openOrders_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_algo_futures_order_delete

> models::SapiV1AlgoFuturesOrderDelete200Response sapi_v1_algo_futures_order_delete(algo_id, timestamp, signature, recv_window)
Cancel Algo Order(TRADE)

Cancel an active order. - You need to enable Futures Trading Permission for the api key which requests this endpoint. - Base URL: https://api.binance.com  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**algo_id** | **i64** | Eg. 14511 | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1AlgoFuturesOrderDelete200Response**](_sapi_v1_algo_futures_order_delete_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_algo_futures_sub_orders_get

> models::SapiV1AlgoFuturesSubOrdersGet200Response sapi_v1_algo_futures_sub_orders_get(algo_id, timestamp, signature, page, page_size, recv_window)
Query Sub Orders (USER_DATA)

- You need to enable Futures Trading Permission for the api key which requests this endpoint. - Base URL: https://api.binance.com  Weight(IP): 1

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

[**models::SapiV1AlgoFuturesSubOrdersGet200Response**](_sapi_v1_algo_futures_subOrders_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

