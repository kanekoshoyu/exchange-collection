# \ConvertApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_convert_accept_quote_post**](ConvertApi.md#sapi_v1_convert_accept_quote_post) | **POST** /sapi/v1/convert/acceptQuote | Accept Quote (TRADE)
[**sapi_v1_convert_asset_info_get**](ConvertApi.md#sapi_v1_convert_asset_info_get) | **GET** /sapi/v1/convert/assetInfo | Query order quantity precision per asset (USER_DATA)
[**sapi_v1_convert_exchange_info_get**](ConvertApi.md#sapi_v1_convert_exchange_info_get) | **GET** /sapi/v1/convert/exchangeInfo | List All Convert Pairs
[**sapi_v1_convert_get_quote_post**](ConvertApi.md#sapi_v1_convert_get_quote_post) | **POST** /sapi/v1/convert/getQuote | Send quote request (USER_DATA)
[**sapi_v1_convert_limit_cancel_order_post**](ConvertApi.md#sapi_v1_convert_limit_cancel_order_post) | **POST** /sapi/v1/convert/limit/cancelOrder | Cancel limit order (USER_DATA)
[**sapi_v1_convert_limit_place_order_post**](ConvertApi.md#sapi_v1_convert_limit_place_order_post) | **POST** /sapi/v1/convert/limit/placeOrder | Place limit order (USER_DATA)
[**sapi_v1_convert_limit_query_open_orders_get**](ConvertApi.md#sapi_v1_convert_limit_query_open_orders_get) | **GET** /sapi/v1/convert/limit/queryOpenOrders | Query limit open orders (USER_DATA)
[**sapi_v1_convert_order_status_get**](ConvertApi.md#sapi_v1_convert_order_status_get) | **GET** /sapi/v1/convert/orderStatus | Order status (USER_DATA)
[**sapi_v1_convert_trade_flow_get**](ConvertApi.md#sapi_v1_convert_trade_flow_get) | **GET** /sapi/v1/convert/tradeFlow | Get Convert Trade History (USER_DATA)



## sapi_v1_convert_accept_quote_post

> models::SapiV1ConvertAcceptQuotePost200Response sapi_v1_convert_accept_quote_post(quote_id, timestamp, signature, recv_window)
Accept Quote (TRADE)

Accept the offered quote by quote ID.  Weight(UID): 500

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quote_id** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1ConvertAcceptQuotePost200Response**](_sapi_v1_convert_acceptQuote_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_convert_asset_info_get

> Vec<models::SapiV1ConvertAssetInfoGet200ResponseInner> sapi_v1_convert_asset_info_get(timestamp, signature, recv_window)
Query order quantity precision per asset (USER_DATA)

Query for supported asset precision information  Weight(IP): 100

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1ConvertAssetInfoGet200ResponseInner>**](_sapi_v1_convert_assetInfo_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_convert_exchange_info_get

> Vec<models::SapiV1ConvertExchangeInfoGet200ResponseInner> sapi_v1_convert_exchange_info_get(from_asset, to_asset)
List All Convert Pairs

Query for all convertible token pairs and the tokens’ respective upper/lower limits  Weight(IP): 3000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_asset** | Option<**String**> | User spends coin |  |
**to_asset** | Option<**String**> | User receives coin |  |

### Return type

[**Vec<models::SapiV1ConvertExchangeInfoGet200ResponseInner>**](_sapi_v1_convert_exchangeInfo_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_convert_get_quote_post

> models::SapiV1ConvertGetQuotePost200Response sapi_v1_convert_get_quote_post(from_asset, to_asset, timestamp, signature, from_amount, to_amount, valid_time, wallet_type, recv_window)
Send quote request (USER_DATA)

Request a quote for the requested token pairs  Weight(UID): 200

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_asset** | **String** |  | [required] |
**to_asset** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**from_amount** | Option<**f32**> | When specified, it is the amount you will be debited after the conversion |  |
**to_amount** | Option<**f32**> | When specified, it is the amount you will be debited after the conversion |  |
**valid_time** | Option<**String**> | 10s, 30s, 1m, 2m, default 10s |  |
**wallet_type** | Option<**String**> | SPOT or FUNDING. Default is SPOT |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1ConvertGetQuotePost200Response**](_sapi_v1_convert_getQuote_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_convert_limit_cancel_order_post

> models::SapiV1ConvertLimitCancelOrderPost200Response sapi_v1_convert_limit_cancel_order_post(order_id, timestamp, signature, recv_window)
Cancel limit order (USER_DATA)

Enable users to cancel a limit order  Weight(UID): 200

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **i64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1ConvertLimitCancelOrderPost200Response**](_sapi_v1_convert_limit_cancelOrder_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_convert_limit_place_order_post

> models::SapiV1ConvertLimitPlaceOrderPost200Response sapi_v1_convert_limit_place_order_post(base_asset, quote_asset, limit_price, side, timestamp, signature, base_amount, quote_amount, wallet_type, expired_type, recv_window)
Place limit order (USER_DATA)

Enable users to place a limit order  - baseAsset or quoteAsset can be determined via exchangeInfo endpoint. - Limit price is defined from baseAsset to quoteAsset. - Either baseAmount or quoteAmount is used.  Weight(UID): 500

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**base_asset** | **String** |  | [required] |
**quote_asset** | **String** |  | [required] |
**limit_price** | **f64** | Symbol limit price (from baseAsset to quoteAsset) | [required] |
**side** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**base_amount** | Option<**f64**> | Base asset amount. (One of baseAmount or quoteAmount is required) |  |
**quote_amount** | Option<**f64**> | Quote asset amount. (One of baseAmount or quoteAmount is required) |  |
**wallet_type** | Option<**String**> | SPOT or FUNDING or SPOT_FUNDING. It is to use which type of assets. Default is SPOT. |  |
**expired_type** | Option<**String**> | 1_D, 3_D, 7_D, 30_D (D means day) |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1ConvertLimitPlaceOrderPost200Response**](_sapi_v1_convert_limit_placeOrder_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_convert_limit_query_open_orders_get

> models::SapiV1ConvertLimitQueryOpenOrdersGet200Response sapi_v1_convert_limit_query_open_orders_get(timestamp, signature, recv_window)
Query limit open orders (USER_DATA)

Enable users to query for all existing limit orders  Weight(UID): 3000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1ConvertLimitQueryOpenOrdersGet200Response**](_sapi_v1_convert_limit_queryOpenOrders_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_convert_order_status_get

> models::SapiV1ConvertOrderStatusGet200Response sapi_v1_convert_order_status_get(timestamp, signature, order_id, quote_id, recv_window)
Order status (USER_DATA)

Query order status by order ID.  Weight(UID): 100

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**order_id** | Option<**String**> |  |  |
**quote_id** | Option<**String**> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1ConvertOrderStatusGet200Response**](_sapi_v1_convert_orderStatus_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_convert_trade_flow_get

> models::SapiV1ConvertTradeFlowGet200Response sapi_v1_convert_trade_flow_get(start_time, end_time, timestamp, signature, limit, recv_window)
Get Convert Trade History (USER_DATA)

- The max interval between startTime and endTime is 30 days.  Weight(UID): 3000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_time** | **i64** | UTC timestamp in ms | [required] |
**end_time** | **i64** | UTC timestamp in ms | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**limit** | Option<**i32**> | default 100, max 1000 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1ConvertTradeFlowGet200Response**](_sapi_v1_convert_tradeFlow_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

