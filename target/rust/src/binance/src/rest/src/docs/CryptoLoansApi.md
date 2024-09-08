# \CryptoLoansApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_loan_adjust_ltv_post**](CryptoLoansApi.md#sapi_v1_loan_adjust_ltv_post) | **POST** /sapi/v1/loan/adjust/ltv | Crypto Loan Adjust LTV (TRADE)
[**sapi_v1_loan_borrow_history_get**](CryptoLoansApi.md#sapi_v1_loan_borrow_history_get) | **GET** /sapi/v1/loan/borrow/history | Get Crypto Loans Borrow History (USER_DATA)
[**sapi_v1_loan_borrow_post**](CryptoLoansApi.md#sapi_v1_loan_borrow_post) | **POST** /sapi/v1/loan/borrow | Crypto Loan Borrow (TRADE)
[**sapi_v1_loan_collateral_data_get**](CryptoLoansApi.md#sapi_v1_loan_collateral_data_get) | **GET** /sapi/v1/loan/collateral/data | Get Collateral Assets Data (USER_DATA)
[**sapi_v1_loan_customize_margin_call_post**](CryptoLoansApi.md#sapi_v1_loan_customize_margin_call_post) | **POST** /sapi/v1/loan/customize/margin_call | Crypto Loan Customize Margin Call (TRADE)
[**sapi_v1_loan_income_get**](CryptoLoansApi.md#sapi_v1_loan_income_get) | **GET** /sapi/v1/loan/income | Get Crypto Loans Income History (USER_DATA)
[**sapi_v1_loan_loanable_data_get**](CryptoLoansApi.md#sapi_v1_loan_loanable_data_get) | **GET** /sapi/v1/loan/loanable/data | Get Loanable Assets Data (USER_DATA)
[**sapi_v1_loan_ltv_adjustment_history_get**](CryptoLoansApi.md#sapi_v1_loan_ltv_adjustment_history_get) | **GET** /sapi/v1/loan/ltv/adjustment/history | Get Loan LTV Adjustment History (USER_DATA)
[**sapi_v1_loan_ongoing_orders_get**](CryptoLoansApi.md#sapi_v1_loan_ongoing_orders_get) | **GET** /sapi/v1/loan/ongoing/orders | Get Loan Ongoing Orders (USER_DATA)
[**sapi_v1_loan_repay_collateral_rate_get**](CryptoLoansApi.md#sapi_v1_loan_repay_collateral_rate_get) | **GET** /sapi/v1/loan/repay/collateral/rate | Check Collateral Repay Rate (USER_DATA)
[**sapi_v1_loan_repay_history_get**](CryptoLoansApi.md#sapi_v1_loan_repay_history_get) | **GET** /sapi/v1/loan/repay/history | Get Loan Repayment History (USER_DATA)
[**sapi_v1_loan_repay_post**](CryptoLoansApi.md#sapi_v1_loan_repay_post) | **POST** /sapi/v1/loan/repay | Crypto Loan Repay (TRADE)
[**sapi_v2_loan_flexible_adjust_ltv_post**](CryptoLoansApi.md#sapi_v2_loan_flexible_adjust_ltv_post) | **POST** /sapi/v2/loan/flexible/adjust/ltv | Adjust LTV - Flexible Loan Adjust LTV (TRADE)
[**sapi_v2_loan_flexible_borrow_history_get**](CryptoLoansApi.md#sapi_v2_loan_flexible_borrow_history_get) | **GET** /sapi/v2/loan/flexible/borrow/history | Borrow - Get Flexible Loan Borrow History (USER_DATA)
[**sapi_v2_loan_flexible_borrow_post**](CryptoLoansApi.md#sapi_v2_loan_flexible_borrow_post) | **POST** /sapi/v2/loan/flexible/borrow | Borrow - Flexible Loan Borrow (TRADE)
[**sapi_v2_loan_flexible_collateral_data_get**](CryptoLoansApi.md#sapi_v2_loan_flexible_collateral_data_get) | **GET** /sapi/v2/loan/flexible/collateral/data | Get Flexible Loan Collateral Assets Data (USER_DATA)
[**sapi_v2_loan_flexible_loanable_data_get**](CryptoLoansApi.md#sapi_v2_loan_flexible_loanable_data_get) | **GET** /sapi/v2/loan/flexible/loanable/data | Get Flexible Loan Assets Data (USER_DATA)
[**sapi_v2_loan_flexible_ltv_adjustment_history_get**](CryptoLoansApi.md#sapi_v2_loan_flexible_ltv_adjustment_history_get) | **GET** /sapi/v2/loan/flexible/ltv/adjustment/history | Adjust LTV - Get Flexible Loan LTV Adjustment History (USER_DATA)
[**sapi_v2_loan_flexible_ongoing_orders_get**](CryptoLoansApi.md#sapi_v2_loan_flexible_ongoing_orders_get) | **GET** /sapi/v2/loan/flexible/ongoing/orders | Borrow - Get Flexible Loan Ongoing Orders (USER_DATA)
[**sapi_v2_loan_flexible_repay_history_get**](CryptoLoansApi.md#sapi_v2_loan_flexible_repay_history_get) | **GET** /sapi/v2/loan/flexible/repay/history | Repay - Get Flexible Loan Repayment History (USER_DATA)
[**sapi_v2_loan_flexible_repay_post**](CryptoLoansApi.md#sapi_v2_loan_flexible_repay_post) | **POST** /sapi/v2/loan/flexible/repay | Repay - Flexible Loan Repay (TRADE)



## sapi_v1_loan_adjust_ltv_post

> models::SapiV1LoanAdjustLtvPost200Response sapi_v1_loan_adjust_ltv_post(order_id, amount, direction, timestamp, signature, recv_window)
Crypto Loan Adjust LTV (TRADE)

Weight(UID): 6000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **i64** | Order ID | [required] |
**amount** | **f64** | Amount | [required] |
**direction** | **String** | 'ADDITIONAL', 'REDUCED' | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LoanAdjustLtvPost200Response**](_sapi_v1_loan_adjust_ltv_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_borrow_history_get

> models::SapiV1LoanBorrowHistoryGet200Response sapi_v1_loan_borrow_history_get(timestamp, signature, order_id, loan_coin, collateral_coin, start_time, end_time, current, limit, recv_window)
Get Crypto Loans Borrow History (USER_DATA)

- If startTime and endTime are not sent, the recent 90-day data will be returned. - The max interval between startTime and endTime is 180 days.  Weight(IP): 400

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**order_id** | Option<**i64**> | orderId in POST /sapi/v1/loan/borrow |  |
**loan_coin** | Option<**String**> | Coin loaned |  |
**collateral_coin** | Option<**String**> | Coin used as collateral |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**limit** | Option<**i64**> | default 10, max 100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LoanBorrowHistoryGet200Response**](_sapi_v1_loan_borrow_history_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_borrow_post

> models::SapiV1LoanBorrowPost200Response sapi_v1_loan_borrow_post(loan_coin, collateral_coin, loan_term, timestamp, signature, loan_amount, collateral_amount, recv_window)
Crypto Loan Borrow (TRADE)

Weight(UID): 6000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**loan_coin** | **String** | Coin loaned | [required] |
**collateral_coin** | **String** | Coin used as collateral | [required] |
**loan_term** | **i32** | 7/14/30/90/180 days | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**loan_amount** | Option<**f32**> | Loan amount |  |
**collateral_amount** | Option<**f32**> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LoanBorrowPost200Response**](_sapi_v1_loan_borrow_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_collateral_data_get

> models::SapiV1LoanCollateralDataGet200Response sapi_v1_loan_collateral_data_get(timestamp, signature, collateral_coin, vip_level, recv_window)
Get Collateral Assets Data (USER_DATA)

Get LTV information and collateral limit of collateral assets. The collateral limit is shown in USD value.  Weight(IP): 400

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**collateral_coin** | Option<**String**> | Coin used as collateral |  |
**vip_level** | Option<**i32**> | Defaults to user's vip level |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LoanCollateralDataGet200Response**](_sapi_v1_loan_collateral_data_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_customize_margin_call_post

> models::SapiV1LoanCustomizeMarginCallPost200Response sapi_v1_loan_customize_margin_call_post(margin_call, timestamp, signature, order_id, collateral_coin, recv_window)
Crypto Loan Customize Margin Call (TRADE)

Customize margin call for ongoing orders only.  Weight(UID): 6000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**margin_call** | **f32** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**order_id** | Option<**i64**> | Mandatory when collateralCoin is empty. Send either orderId or collateralCoin, if both parameters are sent, take orderId only. |  |
**collateral_coin** | Option<**String**> | Coin used as collateral |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LoanCustomizeMarginCallPost200Response**](_sapi_v1_loan_customize_margin_call_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_income_get

> Vec<models::SapiV1LoanIncomeGet200ResponseInner> sapi_v1_loan_income_get(timestamp, signature, asset, r#type, start_time, end_time, limit, recv_window)
Get Crypto Loans Income History (USER_DATA)

- If startTime and endTime are not sent, the recent 7-day data will be returned. - The max interval between startTime and endTime is 30 days.  Weight(UID): 6000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> |  |  |
**r#type** | Option<**String**> | All types will be returned by default.   * `borrowIn`   * `collateralSpent`   * `repayAmount`   * `collateralReturn` - Collateral return after repayment   * `addCollateral`   * `removeCollateral`   * `collateralReturnAfterLiquidation` |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i32**> | default 20, max 100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1LoanIncomeGet200ResponseInner>**](_sapi_v1_loan_income_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_loanable_data_get

> models::SapiV1LoanLoanableDataGet200Response sapi_v1_loan_loanable_data_get(timestamp, signature, loan_coin, vip_level, recv_window)
Get Loanable Assets Data (USER_DATA)

Get interest rate and borrow limit of loanable assets. The borrow limit is shown in USD value.  Weight(IP): 400

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**loan_coin** | Option<**String**> | Coin loaned |  |
**vip_level** | Option<**i32**> | Defaults to user's vip level |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LoanLoanableDataGet200Response**](_sapi_v1_loan_loanable_data_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_ltv_adjustment_history_get

> models::SapiV1LoanLtvAdjustmentHistoryGet200Response sapi_v1_loan_ltv_adjustment_history_get(timestamp, signature, order_id, loan_coin, collateral_coin, start_time, end_time, current, limit, recv_window)
Get Loan LTV Adjustment History (USER_DATA)

If startTime and endTime are not sent, the recent 90-day data will be returned. The max interval between startTime and endTime is 180 days.  Weight(IP): 400

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**order_id** | Option<**i64**> | Order ID |  |
**loan_coin** | Option<**String**> | Coin loaned |  |
**collateral_coin** | Option<**String**> | Coin used as collateral |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**limit** | Option<**i64**> | default 10, max 100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LoanLtvAdjustmentHistoryGet200Response**](_sapi_v1_loan_ltv_adjustment_history_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_ongoing_orders_get

> models::SapiV1LoanOngoingOrdersGet200Response sapi_v1_loan_ongoing_orders_get(timestamp, signature, order_id, loan_coin, collateral_coin, current, limit, recv_window)
Get Loan Ongoing Orders (USER_DATA)

Weight(IP): 300

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**order_id** | Option<**i64**> | orderId in POST /sapi/v1/loan/borrow |  |
**loan_coin** | Option<**String**> | Coin loaned |  |
**collateral_coin** | Option<**String**> | Coin used as collateral |  |
**current** | Option<**i32**> | Current querying page. Start from 1; default:1, max:1000 |  |
**limit** | Option<**i64**> | default 10, max 100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LoanOngoingOrdersGet200Response**](_sapi_v1_loan_ongoing_orders_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_repay_collateral_rate_get

> models::SapiV1LoanRepayCollateralRateGet200Response sapi_v1_loan_repay_collateral_rate_get(loan_coin, collateral_coin, repay_amount, timestamp, signature, recv_window)
Check Collateral Repay Rate (USER_DATA)

Get the the rate of collateral coin / loan coin when using collateral repay, the rate will be valid within 8 second.  Weight(IP): 6000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**loan_coin** | **String** | Coin loaned | [required] |
**collateral_coin** | **String** | Coin used as collateral | [required] |
**repay_amount** | **f32** | repay amount of loanCoin | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LoanRepayCollateralRateGet200Response**](_sapi_v1_loan_repay_collateral_rate_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_repay_history_get

> models::SapiV1LoanRepayHistoryGet200Response sapi_v1_loan_repay_history_get(timestamp, signature, order_id, loan_coin, collateral_coin, start_time, end_time, current, limit, recv_window)
Get Loan Repayment History (USER_DATA)

If startTime and endTime are not sent, the recent 90-day data will be returned. The max interval between startTime and endTime is 180 days.  Weight(IP): 400

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**order_id** | Option<**i64**> | Order ID |  |
**loan_coin** | Option<**String**> | Coin loaned |  |
**collateral_coin** | Option<**String**> | Coin used as collateral |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**limit** | Option<**i64**> | default 10, max 100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LoanRepayHistoryGet200Response**](_sapi_v1_loan_repay_history_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_repay_post

> models::SapiV1LoanRepayPost200Response sapi_v1_loan_repay_post(order_id, amount, timestamp, signature, r#type, collateral_return, recv_window)
Crypto Loan Repay (TRADE)

Weight(UID): 6000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **i64** | Order ID | [required] |
**amount** | **f64** | Repayment Amount | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**r#type** | Option<**i32**> | Default: 1. 1 for 'repay with borrowed coin'; 2 for 'repay with collateral'. |  |
**collateral_return** | Option<**bool**> | Default: TRUE. TRUE: Return extra collateral to spot account; FALSE: Keep extra collateral in the order. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LoanRepayPost200Response**](_sapi_v1_loan_repay_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v2_loan_flexible_adjust_ltv_post

> models::SapiV2LoanFlexibleAdjustLtvPost200Response sapi_v2_loan_flexible_adjust_ltv_post(adjustment_amount, direction, timestamp, signature, loan_coin, collateral_coin, recv_window)
Adjust LTV - Flexible Loan Adjust LTV (TRADE)

- API Key needs Spot & Margin Trading permission for this endpoint  Weight(UID): 6000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**adjustment_amount** | **f32** |  | [required] |
**direction** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**loan_coin** | Option<**String**> | Coin loaned |  |
**collateral_coin** | Option<**String**> | Coin used as collateral |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV2LoanFlexibleAdjustLtvPost200Response**](_sapi_v2_loan_flexible_adjust_ltv_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v2_loan_flexible_borrow_history_get

> models::SapiV2LoanFlexibleBorrowHistoryGet200Response sapi_v2_loan_flexible_borrow_history_get(timestamp, signature, loan_coin, collateral_coin, start_time, end_time, current, limit, recv_window)
Borrow - Get Flexible Loan Borrow History (USER_DATA)

- If startTime and endTime are not sent, the recent 90-day data will be returned. - The max interval between startTime and endTime is 180 days.  Weight(IP): 400

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**loan_coin** | Option<**String**> | Coin loaned |  |
**collateral_coin** | Option<**String**> | Coin used as collateral |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV2LoanFlexibleBorrowHistoryGet200Response**](_sapi_v2_loan_flexible_borrow_history_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v2_loan_flexible_borrow_post

> models::SapiV2LoanFlexibleBorrowPost200Response sapi_v2_loan_flexible_borrow_post(timestamp, signature, loan_coin, loan_amount, collateral_coin, collateral_amount, recv_window)
Borrow - Flexible Loan Borrow (TRADE)

- Only available for master account  Weight(UID): 6000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**loan_coin** | Option<**String**> | Coin loaned |  |
**loan_amount** | Option<**f32**> | Loan amount |  |
**collateral_coin** | Option<**String**> | Coin used as collateral |  |
**collateral_amount** | Option<**f32**> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV2LoanFlexibleBorrowPost200Response**](_sapi_v2_loan_flexible_borrow_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v2_loan_flexible_collateral_data_get

> models::SapiV2LoanFlexibleCollateralDataGet200Response sapi_v2_loan_flexible_collateral_data_get(timestamp, signature, collateral_coin, recv_window)
Get Flexible Loan Collateral Assets Data (USER_DATA)

Get LTV information and collateral limit of flexible loan's collateral assets. The collateral limit is shown in USD value.  Weight(IP): 400

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**collateral_coin** | Option<**String**> | Coin used as collateral |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV2LoanFlexibleCollateralDataGet200Response**](_sapi_v2_loan_flexible_collateral_data_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v2_loan_flexible_loanable_data_get

> models::SapiV2LoanFlexibleLoanableDataGet200Response sapi_v2_loan_flexible_loanable_data_get(timestamp, signature, loan_coin, recv_window)
Get Flexible Loan Assets Data (USER_DATA)

Get interest rate and borrow limit of flexible loanable assets. The borrow limit is shown in USD value.  Weight(IP): 400

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**loan_coin** | Option<**String**> | Coin loaned |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV2LoanFlexibleLoanableDataGet200Response**](_sapi_v2_loan_flexible_loanable_data_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v2_loan_flexible_ltv_adjustment_history_get

> models::SapiV2LoanFlexibleLtvAdjustmentHistoryGet200Response sapi_v2_loan_flexible_ltv_adjustment_history_get(timestamp, signature, loan_coin, collateral_coin, start_time, end_time, current, limit, recv_window)
Adjust LTV - Get Flexible Loan LTV Adjustment History (USER_DATA)

- If startTime and endTime are not sent, the recent 90-day data will be returned. - The max interval between startTime and endTime is 180 days.  Weight(IP): 400

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**loan_coin** | Option<**String**> | Coin loaned |  |
**collateral_coin** | Option<**String**> | Coin used as collateral |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV2LoanFlexibleLtvAdjustmentHistoryGet200Response**](_sapi_v2_loan_flexible_ltv_adjustment_history_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v2_loan_flexible_ongoing_orders_get

> models::SapiV2LoanFlexibleOngoingOrdersGet200Response sapi_v2_loan_flexible_ongoing_orders_get(timestamp, signature, loan_coin, collateral_coin, current, limit, recv_window)
Borrow - Get Flexible Loan Ongoing Orders (USER_DATA)

 Weight(IP): 300

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**loan_coin** | Option<**String**> | Coin loaned |  |
**collateral_coin** | Option<**String**> | Coin used as collateral |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV2LoanFlexibleOngoingOrdersGet200Response**](_sapi_v2_loan_flexible_ongoing_orders_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v2_loan_flexible_repay_history_get

> models::SapiV2LoanFlexibleRepayHistoryGet200Response sapi_v2_loan_flexible_repay_history_get(timestamp, signature, loan_coin, collateral_coin, start_time, end_time, current, limit, recv_window)
Repay - Get Flexible Loan Repayment History (USER_DATA)

- If startTime and endTime are not sent, the recent 90-day data will be returned. - The max interval between startTime and endTime is 180 days.  Weight(IP): 400

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**loan_coin** | Option<**String**> | Coin loaned |  |
**collateral_coin** | Option<**String**> | Coin used as collateral |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV2LoanFlexibleRepayHistoryGet200Response**](_sapi_v2_loan_flexible_repay_history_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v2_loan_flexible_repay_post

> models::SapiV2LoanFlexibleRepayPost200Response sapi_v2_loan_flexible_repay_post(repay_amount, timestamp, signature, loan_coin, collateral_coin, collateral_return, full_repayment, recv_window)
Repay - Flexible Loan Repay (TRADE)

- repayAmount is mandatory even fullRepayment = FALSE  Weight(IP): 6000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repay_amount** | **f32** | repay amount of loanCoin | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**loan_coin** | Option<**String**> | Coin loaned |  |
**collateral_coin** | Option<**String**> | Coin used as collateral |  |
**collateral_return** | Option<**bool**> | Default: TRUE.  TRUE: Return extra collateral to earn account;  FALSE: Keep extra collateral in the order, and lower LTV. |  |
**full_repayment** | Option<**bool**> | Default: FALSE. TRUE: Full repayment; FALSE: Partial repayment, based on loanAmount |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV2LoanFlexibleRepayPost200Response**](_sapi_v2_loan_flexible_repay_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

