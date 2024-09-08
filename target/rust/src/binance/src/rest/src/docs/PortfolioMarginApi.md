# \PortfolioMarginApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_portfolio_account_get**](PortfolioMarginApi.md#sapi_v1_portfolio_account_get) | **GET** /sapi/v1/portfolio/account | Portfolio Margin Account (USER_DATA)
[**sapi_v1_portfolio_asset_collection_post**](PortfolioMarginApi.md#sapi_v1_portfolio_asset_collection_post) | **POST** /sapi/v1/portfolio/asset-collection | Fund Collection by Asset (USER_DATA)
[**sapi_v1_portfolio_asset_index_price_get**](PortfolioMarginApi.md#sapi_v1_portfolio_asset_index_price_get) | **GET** /sapi/v1/portfolio/asset-index-price | Query Portfolio Margin Asset Index Price (MARKET_DATA)
[**sapi_v1_portfolio_auto_collection_post**](PortfolioMarginApi.md#sapi_v1_portfolio_auto_collection_post) | **POST** /sapi/v1/portfolio/auto-collection | Fund Auto-collection (USER_DATA)
[**sapi_v1_portfolio_bnb_transfer_post**](PortfolioMarginApi.md#sapi_v1_portfolio_bnb_transfer_post) | **POST** /sapi/v1/portfolio/bnb-transfer | BNB Transfer (USER_DATA)
[**sapi_v1_portfolio_collateral_rate_get**](PortfolioMarginApi.md#sapi_v1_portfolio_collateral_rate_get) | **GET** /sapi/v1/portfolio/collateralRate | Portfolio Margin Collateral Rate (MARKET_DATA)
[**sapi_v1_portfolio_interest_history_get**](PortfolioMarginApi.md#sapi_v1_portfolio_interest_history_get) | **GET** /sapi/v1/portfolio/interest-history | Query Classic Portfolio Margin Negative Balance Interest History (USER_DATA)
[**sapi_v1_portfolio_margin_asset_leverage_get**](PortfolioMarginApi.md#sapi_v1_portfolio_margin_asset_leverage_get) | **GET** /sapi/v1/portfolio/margin-asset-leverage | Get Portfolio Margin Asset Leverage (USER_DATA)
[**sapi_v1_portfolio_pm_loan_get**](PortfolioMarginApi.md#sapi_v1_portfolio_pm_loan_get) | **GET** /sapi/v1/portfolio/pmLoan | Portfolio Margin Bankruptcy Loan Amount (USER_DATA)
[**sapi_v1_portfolio_repay_futures_negative_balance_post**](PortfolioMarginApi.md#sapi_v1_portfolio_repay_futures_negative_balance_post) | **POST** /sapi/v1/portfolio/repay-futures-negative-balance | Repay futures Negative Balance (USER_DATA)
[**sapi_v1_portfolio_repay_futures_switch_get**](PortfolioMarginApi.md#sapi_v1_portfolio_repay_futures_switch_get) | **GET** /sapi/v1/portfolio/repay-futures-switch | Get Auto-repay-futures Status (USER_DATA)
[**sapi_v1_portfolio_repay_futures_switch_post**](PortfolioMarginApi.md#sapi_v1_portfolio_repay_futures_switch_post) | **POST** /sapi/v1/portfolio/repay-futures-switch | Change Auto-repay-futures Status (USER_DATA)
[**sapi_v1_portfolio_repay_post**](PortfolioMarginApi.md#sapi_v1_portfolio_repay_post) | **POST** /sapi/v1/portfolio/repay | Portfolio Margin Bankruptcy Loan Repay (USER_DATA)



## sapi_v1_portfolio_account_get

> models::SapiV1PortfolioAccountGet200Response sapi_v1_portfolio_account_get(timestamp, signature, recv_window)
Portfolio Margin Account (USER_DATA)

Get the account info  'Weight(IP): 1'

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1PortfolioAccountGet200Response**](_sapi_v1_portfolio_account_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_portfolio_asset_collection_post

> models::SapiV1PortfolioAutoCollectionPost200Response sapi_v1_portfolio_asset_collection_post(asset, timestamp, signature, recv_window)
Fund Collection by Asset (USER_DATA)

Transfers specific asset from Futures Account to Margin account  Weight(IP): 60

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1PortfolioAutoCollectionPost200Response**](_sapi_v1_portfolio_auto_collection_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_portfolio_asset_index_price_get

> Vec<models::SapiV1PortfolioAssetIndexPriceGet200ResponseInner> sapi_v1_portfolio_asset_index_price_get(asset)
Query Portfolio Margin Asset Index Price (MARKET_DATA)

Query Portfolio Margin Asset Index Price  Weight(IP): - 1 if send asset - 50 if not send asset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | Option<**String**> |  |  |

### Return type

[**Vec<models::SapiV1PortfolioAssetIndexPriceGet200ResponseInner>**](_sapi_v1_portfolio_asset_index_price_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_portfolio_auto_collection_post

> models::SapiV1PortfolioAutoCollectionPost200Response sapi_v1_portfolio_auto_collection_post(timestamp, signature, recv_window)
Fund Auto-collection (USER_DATA)

Transfers all assets from Futures Account to Margin account  Weight(IP): 1500

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1PortfolioAutoCollectionPost200Response**](_sapi_v1_portfolio_auto_collection_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_portfolio_bnb_transfer_post

> models::SapiV1MarginBorrowRepayPost200Response sapi_v1_portfolio_bnb_transfer_post(transfer_side, amount, timestamp, signature, recv_window)
BNB Transfer (USER_DATA)

BNB transfer can be between Margin Account and USDM Account  Weight(IP): 1500

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_side** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
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


## sapi_v1_portfolio_collateral_rate_get

> Vec<models::SapiV1PortfolioCollateralRateGet200ResponseInner> sapi_v1_portfolio_collateral_rate_get()
Portfolio Margin Collateral Rate (MARKET_DATA)

Portfolio Margin Collateral Rate.  Weight(IP): 50

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::SapiV1PortfolioCollateralRateGet200ResponseInner>**](_sapi_v1_portfolio_collateralRate_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_portfolio_interest_history_get

> Vec<models::SapiV1PortfolioInterestHistoryGet200ResponseInner> sapi_v1_portfolio_interest_history_get(asset, timestamp, signature, start_time, end_time, size, recv_window)
Query Classic Portfolio Margin Negative Balance Interest History (USER_DATA)

Query interest history of negative balance for portfolio margin.  Weight(IP): 50

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1PortfolioInterestHistoryGet200ResponseInner>**](_sapi_v1_portfolio_interest_history_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_portfolio_margin_asset_leverage_get

> Vec<models::SapiV1PortfolioMarginAssetLeverageGet200ResponseInner> sapi_v1_portfolio_margin_asset_leverage_get()
Get Portfolio Margin Asset Leverage (USER_DATA)

Weight(IP): 50

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::SapiV1PortfolioMarginAssetLeverageGet200ResponseInner>**](_sapi_v1_portfolio_margin_asset_leverage_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_portfolio_pm_loan_get

> models::SapiV1PortfolioPmLoanGet200Response sapi_v1_portfolio_pm_loan_get(timestamp, signature, recv_window)
Portfolio Margin Bankruptcy Loan Amount (USER_DATA)

Query Portfolio Margin Bankruptcy Loan Amount.  Weight(UID): 500

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1PortfolioPmLoanGet200Response**](_sapi_v1_portfolio_pmLoan_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_portfolio_repay_futures_negative_balance_post

> models::SapiV1PortfolioAutoCollectionPost200Response sapi_v1_portfolio_repay_futures_negative_balance_post(timestamp, signature, recv_window)
Repay futures Negative Balance (USER_DATA)

Repay futures Negative Balance  Weight(IP): 1500

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1PortfolioAutoCollectionPost200Response**](_sapi_v1_portfolio_auto_collection_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_portfolio_repay_futures_switch_get

> models::SapiV1PortfolioRepayFuturesSwitchGet200Response sapi_v1_portfolio_repay_futures_switch_get(timestamp, signature, recv_window)
Get Auto-repay-futures Status (USER_DATA)

Query Auto-repay-futures Status  Weight(IP): 30

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1PortfolioRepayFuturesSwitchGet200Response**](_sapi_v1_portfolio_repay_futures_switch_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_portfolio_repay_futures_switch_post

> models::SapiV1PortfolioAutoCollectionPost200Response sapi_v1_portfolio_repay_futures_switch_post(auto_repay, timestamp, signature, recv_window)
Change Auto-repay-futures Status (USER_DATA)

Change Auto-repay-futures Status  Weight(IP): 1500

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auto_repay** | **bool** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1PortfolioAutoCollectionPost200Response**](_sapi_v1_portfolio_auto_collection_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_portfolio_repay_post

> models::SapiV1PortfolioRepayPost200Response sapi_v1_portfolio_repay_post(timestamp, signature, from, recv_window)
Portfolio Margin Bankruptcy Loan Repay (USER_DATA)

Repay Portfolio Margin Bankruptcy Loan.  Weight(UID): 3000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**from** | Option<**String**> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1PortfolioRepayPost200Response**](_sapi_v1_portfolio_repay_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

