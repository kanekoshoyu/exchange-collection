# \MarginApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_bnb_burn_get**](MarginApi.md#sapi_v1_bnb_burn_get) | **GET** /sapi/v1/bnbBurn | Get BNB Burn Status(USER_DATA)
[**sapi_v1_bnb_burn_post**](MarginApi.md#sapi_v1_bnb_burn_post) | **POST** /sapi/v1/bnbBurn | Toggle BNB Burn On Spot Trade And Margin Interest (USER_DATA)
[**sapi_v1_margin_account_get**](MarginApi.md#sapi_v1_margin_account_get) | **GET** /sapi/v1/margin/account | Query Cross Margin Account Details (USER_DATA)
[**sapi_v1_margin_all_assets_get**](MarginApi.md#sapi_v1_margin_all_assets_get) | **GET** /sapi/v1/margin/allAssets | Get All Margin Assets (MARKET_DATA)
[**sapi_v1_margin_all_order_list_get**](MarginApi.md#sapi_v1_margin_all_order_list_get) | **GET** /sapi/v1/margin/allOrderList | Query Margin Account's all OCO (USER_DATA)
[**sapi_v1_margin_all_orders_get**](MarginApi.md#sapi_v1_margin_all_orders_get) | **GET** /sapi/v1/margin/allOrders | Query Margin Account's All Orders (USER_DATA)
[**sapi_v1_margin_all_pairs_get**](MarginApi.md#sapi_v1_margin_all_pairs_get) | **GET** /sapi/v1/margin/allPairs | Get All Cross Margin Pairs (MARKET_DATA)
[**sapi_v1_margin_available_inventory_get**](MarginApi.md#sapi_v1_margin_available_inventory_get) | **GET** /sapi/v1/margin/available-inventory | Query Margin Available Inventory (USER_DATA)
[**sapi_v1_margin_borrow_repay_get**](MarginApi.md#sapi_v1_margin_borrow_repay_get) | **GET** /sapi/v1/margin/borrow-repay | Query borrow/repay records in Margin account(USER_DATA)
[**sapi_v1_margin_borrow_repay_post**](MarginApi.md#sapi_v1_margin_borrow_repay_post) | **POST** /sapi/v1/margin/borrow-repay | Margin account borrow/repay(MARGIN)
[**sapi_v1_margin_capital_flow_get**](MarginApi.md#sapi_v1_margin_capital_flow_get) | **GET** /sapi/v1/margin/capital-flow | Get cross or isolated margin capital flow(USER_DATA)
[**sapi_v1_margin_cross_margin_collateral_ratio_get**](MarginApi.md#sapi_v1_margin_cross_margin_collateral_ratio_get) | **GET** /sapi/v1/margin/crossMarginCollateralRatio | Cross margin collateral ratio (MARKET_DATA)
[**sapi_v1_margin_cross_margin_data_get**](MarginApi.md#sapi_v1_margin_cross_margin_data_get) | **GET** /sapi/v1/margin/crossMarginData | Query Cross Margin Fee Data (USER_DATA)
[**sapi_v1_margin_delist_schedule_get**](MarginApi.md#sapi_v1_margin_delist_schedule_get) | **GET** /sapi/v1/margin/delist-schedule | Get tokens or symbols delist schedule for cross margin and isolated margin (MARKET_DATA)
[**sapi_v1_margin_exchange_small_liability_get**](MarginApi.md#sapi_v1_margin_exchange_small_liability_get) | **GET** /sapi/v1/margin/exchange-small-liability | Get Small Liability Exchange Coin List (USER_DATA)
[**sapi_v1_margin_exchange_small_liability_history_get**](MarginApi.md#sapi_v1_margin_exchange_small_liability_history_get) | **GET** /sapi/v1/margin/exchange-small-liability-history | Get Small Liability Exchange History (USER_DATA)
[**sapi_v1_margin_force_liquidation_rec_get**](MarginApi.md#sapi_v1_margin_force_liquidation_rec_get) | **GET** /sapi/v1/margin/forceLiquidationRec | Get Force Liquidation Record (USER_DATA)
[**sapi_v1_margin_interest_history_get**](MarginApi.md#sapi_v1_margin_interest_history_get) | **GET** /sapi/v1/margin/interestHistory | Get Interest History (USER_DATA)
[**sapi_v1_margin_interest_rate_history_get**](MarginApi.md#sapi_v1_margin_interest_rate_history_get) | **GET** /sapi/v1/margin/interestRateHistory | Margin Interest Rate History (USER_DATA)
[**sapi_v1_margin_isolated_account_delete**](MarginApi.md#sapi_v1_margin_isolated_account_delete) | **DELETE** /sapi/v1/margin/isolated/account | Disable Isolated Margin Account (TRADE)
[**sapi_v1_margin_isolated_account_get**](MarginApi.md#sapi_v1_margin_isolated_account_get) | **GET** /sapi/v1/margin/isolated/account | Query Isolated Margin Account Info (USER_DATA)
[**sapi_v1_margin_isolated_account_limit_get**](MarginApi.md#sapi_v1_margin_isolated_account_limit_get) | **GET** /sapi/v1/margin/isolated/accountLimit | Query Enabled Isolated Margin Account Limit (USER_DATA)
[**sapi_v1_margin_isolated_account_post**](MarginApi.md#sapi_v1_margin_isolated_account_post) | **POST** /sapi/v1/margin/isolated/account | Enable Isolated Margin Account (TRADE)
[**sapi_v1_margin_isolated_all_pairs_get**](MarginApi.md#sapi_v1_margin_isolated_all_pairs_get) | **GET** /sapi/v1/margin/isolated/allPairs | Get All Isolated Margin Symbol(USER_DATA)
[**sapi_v1_margin_isolated_margin_data_get**](MarginApi.md#sapi_v1_margin_isolated_margin_data_get) | **GET** /sapi/v1/margin/isolatedMarginData | Query Isolated Margin Fee Data (USER_DATA)
[**sapi_v1_margin_isolated_margin_tier_get**](MarginApi.md#sapi_v1_margin_isolated_margin_tier_get) | **GET** /sapi/v1/margin/isolatedMarginTier | Query Isolated Margin Tier Data (USER_DATA)
[**sapi_v1_margin_leverage_bracket_get**](MarginApi.md#sapi_v1_margin_leverage_bracket_get) | **GET** /sapi/v1/margin/leverageBracket | Query Liability Coin Leverage Bracket in Cross Margin Pro Mode (MARKET_DATA)
[**sapi_v1_margin_manual_liquidation_post**](MarginApi.md#sapi_v1_margin_manual_liquidation_post) | **POST** /sapi/v1/margin/manual-liquidation | Margin manual liquidation(MARGIN)
[**sapi_v1_margin_max_borrowable_get**](MarginApi.md#sapi_v1_margin_max_borrowable_get) | **GET** /sapi/v1/margin/maxBorrowable | Query Max Borrow (USER_DATA)
[**sapi_v1_margin_max_leverage_post**](MarginApi.md#sapi_v1_margin_max_leverage_post) | **POST** /sapi/v1/margin/max-leverage | Adjust cross margin max leverage (USER_DATA)
[**sapi_v1_margin_max_transferable_get**](MarginApi.md#sapi_v1_margin_max_transferable_get) | **GET** /sapi/v1/margin/maxTransferable | Query Max Transfer-Out Amount (USER_DATA)
[**sapi_v1_margin_my_trades_get**](MarginApi.md#sapi_v1_margin_my_trades_get) | **GET** /sapi/v1/margin/myTrades | Query Margin Account's Trade List (USER_DATA)
[**sapi_v1_margin_next_hourly_interest_rate_get**](MarginApi.md#sapi_v1_margin_next_hourly_interest_rate_get) | **GET** /sapi/v1/margin/next-hourly-interest-rate | Get a future hourly interest rate (USER_DATA)
[**sapi_v1_margin_open_order_list_get**](MarginApi.md#sapi_v1_margin_open_order_list_get) | **GET** /sapi/v1/margin/openOrderList | Query Margin Account's Open OCO (USER_DATA)
[**sapi_v1_margin_open_orders_delete**](MarginApi.md#sapi_v1_margin_open_orders_delete) | **DELETE** /sapi/v1/margin/openOrders | Margin Account Cancel all Open Orders on a Symbol (TRADE)
[**sapi_v1_margin_open_orders_get**](MarginApi.md#sapi_v1_margin_open_orders_get) | **GET** /sapi/v1/margin/openOrders | Query Margin Account's Open Orders (USER_DATA)
[**sapi_v1_margin_order_delete**](MarginApi.md#sapi_v1_margin_order_delete) | **DELETE** /sapi/v1/margin/order | Margin Account Cancel Order (TRADE)
[**sapi_v1_margin_order_get**](MarginApi.md#sapi_v1_margin_order_get) | **GET** /sapi/v1/margin/order | Query Margin Account's Order (USER_DATA)
[**sapi_v1_margin_order_list_delete**](MarginApi.md#sapi_v1_margin_order_list_delete) | **DELETE** /sapi/v1/margin/orderList | Margin Account Cancel OCO (TRADE)
[**sapi_v1_margin_order_list_get**](MarginApi.md#sapi_v1_margin_order_list_get) | **GET** /sapi/v1/margin/orderList | Query Margin Account's OCO (USER_DATA)
[**sapi_v1_margin_order_oco_post**](MarginApi.md#sapi_v1_margin_order_oco_post) | **POST** /sapi/v1/margin/order/oco | Margin Account New OCO (TRADE)
[**sapi_v1_margin_order_post**](MarginApi.md#sapi_v1_margin_order_post) | **POST** /sapi/v1/margin/order | Margin Account New Order (TRADE)
[**sapi_v1_margin_price_index_get**](MarginApi.md#sapi_v1_margin_price_index_get) | **GET** /sapi/v1/margin/priceIndex | Query Margin PriceIndex (MARKET_DATA)
[**sapi_v1_margin_rate_limit_order_get**](MarginApi.md#sapi_v1_margin_rate_limit_order_get) | **GET** /sapi/v1/margin/rateLimit/order | Query Current Margin Order Count Usage (TRADE)
[**sapi_v1_margin_trade_coeff_get**](MarginApi.md#sapi_v1_margin_trade_coeff_get) | **GET** /sapi/v1/margin/tradeCoeff | Get Summary of Margin account (USER_DATA)
[**sapi_v1_margin_transfer_get**](MarginApi.md#sapi_v1_margin_transfer_get) | **GET** /sapi/v1/margin/transfer | Get Cross Margin Transfer History (USER_DATA)



## sapi_v1_bnb_burn_get

> models::BnbBurnStatus sapi_v1_bnb_burn_get(timestamp, signature, recv_window)
Get BNB Burn Status(USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::BnbBurnStatus**](bnbBurnStatus.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_bnb_burn_post

> models::BnbBurnStatus sapi_v1_bnb_burn_post(timestamp, signature, spot_bnb_burn, interest_bnb_burn, recv_window)
Toggle BNB Burn On Spot Trade And Margin Interest (USER_DATA)

- \"spotBNBBurn\" and \"interestBNBBurn\" should be sent at least one.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**spot_bnb_burn** | Option<**String**> | Determines whether to use BNB to pay for trading fees on SPOT |  |
**interest_bnb_burn** | Option<**String**> | Determines whether to use BNB to pay for margin loan's interest |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::BnbBurnStatus**](bnbBurnStatus.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_account_get

> models::SapiV1MarginAccountGet200Response sapi_v1_margin_account_get(timestamp, signature, recv_window)
Query Cross Margin Account Details (USER_DATA)

Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MarginAccountGet200Response**](_sapi_v1_margin_account_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_all_assets_get

> Vec<models::SapiV1MarginAllAssetsGet200ResponseInner> sapi_v1_margin_all_assets_get(asset)
Get All Margin Assets (MARKET_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |

### Return type

[**Vec<models::SapiV1MarginAllAssetsGet200ResponseInner>**](_sapi_v1_margin_allAssets_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_all_order_list_get

> Vec<models::ApiV3AllOrderListGet200ResponseInner> sapi_v1_margin_all_order_list_get(timestamp, signature, is_isolated, symbol, from_id, start_time, end_time, limit, recv_window)
Query Margin Account's all OCO (USER_DATA)

Retrieves all OCO for a specific margin account based on provided optional parameters  Weight(IP): 200

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**symbol** | Option<**String**> | Mandatory for isolated margin, not supported for cross margin |  |
**from_id** | Option<**String**> | If supplied, neither `startTime` or `endTime` can be provided |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i32**> | Default Value: 500; Max Value: 1000 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::ApiV3AllOrderListGet200ResponseInner>**](_api_v3_allOrderList_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_all_orders_get

> Vec<models::MarginOrderDetail> sapi_v1_margin_all_orders_get(symbol, timestamp, signature, is_isolated, order_id, start_time, end_time, limit, recv_window)
Query Margin Account's All Orders (USER_DATA)

- If `orderId` is set, it will get orders >= that orderId. Otherwise most recent orders are returned. - For some historical orders `cummulativeQuoteQty` will be < 0, meaning the data is not available at this time.  Weight(IP): 200  Request Limit: 60 times/min per IP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**order_id** | Option<**i64**> | Order id |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::MarginOrderDetail>**](marginOrderDetail.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_all_pairs_get

> Vec<models::SapiV1MarginAllPairsGet200ResponseInner> sapi_v1_margin_all_pairs_get(symbol)
Get All Cross Margin Pairs (MARKET_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |

### Return type

[**Vec<models::SapiV1MarginAllPairsGet200ResponseInner>**](_sapi_v1_margin_allPairs_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_available_inventory_get

> models::SapiV1MarginAvailableInventoryGet200Response sapi_v1_margin_available_inventory_get(r#type, timestamp, signature)
Query Margin Available Inventory (USER_DATA)

Margin available Inventory query  Weight(UID): 50

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |

### Return type

[**models::SapiV1MarginAvailableInventoryGet200Response**](_sapi_v1_margin_available_inventory_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_borrow_repay_get

> models::SapiV1MarginBorrowRepayGet200Response sapi_v1_margin_borrow_repay_get(asset, r#type, timestamp, signature, isolated_symbol, tx_id, start_time, end_time, current, size, recv_window)
Query borrow/repay records in Margin account(USER_DATA)

Query borrow/repay records in Margin account  - txId or startTime must be sent. txId takes precedence. Response in descending order - If an asset is sent, data within 30 days before endTime; If an asset is not sent, data within 7 days before endTime - If neither startTime nor endTime is sent, the recent 7-day data will be returned. - startTime set as endTime - 7 days by default, endTime set as current time by default  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**r#type** | **String** | BORROW or REPAY | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**isolated_symbol** | Option<**String**> | Isolated symbol |  |
**tx_id** | Option<**i64**> | tranId in POST /sapi/v1/margin/loan |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MarginBorrowRepayGet200Response**](_sapi_v1_margin_borrow_repay_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_borrow_repay_post

> models::SapiV1MarginBorrowRepayPost200Response sapi_v1_margin_borrow_repay_post(asset, is_isolated, symbol, amount, r#type, timestamp, signature, recv_window)
Margin account borrow/repay(MARGIN)

Margin account borrow/repay(MARGIN)  Weight(UID): 3000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**is_isolated** | **String** | TRUE for isolated margin, FALSE for crossed margin | [required] |
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**amount** | **f64** |  | [required] |
**r#type** | **String** | BORROW or REPAY | [required] |
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


## sapi_v1_margin_capital_flow_get

> Vec<models::SapiV1MarginCapitalFlowGet200ResponseInner> sapi_v1_margin_capital_flow_get(timestamp, signature, asset, symbol, r#type, start_time, end_time, from_id, limit, recv_window)
Get cross or isolated margin capital flow(USER_DATA)

Get cross or isolated margin capital flow  Weight(IP): 100

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> |  |  |
**symbol** | Option<**String**> | Required when querying isolated data |  |
**r#type** | Option<**String**> |  |  |
**start_time** | Option<**i64**> | Only supports querying the data of the last 90 days |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**from_id** | Option<**i64**> | If fromId is set, the data with id > fromId will be returned. Otherwise the latest data will be returned |  |
**limit** | Option<**i64**> | The number of data items returned each time is limited. Default 500; Max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1MarginCapitalFlowGet200ResponseInner>**](_sapi_v1_margin_capital_flow_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_cross_margin_collateral_ratio_get

> Vec<models::SapiV1MarginCrossMarginCollateralRatioGet200ResponseInner> sapi_v1_margin_cross_margin_collateral_ratio_get()
Cross margin collateral ratio (MARKET_DATA)

 Weight(IP): 100

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::SapiV1MarginCrossMarginCollateralRatioGet200ResponseInner>**](_sapi_v1_margin_crossMarginCollateralRatio_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_cross_margin_data_get

> Vec<models::SapiV1MarginCrossMarginDataGet200ResponseInner> sapi_v1_margin_cross_margin_data_get(timestamp, signature, vip_level, coin, recv_window)
Query Cross Margin Fee Data (USER_DATA)

Get cross margin fee data collection with any vip level or user's current specific data as https://www.binance.com/en/margin-fee  Weight(IP): 1 when coin is specified; 5 when the coin parameter is omitted

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**vip_level** | Option<**i32**> | Defaults to user's vip level |  |
**coin** | Option<**String**> | Coin name |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1MarginCrossMarginDataGet200ResponseInner>**](_sapi_v1_margin_crossMarginData_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_delist_schedule_get

> Vec<models::SapiV1MarginDelistScheduleGet200ResponseInner> sapi_v1_margin_delist_schedule_get(timestamp, signature, recv_window)
Get tokens or symbols delist schedule for cross margin and isolated margin (MARKET_DATA)

Get tokens or symbols delist schedule for cross margin and isolated margin  Weight(IP): 100

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1MarginDelistScheduleGet200ResponseInner>**](_sapi_v1_margin_delist_schedule_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_exchange_small_liability_get

> Vec<models::SapiV1MarginExchangeSmallLiabilityGet200ResponseInner> sapi_v1_margin_exchange_small_liability_get(timestamp, signature, recv_window)
Get Small Liability Exchange Coin List (USER_DATA)

Query the coins which can be small liability exchange  Weight(UID): 100

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1MarginExchangeSmallLiabilityGet200ResponseInner>**](_sapi_v1_margin_exchange_small_liability_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_exchange_small_liability_history_get

> models::SapiV1MarginExchangeSmallLiabilityHistoryGet200Response sapi_v1_margin_exchange_small_liability_history_get(timestamp, signature, current, size, start_time, end_time, recv_window)
Get Small Liability Exchange History (USER_DATA)

Get Small liability Exchange History  Weight(UID): 100

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MarginExchangeSmallLiabilityHistoryGet200Response**](_sapi_v1_margin_exchange_small_liability_history_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_force_liquidation_rec_get

> models::SapiV1MarginForceLiquidationRecGet200Response sapi_v1_margin_force_liquidation_rec_get(timestamp, signature, start_time, end_time, isolated_symbol, current, size, recv_window)
Get Force Liquidation Record (USER_DATA)

- Response in descending order  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**isolated_symbol** | Option<**String**> | Isolated symbol |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MarginForceLiquidationRecGet200Response**](_sapi_v1_margin_forceLiquidationRec_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_interest_history_get

> models::SapiV1MarginInterestHistoryGet200Response sapi_v1_margin_interest_history_get(timestamp, signature, asset, isolated_symbol, start_time, end_time, current, size, archived, recv_window)
Get Interest History (USER_DATA)

- Response in descending order - If `isolatedSymbol` is not sent, crossed margin data will be returned - Set `archived` to `true` to query data from 6 months ago - `type` in response has 4 enums:   - `PERIODIC` interest charged per hour   - `ON_BORROW` first interest charged on borrow   - `PERIODIC_CONVERTED` interest charged per hour converted into BNB   - `ON_BORROW_CONVERTED` first interest charged on borrow converted into BNB  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> |  |  |
**isolated_symbol** | Option<**String**> | Isolated symbol |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**archived** | Option<**String**> | Default: false. Set to true for archived data from 6 months ago |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MarginInterestHistoryGet200Response**](_sapi_v1_margin_interestHistory_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_interest_rate_history_get

> Vec<models::SapiV1MarginInterestRateHistoryGet200ResponseInner> sapi_v1_margin_interest_rate_history_get(asset, timestamp, signature, vip_level, start_time, end_time, recv_window)
Margin Interest Rate History (USER_DATA)

The max interval between startTime and endTime is 30 days.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**vip_level** | Option<**i32**> | Defaults to user's vip level |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1MarginInterestRateHistoryGet200ResponseInner>**](_sapi_v1_margin_interestRateHistory_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_isolated_account_delete

> models::SapiV1MarginIsolatedAccountPost200Response sapi_v1_margin_isolated_account_delete(symbol, timestamp, signature, recv_window)
Disable Isolated Margin Account (TRADE)

Disable isolated margin account for a specific symbol. Each trading pair can only be deactivated once every 24 hours .  Weight(UID): 300

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MarginIsolatedAccountPost200Response**](_sapi_v1_margin_isolated_account_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_isolated_account_get

> models::IsolatedMarginAccountInfo sapi_v1_margin_isolated_account_get(timestamp, signature, symbols, recv_window)
Query Isolated Margin Account Info (USER_DATA)

- If \"symbols\" is not sent, all isolated assets will be returned. - If \"symbols\" is sent, only the isolated assets of the sent symbols will be returned.  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**symbols** | Option<**String**> | Max 5 symbols can be sent; separated by ',' |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::IsolatedMarginAccountInfo**](isolatedMarginAccountInfo.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_isolated_account_limit_get

> models::SapiV1MarginIsolatedAccountLimitGet200Response sapi_v1_margin_isolated_account_limit_get(timestamp, signature, recv_window)
Query Enabled Isolated Margin Account Limit (USER_DATA)

Query enabled isolated margin account limit.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MarginIsolatedAccountLimitGet200Response**](_sapi_v1_margin_isolated_accountLimit_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_isolated_account_post

> models::SapiV1MarginIsolatedAccountPost200Response sapi_v1_margin_isolated_account_post(symbol, timestamp, signature, recv_window)
Enable Isolated Margin Account (TRADE)

Enable isolated margin account for a specific symbol.  Weight(UID): 300

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MarginIsolatedAccountPost200Response**](_sapi_v1_margin_isolated_account_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_isolated_all_pairs_get

> Vec<models::SapiV1MarginIsolatedAllPairsGet200ResponseInner> sapi_v1_margin_isolated_all_pairs_get(symbol, timestamp, signature, recv_window)
Get All Isolated Margin Symbol(USER_DATA)

Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1MarginIsolatedAllPairsGet200ResponseInner>**](_sapi_v1_margin_isolated_allPairs_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_isolated_margin_data_get

> Vec<models::SapiV1MarginIsolatedMarginDataGet200ResponseInner> sapi_v1_margin_isolated_margin_data_get(timestamp, signature, vip_level, symbol, recv_window)
Query Isolated Margin Fee Data (USER_DATA)

Get isolated margin fee data collection with any vip level or user's current specific data as https://www.binance.com/en/margin-fee  Weight(IP): 1 when a single is specified; 10 when the symbol parameter is omitted

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**vip_level** | Option<**i32**> | Defaults to user's vip level |  |
**symbol** | Option<**String**> | Trading symbol, e.g. BNBUSDT |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1MarginIsolatedMarginDataGet200ResponseInner>**](_sapi_v1_margin_isolatedMarginData_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_isolated_margin_tier_get

> Vec<models::SapiV1MarginIsolatedMarginTierGet200ResponseInner> sapi_v1_margin_isolated_margin_tier_get(symbol, timestamp, signature, tier, recv_window)
Query Isolated Margin Tier Data (USER_DATA)

Get isolated margin tier data collection with any tier as https://www.binance.com/en/margin-data  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**tier** | Option<**String**> | All margin tier data will be returned if tier is omitted |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1MarginIsolatedMarginTierGet200ResponseInner>**](_sapi_v1_margin_isolatedMarginTier_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_leverage_bracket_get

> Vec<models::SapiV1MarginLeverageBracketGet200ResponseInner> sapi_v1_margin_leverage_bracket_get()
Query Liability Coin Leverage Bracket in Cross Margin Pro Mode (MARKET_DATA)

Liability Coin Leverage Bracket in Cross Margin Pro Mode  Weight(IP): 1

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::SapiV1MarginLeverageBracketGet200ResponseInner>**](_sapi_v1_margin_leverageBracket_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_manual_liquidation_post

> Vec<models::SapiV1MarginExchangeSmallLiabilityGet200ResponseInner> sapi_v1_margin_manual_liquidation_post(r#type, timestamp, signature, symbol)
Margin manual liquidation(MARGIN)

Margin manual liquidation  Weight(UID): 3000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**symbol** | Option<**String**> |  |  |

### Return type

[**Vec<models::SapiV1MarginExchangeSmallLiabilityGet200ResponseInner>**](_sapi_v1_margin_exchange_small_liability_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_max_borrowable_get

> models::SapiV1MarginMaxBorrowableGet200Response sapi_v1_margin_max_borrowable_get(asset, timestamp, signature, isolated_symbol, recv_window)
Query Max Borrow (USER_DATA)

- If `isolatedSymbol` is not sent, crossed margin data will be sent. - `borrowLimit` is also available from https://www.binance.com/en/margin-fee  Weight(IP): 50

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**isolated_symbol** | Option<**String**> | Isolated symbol |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MarginMaxBorrowableGet200Response**](_sapi_v1_margin_maxBorrowable_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_max_leverage_post

> models::SapiV1MarginMaxLeveragePost200Response sapi_v1_margin_max_leverage_post(max_leverage, timestamp, signature, recv_window)
Adjust cross margin max leverage (USER_DATA)

Adjust cross margin max leverage  Weight(UID): 3000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max_leverage** | **i32** | Can only adjust 3 or 5 | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MarginMaxLeveragePost200Response**](_sapi_v1_margin_max_leverage_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_max_transferable_get

> models::SapiV1MarginMaxTransferableGet200Response sapi_v1_margin_max_transferable_get(asset, timestamp, signature, isolated_symbol, recv_window)
Query Max Transfer-Out Amount (USER_DATA)

- If `isolatedSymbol` is not sent, crossed margin data will be sent.  Weight(IP): 50

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**isolated_symbol** | Option<**String**> | Isolated symbol |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MarginMaxTransferableGet200Response**](_sapi_v1_margin_maxTransferable_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_my_trades_get

> Vec<models::MarginTrade> sapi_v1_margin_my_trades_get(symbol, timestamp, signature, is_isolated, start_time, end_time, from_id, limit, recv_window)
Query Margin Account's Trade List (USER_DATA)

- If `fromId` is set, it will get orders >= that `fromId`. Otherwise most recent trades are returned.  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**from_id** | Option<**i64**> | Trade id to fetch from. Default gets most recent trades. |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::MarginTrade>**](marginTrade.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_next_hourly_interest_rate_get

> Vec<models::SapiV1MarginNextHourlyInterestRateGet200ResponseInner> sapi_v1_margin_next_hourly_interest_rate_get(timestamp, signature, assets, is_isolated, recv_window)
Get a future hourly interest rate (USER_DATA)

Get user the next hourly estimate interest  Weight(UID): 100

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**assets** | Option<**String**> | List of assets, separated by commas, up to 20 |  |
**is_isolated** | Option<**String**> | for isolated margin or not, \"TRUE\", \"FALSE\" |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1MarginNextHourlyInterestRateGet200ResponseInner>**](_sapi_v1_margin_next_hourly_interest_rate_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_open_order_list_get

> Vec<models::SapiV1MarginOpenOrderListGet200ResponseInner> sapi_v1_margin_open_order_list_get(timestamp, signature, is_isolated, symbol, recv_window)
Query Margin Account's Open OCO (USER_DATA)

Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**symbol** | Option<**String**> | Mandatory for isolated margin, not supported for cross margin |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1MarginOpenOrderListGet200ResponseInner>**](_sapi_v1_margin_openOrderList_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_open_orders_delete

> Vec<models::SapiV1MarginOpenOrdersDelete200ResponseInner> sapi_v1_margin_open_orders_delete(symbol, timestamp, signature, is_isolated, recv_window)
Margin Account Cancel all Open Orders on a Symbol (TRADE)

- Cancels all active orders on a symbol for margin account. - This includes OCO orders.  Weight(IP): 1 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1MarginOpenOrdersDelete200ResponseInner>**](_sapi_v1_margin_openOrders_delete_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_open_orders_get

> Vec<models::MarginOrderDetail> sapi_v1_margin_open_orders_get(timestamp, signature, symbol, is_isolated, recv_window)
Query Margin Account's Open Orders (USER_DATA)

- If the `symbol` is not sent, orders for all symbols will be returned in an array. - When all symbols are returned, the number of requests counted against the rate limiter is equal to the number of symbols currently trading on the exchange - If isIsolated =\"TRUE\", symbol must be sent.  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**symbol** | Option<**String**> | Trading symbol, e.g. BNBUSDT |  |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::MarginOrderDetail>**](marginOrderDetail.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_order_delete

> models::MarginOrder sapi_v1_margin_order_delete(symbol, timestamp, signature, is_isolated, order_id, orig_client_order_id, new_client_order_id, recv_window)
Margin Account Cancel Order (TRADE)

Cancel an active order for margin account.  Either `orderId` or `origClientOrderId` must be sent.  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**order_id** | Option<**i64**> | Order id |  |
**orig_client_order_id** | Option<**String**> | Order id from client |  |
**new_client_order_id** | Option<**String**> | Used to uniquely identify this cancel. Automatically generated by default |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::MarginOrder**](marginOrder.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_order_get

> models::MarginOrderDetail sapi_v1_margin_order_get(symbol, timestamp, signature, is_isolated, order_id, orig_client_order_id, recv_window)
Query Margin Account's Order (USER_DATA)

- Either `orderId` or `origClientOrderId` must be sent. - For some historical orders `cummulativeQuoteQty` will be < 0, meaning the data is not available at this time.  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**order_id** | Option<**i64**> | Order id |  |
**orig_client_order_id** | Option<**String**> | Order id from client |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::MarginOrderDetail**](marginOrderDetail.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_order_list_delete

> models::MarginOcoOrder sapi_v1_margin_order_list_delete(symbol, timestamp, signature, is_isolated, order_list_id, list_client_order_id, new_client_order_id, recv_window)
Margin Account Cancel OCO (TRADE)

Cancel an entire Order List for a margin account  - Canceling an individual leg will cancel the entire OCO - Either `orderListId` or `listClientOrderId` must be provided  Weight(UID): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**order_list_id** | Option<**i64**> | Order list id |  |
**list_client_order_id** | Option<**String**> | A unique Id for the entire orderList |  |
**new_client_order_id** | Option<**String**> | Used to uniquely identify this cancel. Automatically generated by default |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::MarginOcoOrder**](marginOcoOrder.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_order_list_get

> models::SapiV1MarginOrderListGet200Response sapi_v1_margin_order_list_get(timestamp, signature, is_isolated, symbol, order_list_id, orig_client_order_id, recv_window)
Query Margin Account's OCO (USER_DATA)

Retrieves a specific OCO based on provided optional parameters  - Either `orderListId` or `origClientOrderId` must be provided  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**symbol** | Option<**String**> | Mandatory for isolated margin, not supported for cross margin |  |
**order_list_id** | Option<**i64**> | Order list id |  |
**orig_client_order_id** | Option<**String**> | Order id from client |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MarginOrderListGet200Response**](_sapi_v1_margin_orderList_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_order_oco_post

> models::SapiV1MarginOrderOcoPost200Response sapi_v1_margin_order_oco_post(symbol, side, quantity, price, stop_price, timestamp, signature, is_isolated, list_client_order_id, limit_client_order_id, limit_iceberg_qty, stop_client_order_id, stop_limit_price, stop_iceberg_qty, stop_limit_time_in_force, new_order_resp_type, side_effect_type, self_trade_prevention_mode, recv_window)
Margin Account New OCO (TRADE)

Send in a new OCO for a margin account  - Price Restrictions:   - SELL: Limit Price > Last Price > Stop Price   - BUY: Limit Price < Last Price < Stop Price - Quantity Restrictions:   - Both legs must have the same quantity   - ICEBERG quantities however do not have to be the same. - Order Rate Limit   - OCO counts as 2 orders against the order rate limit.  Weight(UID): 6

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**side** | **String** |  | [required] |
**quantity** | **f64** |  | [required] |
**price** | **f64** | Order price | [required] |
**stop_price** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**list_client_order_id** | Option<**String**> | A unique Id for the entire orderList |  |
**limit_client_order_id** | Option<**String**> | A unique Id for the limit order |  |
**limit_iceberg_qty** | Option<**f64**> |  |  |
**stop_client_order_id** | Option<**String**> | A unique Id for the stop loss/stop loss limit leg |  |
**stop_limit_price** | Option<**f64**> | If provided, stopLimitTimeInForce is required. |  |
**stop_iceberg_qty** | Option<**f64**> |  |  |
**stop_limit_time_in_force** | Option<**String**> |  |  |
**new_order_resp_type** | Option<**String**> | Set the response JSON. |  |
**side_effect_type** | Option<**String**> | Default `NO_SIDE_EFFECT` |  |
**self_trade_prevention_mode** | Option<**String**> | The allowed enums is dependent on what is configured on the symbol. The possible supported values are EXPIRE_TAKER, EXPIRE_MAKER, EXPIRE_BOTH, NONE. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MarginOrderOcoPost200Response**](_sapi_v1_margin_order_oco_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_order_post

> models::SapiV1MarginOrderPost200Response sapi_v1_margin_order_post(symbol, side, r#type, quantity, auto_repay_at_cancel, timestamp, signature, is_isolated, quote_order_qty, price, stop_price, new_client_order_id, iceberg_qty, new_order_resp_type, side_effect_type, time_in_force, self_trade_prevention_mode, recv_window)
Margin Account New Order (TRADE)

Post a new order for margin account.  Weight(UID): 6

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**side** | **String** |  | [required] |
**r#type** | **String** | Order type | [required] |
**quantity** | **f64** |  | [required] |
**auto_repay_at_cancel** | **bool** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**quote_order_qty** | Option<**f64**> | Quote quantity |  |
**price** | Option<**f64**> | Order price |  |
**stop_price** | Option<**f64**> | Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and TAKE_PROFIT_LIMIT orders. |  |
**new_client_order_id** | Option<**String**> | Used to uniquely identify this cancel. Automatically generated by default |  |
**iceberg_qty** | Option<**f64**> | Used with LIMIT, STOP_LOSS_LIMIT, and TAKE_PROFIT_LIMIT to create an iceberg order. |  |
**new_order_resp_type** | Option<**String**> | Set the response JSON. |  |
**side_effect_type** | Option<**String**> | Default `NO_SIDE_EFFECT` |  |
**time_in_force** | Option<**String**> | Order time in force |  |
**self_trade_prevention_mode** | Option<**String**> | The allowed enums is dependent on what is configured on the symbol. The possible supported values are EXPIRE_TAKER, EXPIRE_MAKER, EXPIRE_BOTH, NONE. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MarginOrderPost200Response**](_sapi_v1_margin_order_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_price_index_get

> models::SapiV1MarginPriceIndexGet200Response sapi_v1_margin_price_index_get(symbol)
Query Margin PriceIndex (MARKET_DATA)

Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |

### Return type

[**models::SapiV1MarginPriceIndexGet200Response**](_sapi_v1_margin_priceIndex_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_rate_limit_order_get

> Vec<models::SapiV1MarginRateLimitOrderGet200ResponseInner> sapi_v1_margin_rate_limit_order_get(timestamp, signature, is_isolated, symbol, recv_window)
Query Current Margin Order Count Usage (TRADE)

Displays the user's current margin order count usage for all intervals.  Weight(IP): 20

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**symbol** | Option<**String**> | isolated symbol, mandatory for isolated margin |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1MarginRateLimitOrderGet200ResponseInner>**](_sapi_v1_margin_rateLimit_order_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_trade_coeff_get

> models::SapiV1MarginTradeCoeffGet200Response sapi_v1_margin_trade_coeff_get(email, timestamp, signature, recv_window)
Get Summary of Margin account (USER_DATA)

Get personal margin level information  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email Address | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MarginTradeCoeffGet200Response**](_sapi_v1_margin_tradeCoeff_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_transfer_get

> models::SapiV1MarginTransferGet200Response sapi_v1_margin_transfer_get(timestamp, signature, asset, r#type, start_time, end_time, current, size, isolated_symbol, recv_window)
Get Cross Margin Transfer History (USER_DATA)

- Response in descending order - Returns data for last 7 days by default - Set `archived` to `true` to query data from 6 months ago  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> |  |  |
**r#type** | Option<**String**> |  |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**isolated_symbol** | Option<**String**> | Isolated symbol |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MarginTransferGet200Response**](_sapi_v1_margin_transfer_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

