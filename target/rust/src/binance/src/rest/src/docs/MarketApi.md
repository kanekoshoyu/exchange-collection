# \MarketApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_agg_trades_get**](MarketApi.md#api_v3_agg_trades_get) | **GET** /api/v3/aggTrades | Compressed/Aggregate Trades List
[**api_v3_avg_price_get**](MarketApi.md#api_v3_avg_price_get) | **GET** /api/v3/avgPrice | Current Average Price
[**api_v3_depth_get**](MarketApi.md#api_v3_depth_get) | **GET** /api/v3/depth | Order Book
[**api_v3_exchange_info_get**](MarketApi.md#api_v3_exchange_info_get) | **GET** /api/v3/exchangeInfo | Exchange Information
[**api_v3_historical_trades_get**](MarketApi.md#api_v3_historical_trades_get) | **GET** /api/v3/historicalTrades | Old Trade Lookup
[**api_v3_klines_get**](MarketApi.md#api_v3_klines_get) | **GET** /api/v3/klines | Kline/Candlestick Data
[**api_v3_ping_get**](MarketApi.md#api_v3_ping_get) | **GET** /api/v3/ping | Test Connectivity
[**api_v3_ticker24hr_get**](MarketApi.md#api_v3_ticker24hr_get) | **GET** /api/v3/ticker/24hr | 24hr Ticker Price Change Statistics
[**api_v3_ticker_book_ticker_get**](MarketApi.md#api_v3_ticker_book_ticker_get) | **GET** /api/v3/ticker/bookTicker | Symbol Order Book Ticker
[**api_v3_ticker_get**](MarketApi.md#api_v3_ticker_get) | **GET** /api/v3/ticker | Rolling window price change statistics
[**api_v3_ticker_price_get**](MarketApi.md#api_v3_ticker_price_get) | **GET** /api/v3/ticker/price | Symbol Price Ticker
[**api_v3_ticker_trading_day_get**](MarketApi.md#api_v3_ticker_trading_day_get) | **GET** /api/v3/ticker/tradingDay | Trading Day Ticker
[**api_v3_time_get**](MarketApi.md#api_v3_time_get) | **GET** /api/v3/time | Check Server Time
[**api_v3_trades_get**](MarketApi.md#api_v3_trades_get) | **GET** /api/v3/trades | Recent Trades List
[**api_v3_ui_klines_get**](MarketApi.md#api_v3_ui_klines_get) | **GET** /api/v3/uiKlines | UIKlines



## api_v3_agg_trades_get

> Vec<models::AggTrade> api_v3_agg_trades_get(symbol, from_id, start_time, end_time, limit)
Compressed/Aggregate Trades List

Get compressed, aggregate trades. Trades that fill at the time, from the same order, with the same price will have the quantity aggregated. - If `fromId`, `startTime`, and `endTime` are not sent, the most recent aggregate trades will be returned. - Note that if a trade has the following values, this was a duplicate aggregate trade and marked as invalid:    p = '0' // price    q = '0' // qty    f = -1 // ﬁrst_trade_id    l = -1 // last_trade_id  Weight(IP): 2

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**from_id** | Option<**i64**> | Trade id to fetch from. Default gets most recent trades. |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |

### Return type

[**Vec<models::AggTrade>**](aggTrade.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_avg_price_get

> models::ApiV3AvgPriceGet200Response api_v3_avg_price_get(symbol)
Current Average Price

Current average price for a symbol.  Weight(IP): 2

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |

### Return type

[**models::ApiV3AvgPriceGet200Response**](_api_v3_avgPrice_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_depth_get

> models::ApiV3DepthGet200Response api_v3_depth_get(symbol, limit)
Order Book

| Limit               | Weight(IP)  | |---------------------|-------------| | 1-100               | 5           | | 101-500             | 25          | | 501-1000            | 50          | | 1001-5000           | 250         |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**limit** | Option<**i32**> | If limit > 5000, then the response will truncate to 5000 |  |[default to 100]

### Return type

[**models::ApiV3DepthGet200Response**](_api_v3_depth_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_exchange_info_get

> models::ApiV3ExchangeInfoGet200Response api_v3_exchange_info_get(symbol, symbols, permissions)
Exchange Information

Current exchange trading rules and symbol information  - If any symbol provided in either symbol or symbols do not exist, the endpoint will throw an error. - All parameters are optional. - permissions can support single or multiple values (e.g. SPOT, [\"MARGIN\",\"LEVERAGED\"]) - If permissions parameter not provided, the default values will be [\"SPOT\",\"MARGIN\",\"LEVERAGED\"].   - To display all permissions you need to specify them explicitly. (e.g. SPOT, MARGIN,...)  Examples of Symbol Permissions Interpretation from the Response: - [[\"A\",\"B\"]] means you may place an order if your account has either permission \"A\" or permission \"B\".  - [[\"A\"],[\"B\"]] means you can place an order if your account has permission \"A\" and permission \"B\". - [[\"A\"],[\"B\",\"C\"]] means you can place an order if your account has permission \"A\" and permission \"B\" or permission \"C\". (Inclusive or is applied here, not exclusive or, so your account may have both permission \"B\" and permission \"C\".)  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Trading symbol, e.g. BNBUSDT |  |
**symbols** | Option<**String**> |  |  |
**permissions** | Option<**String**> |  |  |

### Return type

[**models::ApiV3ExchangeInfoGet200Response**](_api_v3_exchangeInfo_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_historical_trades_get

> Vec<models::Trade> api_v3_historical_trades_get(symbol, limit, from_id)
Old Trade Lookup

Get older market trades.  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**from_id** | Option<**i64**> | Trade id to fetch from. Default gets most recent trades. |  |

### Return type

[**Vec<models::Trade>**](trade.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_klines_get

> Vec<Vec<models::ApiV3KlinesGet200ResponseInnerInner>> api_v3_klines_get(symbol, interval, start_time, end_time, time_zone, limit)
Kline/Candlestick Data

Kline/candlestick bars for a symbol. Klines are uniquely identified by their open time.  - If `startTime` and `endTime` are not sent, the most recent klines are returned.  Weight(IP): 2

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**interval** | **String** | kline intervals | [required] |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**time_zone** | Option<**String**> | Default: 0 (UTC) |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |

### Return type

[**Vec<Vec<models::ApiV3KlinesGet200ResponseInnerInner>>**](Vec.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_ping_get

> serde_json::Value api_v3_ping_get()
Test Connectivity

Test connectivity to the Rest API.  Weight(IP): 1

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_ticker24hr_get

> models::ApiV3Ticker24hrGet200Response api_v3_ticker24hr_get(symbol, symbols, r#type)
24hr Ticker Price Change Statistics

24 hour rolling window price change statistics. Careful when accessing this with no symbol.  - If the symbol is not sent, tickers for all symbols will be returned in an array.  Weight(IP): - `2` for a single symbol; - `80` when the symbol parameter is omitted;

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Trading symbol, e.g. BNBUSDT |  |
**symbols** | Option<**String**> |  |  |
**r#type** | Option<**String**> | Supported values: FULL or MINI. If none provided, the default is FULL |  |

### Return type

[**models::ApiV3Ticker24hrGet200Response**](_api_v3_ticker_24hr_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_ticker_book_ticker_get

> models::ApiV3TickerBookTickerGet200Response api_v3_ticker_book_ticker_get(symbol, symbols)
Symbol Order Book Ticker

Best price/qty on the order book for a symbol or symbols.  - If the symbol is not sent, bookTickers for all symbols will be returned in an array.  Weight(IP): - `2` for a single symbol; - `4` when the symbol parameter is omitted;

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Trading symbol, e.g. BNBUSDT |  |
**symbols** | Option<**String**> |  |  |

### Return type

[**models::ApiV3TickerBookTickerGet200Response**](_api_v3_ticker_bookTicker_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_ticker_get

> models::ApiV3TickerGet200Response api_v3_ticker_get(symbol, symbols, window_size, r#type)
Rolling window price change statistics

The window used to compute statistics is typically slightly wider than requested windowSize.  openTime for /api/v3/ticker always starts on a minute, while the closeTime is the current time of the request. As such, the effective window might be up to 1 minute wider than requested.  E.g. If the closeTime is 1641287867099 (January 04, 2022 09:17:47:099 UTC) , and the windowSize is 1d. the openTime will be: 1641201420000 (January 3, 2022, 09:17:00 UTC)  Weight(IP): 4 for each requested symbol regardless of windowSize.  The weight for this request will cap at 200 once the number of symbols in the request is more than 50.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Trading symbol, e.g. BNBUSDT |  |
**symbols** | Option<**String**> |  |  |
**window_size** | Option<**String**> | Defaults to 1d if no parameter provided. Supported windowSize values: 1m,2m....59m for minutes 1h, 2h....23h - for hours 1d...7d - for days.  Units cannot be combined (e.g. 1d2h is not allowed) |  |
**r#type** | Option<**String**> | Supported values: FULL or MINI. If none provided, the default is FULL |  |

### Return type

[**models::ApiV3TickerGet200Response**](_api_v3_ticker_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_ticker_price_get

> models::ApiV3TickerPriceGet200Response api_v3_ticker_price_get(symbol, symbols)
Symbol Price Ticker

Latest price for a symbol or symbols.  - If the symbol is not sent, prices for all symbols will be returned in an array.  Weight(IP): - `2` for a single symbol; - `4` when the symbol parameter is omitted;

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Trading symbol, e.g. BNBUSDT |  |
**symbols** | Option<**String**> |  |  |

### Return type

[**models::ApiV3TickerPriceGet200Response**](_api_v3_ticker_price_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_ticker_trading_day_get

> models::ApiV3TickerTradingDayGet200Response api_v3_ticker_trading_day_get(symbol, symbols, time_zone, r#type)
Trading Day Ticker

Price change statistics for a trading day.  Notes: - Supported values for timeZone:   - Hours and minutes (e.g. -1:00, 05:45)   - Only hours (e.g. 0, 8, 4)  Weight: - `4` for each requested symbol.  - The weight for this request will cap at `200` once the number of symbols in the request is more than `50`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Trading symbol, e.g. BNBUSDT |  |
**symbols** | Option<**String**> |  |  |
**time_zone** | Option<**String**> | Default: 0 (UTC) |  |
**r#type** | Option<**String**> | Supported values: FULL or MINI. If none provided, the default is FULL |  |

### Return type

[**models::ApiV3TickerTradingDayGet200Response**](_api_v3_ticker_tradingDay_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_time_get

> models::ApiV3TimeGet200Response api_v3_time_get()
Check Server Time

Test connectivity to the Rest API and get the current server time.  Weight(IP): 1

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiV3TimeGet200Response**](_api_v3_time_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_trades_get

> Vec<models::Trade> api_v3_trades_get(symbol, limit)
Recent Trades List

Get recent trades.  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**limit** | Option<**i32**> | Default 500; max 1000. |  |

### Return type

[**Vec<models::Trade>**](trade.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_ui_klines_get

> Vec<Vec<models::ApiV3KlinesGet200ResponseInnerInner>> api_v3_ui_klines_get(symbol, interval, start_time, end_time, time_zone, limit)
UIKlines

The request is similar to klines having the same parameters and response.  uiKlines return modified kline data, optimized for presentation of candlestick charts.  Weight(IP): 2

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**interval** | **String** | kline intervals | [required] |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**time_zone** | Option<**String**> | Default: 0 (UTC) |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |

### Return type

[**Vec<Vec<models::ApiV3KlinesGet200ResponseInnerInner>>**](Vec.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

