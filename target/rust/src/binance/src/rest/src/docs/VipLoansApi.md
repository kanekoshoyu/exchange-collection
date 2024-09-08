# \VipLoansApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_loan_vip_borrow_post**](VipLoansApi.md#sapi_v1_loan_vip_borrow_post) | **POST** /sapi/v1/loan/vip/borrow | VIP Loan Borrow
[**sapi_v1_loan_vip_collateral_account_get**](VipLoansApi.md#sapi_v1_loan_vip_collateral_account_get) | **GET** /sapi/v1/loan/vip/collateral/account | Check Locked Value of VIP Collateral Account (USER_DATA)
[**sapi_v1_loan_vip_collateral_data_get**](VipLoansApi.md#sapi_v1_loan_vip_collateral_data_get) | **GET** /sapi/v1/loan/vip/collateral/data | Get Collateral Asset Data (USER_DATA)
[**sapi_v1_loan_vip_loanable_data_get**](VipLoansApi.md#sapi_v1_loan_vip_loanable_data_get) | **GET** /sapi/v1/loan/vip/loanable/data | Get Loanable Assets Data
[**sapi_v1_loan_vip_ongoing_orders_get**](VipLoansApi.md#sapi_v1_loan_vip_ongoing_orders_get) | **GET** /sapi/v1/loan/vip/ongoing/orders | Get VIP Loan Ongoing Orders (USER_DATA)
[**sapi_v1_loan_vip_renew_post**](VipLoansApi.md#sapi_v1_loan_vip_renew_post) | **POST** /sapi/v1/loan/vip/renew | VIP Loan Renew
[**sapi_v1_loan_vip_repay_history_get**](VipLoansApi.md#sapi_v1_loan_vip_repay_history_get) | **GET** /sapi/v1/loan/vip/repay/history | Get VIP Loan Repayment History (USER_DATA)
[**sapi_v1_loan_vip_repay_post**](VipLoansApi.md#sapi_v1_loan_vip_repay_post) | **POST** /sapi/v1/loan/vip/repay | VIP Loan Repay (TRADE)
[**sapi_v1_loan_vip_request_data_get**](VipLoansApi.md#sapi_v1_loan_vip_request_data_get) | **GET** /sapi/v1/loan/vip/request/data | Query Application Status (USER_DATA)
[**sapi_v1_loan_vip_request_interest_rate_get**](VipLoansApi.md#sapi_v1_loan_vip_request_interest_rate_get) | **GET** /sapi/v1/loan/vip/request/interestRate | Get Borrow Interest Rate (USER_DATA)



## sapi_v1_loan_vip_borrow_post

> models::SapiV1LoanVipBorrowPost200Response sapi_v1_loan_vip_borrow_post(loan_account_id, loan_amount, collateral_account_id, collateral_coin, is_flexible_rate, timestamp, signature, loan_coin, loan_term, recv_window)
VIP Loan Borrow

VIP loan is available for VIP users only.  Weight(UID): 6000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**loan_account_id** | **i64** |  | [required] |
**loan_amount** | **f32** |  | [required] |
**collateral_account_id** | **String** |  | [required] |
**collateral_coin** | **String** |  | [required] |
**is_flexible_rate** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**loan_coin** | Option<**String**> | Coin loaned |  |
**loan_term** | Option<**i32**> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LoanVipBorrowPost200Response**](_sapi_v1_loan_vip_borrow_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_vip_collateral_account_get

> models::SapiV1LoanVipCollateralAccountGet200Response sapi_v1_loan_vip_collateral_account_get(timestamp, signature, order_id, collateral_account_id, recv_window)
Check Locked Value of VIP Collateral Account (USER_DATA)

VIP loan is available for VIP users only.  Weight(IP): 6000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**order_id** | Option<**i64**> | Order id |  |
**collateral_account_id** | Option<**i64**> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LoanVipCollateralAccountGet200Response**](_sapi_v1_loan_vip_collateral_account_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_vip_collateral_data_get

> models::SapiV1LoanVipCollateralDataGet200Response sapi_v1_loan_vip_collateral_data_get(timestamp, signature, collateral_coin, recv_window)
Get Collateral Asset Data (USER_DATA)

Get collateral asset data.  Weight(IP): 400

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**collateral_coin** | Option<**String**> | Coin used as collateral |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LoanVipCollateralDataGet200Response**](_sapi_v1_loan_vip_collateral_data_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_vip_loanable_data_get

> models::SapiV1LoanVipLoanableDataGet200Response sapi_v1_loan_vip_loanable_data_get(timestamp, signature, loan_coin, vip_level, recv_window)
Get Loanable Assets Data

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

[**models::SapiV1LoanVipLoanableDataGet200Response**](_sapi_v1_loan_vip_loanable_data_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_vip_ongoing_orders_get

> models::SapiV1LoanVipOngoingOrdersGet200Response sapi_v1_loan_vip_ongoing_orders_get(timestamp, signature, order_id, collateral_account_id, loan_coin, collateral_coin, current, limit, recv_window)
Get VIP Loan Ongoing Orders (USER_DATA)

VIP loan is available for VIP users only.  Weight(IP): 400

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**order_id** | Option<**i64**> | Order id |  |
**collateral_account_id** | Option<**i64**> |  |  |
**loan_coin** | Option<**String**> | Coin loaned |  |
**collateral_coin** | Option<**String**> | Coin used as collateral |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**limit** | Option<**i32**> | Default 10; max 100. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LoanVipOngoingOrdersGet200Response**](_sapi_v1_loan_vip_ongoing_orders_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_vip_renew_post

> models::SapiV1LoanVipRenewPost200Response sapi_v1_loan_vip_renew_post(timestamp, signature, order_id, loan_term, recv_window)
VIP Loan Renew

VIP loan is available for VIP users only.  Weight(UID): 6000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**order_id** | Option<**i64**> | Order id |  |
**loan_term** | Option<**i32**> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LoanVipRenewPost200Response**](_sapi_v1_loan_vip_renew_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_vip_repay_history_get

> models::SapiV1LoanVipRepayHistoryGet200Response sapi_v1_loan_vip_repay_history_get(timestamp, signature, order_id, loan_coin, start_time, end_time, current, limit, recv_window)
Get VIP Loan Repayment History (USER_DATA)

VIP loan is available for VIP users only.  Weight(IP): 400

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**order_id** | Option<**i64**> | Order id |  |
**loan_coin** | Option<**String**> | Coin loaned |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**limit** | Option<**i32**> | Default 10; max 100. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LoanVipRepayHistoryGet200Response**](_sapi_v1_loan_vip_repay_history_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_vip_repay_post

> models::SapiV1LoanVipRepayPost200Response sapi_v1_loan_vip_repay_post(amount, timestamp, signature, order_id, recv_window)
VIP Loan Repay (TRADE)

VIP loan is available for VIP users only.  Weight(UID): 6000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**amount** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**order_id** | Option<**i64**> | Order id |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LoanVipRepayPost200Response**](_sapi_v1_loan_vip_repay_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_vip_request_data_get

> models::SapiV1LoanVipRequestDataGet200Response sapi_v1_loan_vip_request_data_get(timestamp, signature, current, limit, recv_window)
Query Application Status (USER_DATA)

Get Application Status  Weight(UID): 400

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LoanVipRequestDataGet200Response**](_sapi_v1_loan_vip_request_data_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_vip_request_interest_rate_get

> Vec<models::SapiV1LoanVipRequestInterestRateGet200ResponseInner> sapi_v1_loan_vip_request_interest_rate_get(timestamp, signature, loan_coin, recv_window)
Get Borrow Interest Rate (USER_DATA)

Get borrow interest rate.  Weight(UID): 400

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**loan_coin** | Option<**String**> | Max 10 assets, Multiple split by \",\" |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1LoanVipRequestInterestRateGet200ResponseInner>**](_sapi_v1_loan_vip_request_interestRate_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

