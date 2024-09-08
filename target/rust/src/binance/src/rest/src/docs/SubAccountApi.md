# \SubAccountApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_capital_deposit_sub_address_get**](SubAccountApi.md#sapi_v1_capital_deposit_sub_address_get) | **GET** /sapi/v1/capital/deposit/subAddress | Sub-account Spot Assets Summary (For Master Account)
[**sapi_v1_capital_deposit_sub_hisrec_get**](SubAccountApi.md#sapi_v1_capital_deposit_sub_hisrec_get) | **GET** /sapi/v1/capital/deposit/subHisrec | Sub-account Deposit History (For Master Account)
[**sapi_v1_managed_subaccount_account_snapshot_get**](SubAccountApi.md#sapi_v1_managed_subaccount_account_snapshot_get) | **GET** /sapi/v1/managed-subaccount/accountSnapshot | Managed sub-account snapshot (For Investor Master Account)
[**sapi_v1_managed_subaccount_asset_get**](SubAccountApi.md#sapi_v1_managed_subaccount_asset_get) | **GET** /sapi/v1/managed-subaccount/asset | Managed sub-account asset details(For Investor Master Account)
[**sapi_v1_managed_subaccount_deposit_address_get**](SubAccountApi.md#sapi_v1_managed_subaccount_deposit_address_get) | **GET** /sapi/v1/managed-subaccount/deposit/address | Get Managed Sub-account Deposit Address (For Investor Master Account)
[**sapi_v1_managed_subaccount_deposit_post**](SubAccountApi.md#sapi_v1_managed_subaccount_deposit_post) | **POST** /sapi/v1/managed-subaccount/deposit | Deposit assets into the managed sub-account(For Investor Master Account)
[**sapi_v1_managed_subaccount_fetch_future_asset_get**](SubAccountApi.md#sapi_v1_managed_subaccount_fetch_future_asset_get) | **GET** /sapi/v1/managed-subaccount/fetch-future-asset | Query Managed Sub-account Futures Asset Details (For Investor Master Account)
[**sapi_v1_managed_subaccount_info_get**](SubAccountApi.md#sapi_v1_managed_subaccount_info_get) | **GET** /sapi/v1/managed-subaccount/info | Query Managed Sub-account List (For Investor)
[**sapi_v1_managed_subaccount_margin_asset_get**](SubAccountApi.md#sapi_v1_managed_subaccount_margin_asset_get) | **GET** /sapi/v1/managed-subaccount/marginAsset | Query Managed Sub-account Margin Asset Details (For Investor Master Account)
[**sapi_v1_managed_subaccount_query_trans_log_for_investor_get**](SubAccountApi.md#sapi_v1_managed_subaccount_query_trans_log_for_investor_get) | **GET** /sapi/v1/managed-subaccount/queryTransLogForInvestor | Query Managed Sub Account Transfer Log (For Investor Master Account)
[**sapi_v1_managed_subaccount_query_trans_log_for_trade_parent_get**](SubAccountApi.md#sapi_v1_managed_subaccount_query_trans_log_for_trade_parent_get) | **GET** /sapi/v1/managed-subaccount/queryTransLogForTradeParent | Query Managed Sub Account Transfer Log (For Trading Team Master Account)
[**sapi_v1_managed_subaccount_query_trans_log_get**](SubAccountApi.md#sapi_v1_managed_subaccount_query_trans_log_get) | **GET** /sapi/v1/managed-subaccount/query-trans-log | Query Managed Sub Account Transfer Log (For Trading Team Sub Account)(USER_DATA)
[**sapi_v1_managed_subaccount_withdraw_post**](SubAccountApi.md#sapi_v1_managed_subaccount_withdraw_post) | **POST** /sapi/v1/managed-subaccount/withdraw | Withdrawl assets from the managed sub-account(For Investor Master Account)
[**sapi_v1_sub_account_blvt_enable_post**](SubAccountApi.md#sapi_v1_sub_account_blvt_enable_post) | **POST** /sapi/v1/sub-account/blvt/enable | Enable Leverage Token for Sub-account (For Master Account)
[**sapi_v1_sub_account_eoptions_enable_post**](SubAccountApi.md#sapi_v1_sub_account_eoptions_enable_post) | **POST** /sapi/v1/sub-account/eoptions/enable | Enable Options for Sub-account (For Master Account)(USER_DATA)
[**sapi_v1_sub_account_futures_account_get**](SubAccountApi.md#sapi_v1_sub_account_futures_account_get) | **GET** /sapi/v1/sub-account/futures/account | Detail on Sub-account's Futures Account (For Master Account)
[**sapi_v1_sub_account_futures_account_summary_get**](SubAccountApi.md#sapi_v1_sub_account_futures_account_summary_get) | **GET** /sapi/v1/sub-account/futures/accountSummary | Summary of Sub-account's Futures Account (For Master Account)
[**sapi_v1_sub_account_futures_enable_post**](SubAccountApi.md#sapi_v1_sub_account_futures_enable_post) | **POST** /sapi/v1/sub-account/futures/enable | Enable Futures for Sub-account (For Master Account)
[**sapi_v1_sub_account_futures_internal_transfer_get**](SubAccountApi.md#sapi_v1_sub_account_futures_internal_transfer_get) | **GET** /sapi/v1/sub-account/futures/internalTransfer | Sub-account Futures Asset Transfer History (For Master Account)
[**sapi_v1_sub_account_futures_internal_transfer_post**](SubAccountApi.md#sapi_v1_sub_account_futures_internal_transfer_post) | **POST** /sapi/v1/sub-account/futures/internalTransfer | Sub-account Futures Asset Transfer (For Master Account)
[**sapi_v1_sub_account_futures_position_risk_get**](SubAccountApi.md#sapi_v1_sub_account_futures_position_risk_get) | **GET** /sapi/v1/sub-account/futures/positionRisk | Futures Position-Risk of Sub-account (For Master Account)
[**sapi_v1_sub_account_futures_transfer_post**](SubAccountApi.md#sapi_v1_sub_account_futures_transfer_post) | **POST** /sapi/v1/sub-account/futures/transfer | Transfer for Sub-account (For Master Account)
[**sapi_v1_sub_account_list_get**](SubAccountApi.md#sapi_v1_sub_account_list_get) | **GET** /sapi/v1/sub-account/list | Query Sub-account List (For Master Account)
[**sapi_v1_sub_account_margin_account_get**](SubAccountApi.md#sapi_v1_sub_account_margin_account_get) | **GET** /sapi/v1/sub-account/margin/account | Detail on Sub-account's Margin Account (For Master Account)
[**sapi_v1_sub_account_margin_account_summary_get**](SubAccountApi.md#sapi_v1_sub_account_margin_account_summary_get) | **GET** /sapi/v1/sub-account/margin/accountSummary | Summary of Sub-account's Margin Account (For Master Account)
[**sapi_v1_sub_account_margin_enable_post**](SubAccountApi.md#sapi_v1_sub_account_margin_enable_post) | **POST** /sapi/v1/sub-account/margin/enable | Enable Margin for Sub-account (For Master Account)
[**sapi_v1_sub_account_margin_transfer_post**](SubAccountApi.md#sapi_v1_sub_account_margin_transfer_post) | **POST** /sapi/v1/sub-account/margin/transfer | Margin Transfer for Sub-account (For Master Account)
[**sapi_v1_sub_account_spot_summary_get**](SubAccountApi.md#sapi_v1_sub_account_spot_summary_get) | **GET** /sapi/v1/sub-account/spotSummary | Sub-account Spot Assets Summary (For Master Account)
[**sapi_v1_sub_account_status_get**](SubAccountApi.md#sapi_v1_sub_account_status_get) | **GET** /sapi/v1/sub-account/status | Sub-account's Status on Margin/Futures (For Master Account)
[**sapi_v1_sub_account_sub_account_api_ip_restriction_get**](SubAccountApi.md#sapi_v1_sub_account_sub_account_api_ip_restriction_get) | **GET** /sapi/v1/sub-account/subAccountApi/ipRestriction | Get IP Restriction for a Sub-account API Key (For Master Account)
[**sapi_v1_sub_account_sub_account_api_ip_restriction_ip_list_delete**](SubAccountApi.md#sapi_v1_sub_account_sub_account_api_ip_restriction_ip_list_delete) | **DELETE** /sapi/v1/sub-account/subAccountApi/ipRestriction/ipList | Delete IP List for a Sub-account API Key (For Master Account)
[**sapi_v1_sub_account_sub_transfer_history_get**](SubAccountApi.md#sapi_v1_sub_account_sub_transfer_history_get) | **GET** /sapi/v1/sub-account/sub/transfer/history | Sub-account Spot Asset Transfer History (For Master Account)
[**sapi_v1_sub_account_transaction_statistics_get**](SubAccountApi.md#sapi_v1_sub_account_transaction_statistics_get) | **GET** /sapi/v1/sub-account/transaction-statistics | Query Sub-account Transaction Statistics (For Master Account)
[**sapi_v1_sub_account_transfer_sub_to_master_post**](SubAccountApi.md#sapi_v1_sub_account_transfer_sub_to_master_post) | **POST** /sapi/v1/sub-account/transfer/subToMaster | Transfer to Master (For Sub-account)
[**sapi_v1_sub_account_transfer_sub_to_sub_post**](SubAccountApi.md#sapi_v1_sub_account_transfer_sub_to_sub_post) | **POST** /sapi/v1/sub-account/transfer/subToSub | Transfer to Sub-account of Same Master (For Sub-account)
[**sapi_v1_sub_account_transfer_sub_user_history_get**](SubAccountApi.md#sapi_v1_sub_account_transfer_sub_user_history_get) | **GET** /sapi/v1/sub-account/transfer/subUserHistory | Sub-account Transfer History (For Sub-account)
[**sapi_v1_sub_account_universal_transfer_get**](SubAccountApi.md#sapi_v1_sub_account_universal_transfer_get) | **GET** /sapi/v1/sub-account/universalTransfer | Universal Transfer History (For Master Account)
[**sapi_v1_sub_account_universal_transfer_post**](SubAccountApi.md#sapi_v1_sub_account_universal_transfer_post) | **POST** /sapi/v1/sub-account/universalTransfer | Universal Transfer (For Master Account)
[**sapi_v1_sub_account_virtual_sub_account_post**](SubAccountApi.md#sapi_v1_sub_account_virtual_sub_account_post) | **POST** /sapi/v1/sub-account/virtualSubAccount | Create a Virtual Sub-account(For Master Account)
[**sapi_v2_sub_account_futures_account_get**](SubAccountApi.md#sapi_v2_sub_account_futures_account_get) | **GET** /sapi/v2/sub-account/futures/account | Detail on Sub-account's Futures Account V2 (For Master Account)
[**sapi_v2_sub_account_futures_account_summary_get**](SubAccountApi.md#sapi_v2_sub_account_futures_account_summary_get) | **GET** /sapi/v2/sub-account/futures/accountSummary | Summary of Sub-account's Futures Account V2 (For Master Account)
[**sapi_v2_sub_account_futures_position_risk_get**](SubAccountApi.md#sapi_v2_sub_account_futures_position_risk_get) | **GET** /sapi/v2/sub-account/futures/positionRisk | Futures Position-Risk of Sub-account V2 (For Master Account)
[**sapi_v2_sub_account_sub_account_api_ip_restriction_post**](SubAccountApi.md#sapi_v2_sub_account_sub_account_api_ip_restriction_post) | **POST** /sapi/v2/sub-account/subAccountApi/ipRestriction | Update IP Restriction for Sub-Account API key (For Master Account)
[**sapi_v3_sub_account_assets_get**](SubAccountApi.md#sapi_v3_sub_account_assets_get) | **GET** /sapi/v3/sub-account/assets | Sub-account Assets (For Master Account)
[**sapi_v4_sub_account_assets_get**](SubAccountApi.md#sapi_v4_sub_account_assets_get) | **GET** /sapi/v4/sub-account/assets | Query Sub-account Assets (For Master Account)



## sapi_v1_capital_deposit_sub_address_get

> models::SapiV1CapitalDepositSubAddressGet200Response sapi_v1_capital_deposit_sub_address_get(email, coin, timestamp, signature, network, recv_window)
Sub-account Spot Assets Summary (For Master Account)

Fetch sub-account deposit address  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Sub-account email | [required] |
**coin** | **String** | Coin name | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**network** | Option<**String**> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1CapitalDepositSubAddressGet200Response**](_sapi_v1_capital_deposit_subAddress_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_capital_deposit_sub_hisrec_get

> Vec<models::SapiV1CapitalDepositSubHisrecGet200ResponseInner> sapi_v1_capital_deposit_sub_hisrec_get(email, timestamp, signature, coin, status, start_time, end_time, limit, offset, recv_window)
Sub-account Deposit History (For Master Account)

Fetch sub-account deposit history  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Sub-account email | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**coin** | Option<**String**> | Coin name |  |
**status** | Option<**i32**> | 0(0:pending,6: credited but cannot withdraw, 1:success) |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i64**> |  |  |
**offset** | Option<**i32**> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1CapitalDepositSubHisrecGet200ResponseInner>**](_sapi_v1_capital_deposit_subHisrec_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_managed_subaccount_account_snapshot_get

> models::SapiV1ManagedSubaccountAccountSnapshotGet200Response sapi_v1_managed_subaccount_account_snapshot_get(email, r#type, timestamp, signature, start_time, end_time, limit, recv_window)
Managed sub-account snapshot (For Investor Master Account)

- The query time period must be less then 30 days - Support query within the last one month only - If `startTime` and `endTime` not sent, return records of the last 7 days by default  Weight(IP): 2400

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Sub-account email | [required] |
**r#type** | **String** | \"SPOT\", \"MARGIN\"(cross), \"FUTURES\"(UM) | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i32**> | min 7, max 30, default 7 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1ManagedSubaccountAccountSnapshotGet200Response**](_sapi_v1_managed_subaccount_accountSnapshot_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_managed_subaccount_asset_get

> Vec<models::SapiV1ManagedSubaccountAssetGet200ResponseInner> sapi_v1_managed_subaccount_asset_get(email, timestamp, signature, recv_window)
Managed sub-account asset details(For Investor Master Account)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Sub-account email | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1ManagedSubaccountAssetGet200ResponseInner>**](_sapi_v1_managed_subaccount_asset_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_managed_subaccount_deposit_address_get

> models::SapiV1ManagedSubaccountDepositAddressGet200Response sapi_v1_managed_subaccount_deposit_address_get(email, coin, timestamp, signature, network, recv_window)
Get Managed Sub-account Deposit Address (For Investor Master Account)

Get investor's managed sub-account deposit address  Weight(UID): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |
**coin** | **String** | Coin name | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**network** | Option<**String**> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1ManagedSubaccountDepositAddressGet200Response**](_sapi_v1_managed_subaccount_deposit_address_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_managed_subaccount_deposit_post

> models::SapiV1ManagedSubaccountDepositPost200Response sapi_v1_managed_subaccount_deposit_post(to_email, asset, amount, timestamp, signature, recv_window)
Deposit assets into the managed sub-account(For Investor Master Account)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**to_email** | **String** | Recipient email | [required] |
**asset** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1ManagedSubaccountDepositPost200Response**](_sapi_v1_managed_subaccount_deposit_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_managed_subaccount_fetch_future_asset_get

> models::SapiV1ManagedSubaccountFetchFutureAssetGet200Response sapi_v1_managed_subaccount_fetch_future_asset_get(email, timestamp, signature, recv_window)
Query Managed Sub-account Futures Asset Details (For Investor Master Account)

Investor can use this api to query managed sub account futures asset details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1ManagedSubaccountFetchFutureAssetGet200Response**](_sapi_v1_managed_subaccount_fetch_future_asset_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_managed_subaccount_info_get

> models::SapiV1ManagedSubaccountInfoGet200Response sapi_v1_managed_subaccount_info_get(email, timestamp, signature, page, limit, recv_window)
Query Managed Sub-account List (For Investor)

Get investor's managed sub-account list.  Weight(UID): 60

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**page** | Option<**i32**> | Default 1 |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1ManagedSubaccountInfoGet200Response**](_sapi_v1_managed_subaccount_info_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_managed_subaccount_margin_asset_get

> models::SapiV1ManagedSubaccountMarginAssetGet200Response sapi_v1_managed_subaccount_margin_asset_get(email, timestamp, signature, recv_window)
Query Managed Sub-account Margin Asset Details (For Investor Master Account)

Investor can use this api to query managed sub account margin asset details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1ManagedSubaccountMarginAssetGet200Response**](_sapi_v1_managed_subaccount_marginAsset_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_managed_subaccount_query_trans_log_for_investor_get

> models::SapiV1ManagedSubaccountQueryTransLogForInvestorGet200Response sapi_v1_managed_subaccount_query_trans_log_for_investor_get(email, timestamp, signature, start_time, end_time, page, limit, transfers, transfer_function_account_type, recv_window)
Query Managed Sub Account Transfer Log (For Investor Master Account)

Investor can use this api to query managed sub account transfer log. This endpoint is available for investor of Managed Sub-Account. A Managed Sub-Account is an account type for investors who value flexibility in asset allocation and account application, while delegating trades to a professional trading team.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**page** | Option<**i32**> | Default 1 |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**transfers** | Option<**String**> | Transfer Direction (FROM/TO) |  |
**transfer_function_account_type** | Option<**String**> | Transfer function account type (SPOT/MARGIN/ISOLATED_MARGIN/USDT_FUTURE/COIN_FUTURE) |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1ManagedSubaccountQueryTransLogForInvestorGet200Response**](_sapi_v1_managed_subaccount_queryTransLogForInvestor_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_managed_subaccount_query_trans_log_for_trade_parent_get

> models::SapiV1ManagedSubaccountQueryTransLogForInvestorGet200Response sapi_v1_managed_subaccount_query_trans_log_for_trade_parent_get(email, timestamp, signature, start_time, end_time, page, limit, transfers, transfer_function_account_type, recv_window)
Query Managed Sub Account Transfer Log (For Trading Team Master Account)

Trading team can use this api to query managed sub account transfer log. This endpoint is available for trading team of Managed Sub-Account. A Managed Sub-Account is an account type for investors who value flexibility in asset allocation and account application, while delegating trades to a professional trading team  Weight(IP): 60

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**page** | Option<**i32**> | Default 1 |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**transfers** | Option<**String**> | Transfer Direction (FROM/TO) |  |
**transfer_function_account_type** | Option<**String**> | Transfer function account type (SPOT/MARGIN/ISOLATED_MARGIN/USDT_FUTURE/COIN_FUTURE) |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1ManagedSubaccountQueryTransLogForInvestorGet200Response**](_sapi_v1_managed_subaccount_queryTransLogForInvestor_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_managed_subaccount_query_trans_log_get

> models::SapiV1ManagedSubaccountQueryTransLogGet200Response sapi_v1_managed_subaccount_query_trans_log_get(transfers, transfer_function_account_type, timestamp, signature, start_time, end_time, page, limit, recv_window)
Query Managed Sub Account Transfer Log (For Trading Team Sub Account)(USER_DATA)

Query Managed Sub Account Transfer Log (For Trading Team Sub Account)  Weight(UID): 60

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfers** | **String** | Transfer Direction | [required] |
**transfer_function_account_type** | **String** | Transfer function account type | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**page** | Option<**i32**> | Default 1 |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1ManagedSubaccountQueryTransLogGet200Response**](_sapi_v1_managed_subaccount_query_trans_log_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_managed_subaccount_withdraw_post

> models::SapiV1ManagedSubaccountDepositPost200Response sapi_v1_managed_subaccount_withdraw_post(from_email, asset, amount, timestamp, signature, transfer_date, recv_window)
Withdrawl assets from the managed sub-account(For Investor Master Account)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_email** | **String** | Sender email | [required] |
**asset** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**transfer_date** | Option<**i64**> | Withdrawals is automatically occur on the transfer date(UTC0). If a date is not selected, the withdrawal occurs right now |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1ManagedSubaccountDepositPost200Response**](_sapi_v1_managed_subaccount_deposit_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_blvt_enable_post

> models::SapiV1SubAccountBlvtEnablePost200Response sapi_v1_sub_account_blvt_enable_post(email, enable_blvt, timestamp, signature, recv_window)
Enable Leverage Token for Sub-account (For Master Account)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Sub-account email | [required] |
**enable_blvt** | **bool** | Only true for now | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SubAccountBlvtEnablePost200Response**](_sapi_v1_sub_account_blvt_enable_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_eoptions_enable_post

> models::SapiV1SubAccountEoptionsEnablePost200Response sapi_v1_sub_account_eoptions_enable_post(email, timestamp, signature, recv_window)
Enable Options for Sub-account (For Master Account)(USER_DATA)

Enable Options for Sub-account (For Master Account).  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SubAccountEoptionsEnablePost200Response**](_sapi_v1_sub_account_eoptions_enable_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_futures_account_get

> models::SapiV1SubAccountFuturesAccountGet200Response sapi_v1_sub_account_futures_account_get(email, timestamp, signature, recv_window)
Detail on Sub-account's Futures Account (For Master Account)

Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SubAccountFuturesAccountGet200Response**](_sapi_v1_sub_account_futures_account_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_futures_account_summary_get

> models::SapiV1SubAccountFuturesAccountSummaryGet200Response sapi_v1_sub_account_futures_account_summary_get(timestamp, signature, recv_window)
Summary of Sub-account's Futures Account (For Master Account)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SubAccountFuturesAccountSummaryGet200Response**](_sapi_v1_sub_account_futures_accountSummary_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_futures_enable_post

> models::SapiV1SubAccountFuturesEnablePost200Response sapi_v1_sub_account_futures_enable_post(email, timestamp, signature, recv_window)
Enable Futures for Sub-account (For Master Account)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Sub-account email | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SubAccountFuturesEnablePost200Response**](_sapi_v1_sub_account_futures_enable_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_futures_internal_transfer_get

> models::SapiV1SubAccountFuturesInternalTransferGet200Response sapi_v1_sub_account_futures_internal_transfer_get(email, futures_type, timestamp, signature, start_time, end_time, page, limit, recv_window)
Sub-account Futures Asset Transfer History (For Master Account)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Sub-account email | [required] |
**futures_type** | **i32** | 1:USDT-margined Futures, 2: Coin-margined Futures | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**page** | Option<**i32**> | Default 1 |  |
**limit** | Option<**i32**> | Default value: 50, Max value: 500 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SubAccountFuturesInternalTransferGet200Response**](_sapi_v1_sub_account_futures_internalTransfer_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_futures_internal_transfer_post

> models::SapiV1SubAccountFuturesInternalTransferPost200Response sapi_v1_sub_account_futures_internal_transfer_post(from_email, to_email, futures_type, asset, amount, timestamp, signature, recv_window)
Sub-account Futures Asset Transfer (For Master Account)

- Master account can transfer max 2000 times a minute  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_email** | **String** | Sender email | [required] |
**to_email** | **String** | Recipient email | [required] |
**futures_type** | **i32** | 1:USDT-margined Futures,2: Coin-margined Futures | [required] |
**asset** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SubAccountFuturesInternalTransferPost200Response**](_sapi_v1_sub_account_futures_internalTransfer_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_futures_position_risk_get

> Vec<models::SapiV1SubAccountFuturesPositionRiskGet200ResponseInner> sapi_v1_sub_account_futures_position_risk_get(email, timestamp, signature, recv_window)
Futures Position-Risk of Sub-account (For Master Account)

Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Sub-account email | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1SubAccountFuturesPositionRiskGet200ResponseInner>**](_sapi_v1_sub_account_futures_positionRisk_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_futures_transfer_post

> models::SapiV1SubAccountFuturesTransferPost200Response sapi_v1_sub_account_futures_transfer_post(email, asset, amount, r#type, timestamp, signature, recv_window)
Transfer for Sub-account (For Master Account)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Sub-account email | [required] |
**asset** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**r#type** | **i32** | * `1` - transfer from subaccount's spot account to its USDT-margined futures account * `2` - transfer from subaccount's USDT-margined futures account to its spot account * `3` - transfer from subaccount's spot account to its COIN-margined futures account * `4` - transfer from subaccount's COIN-margined futures account to its spot account | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SubAccountFuturesTransferPost200Response**](_sapi_v1_sub_account_futures_transfer_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_list_get

> models::SapiV1SubAccountListGet200Response sapi_v1_sub_account_list_get(timestamp, signature, email, is_freeze, page, limit, recv_window)
Query Sub-account List (For Master Account)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**email** | Option<**String**> | Sub-account email |  |
**is_freeze** | Option<**String**> |  |  |
**page** | Option<**i32**> | Default 1 |  |
**limit** | Option<**i32**> | Default 1; max 200 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SubAccountListGet200Response**](_sapi_v1_sub_account_list_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_margin_account_get

> models::SapiV1SubAccountMarginAccountGet200Response sapi_v1_sub_account_margin_account_get(email, timestamp, signature, recv_window)
Detail on Sub-account's Margin Account (For Master Account)

Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Sub-account email | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SubAccountMarginAccountGet200Response**](_sapi_v1_sub_account_margin_account_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_margin_account_summary_get

> models::SapiV1SubAccountMarginAccountSummaryGet200Response sapi_v1_sub_account_margin_account_summary_get(timestamp, signature, recv_window)
Summary of Sub-account's Margin Account (For Master Account)

Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SubAccountMarginAccountSummaryGet200Response**](_sapi_v1_sub_account_margin_accountSummary_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_margin_enable_post

> models::SapiV1SubAccountMarginEnablePost200Response sapi_v1_sub_account_margin_enable_post(email, timestamp, signature, recv_window)
Enable Margin for Sub-account (For Master Account)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Sub-account email | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SubAccountMarginEnablePost200Response**](_sapi_v1_sub_account_margin_enable_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_margin_transfer_post

> models::SapiV1SubAccountFuturesTransferPost200Response sapi_v1_sub_account_margin_transfer_post(email, asset, amount, r#type, timestamp, signature, recv_window)
Margin Transfer for Sub-account (For Master Account)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Sub-account email | [required] |
**asset** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**r#type** | **i32** | * `1` - transfer from subaccount's spot account to margin account * `2` - transfer from subaccount's margin account to its spot account | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SubAccountFuturesTransferPost200Response**](_sapi_v1_sub_account_futures_transfer_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_spot_summary_get

> models::SapiV1SubAccountSpotSummaryGet200Response sapi_v1_sub_account_spot_summary_get(timestamp, signature, email, page, size, recv_window)
Sub-account Spot Assets Summary (For Master Account)

Get BTC valued asset summary of subaccounts.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**email** | Option<**String**> | Sub-account email |  |
**page** | Option<**i32**> | Default 1 |  |
**size** | Option<**i32**> | Default:10 Max:20 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SubAccountSpotSummaryGet200Response**](_sapi_v1_sub_account_spotSummary_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_status_get

> Vec<models::SapiV1SubAccountStatusGet200ResponseInner> sapi_v1_sub_account_status_get(timestamp, signature, email, recv_window)
Sub-account's Status on Margin/Futures (For Master Account)

- If no `email` sent, all sub-accounts' information will be returned.  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**email** | Option<**String**> | Sub-account email |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1SubAccountStatusGet200ResponseInner>**](_sapi_v1_sub_account_status_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_sub_account_api_ip_restriction_get

> models::SapiV1SubAccountSubAccountApiIpRestrictionGet200Response sapi_v1_sub_account_sub_account_api_ip_restriction_get(email, sub_account_api_key, timestamp, signature, recv_window)
Get IP Restriction for a Sub-account API Key (For Master Account)

Weight(UID): 3000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Sub-account email | [required] |
**sub_account_api_key** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SubAccountSubAccountApiIpRestrictionGet200Response**](_sapi_v1_sub_account_subAccountApi_ipRestriction_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_sub_account_api_ip_restriction_ip_list_delete

> models::SapiV1SubAccountSubAccountApiIpRestrictionIpListDelete200Response sapi_v1_sub_account_sub_account_api_ip_restriction_ip_list_delete(email, sub_account_api_key, timestamp, signature, ip_address, third_party_name, recv_window)
Delete IP List for a Sub-account API Key (For Master Account)

Weight(UID): 3000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Sub-account email | [required] |
**sub_account_api_key** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**ip_address** | Option<**String**> | Can be added in batches, separated by commas |  |
**third_party_name** | Option<**String**> | third party IP list name |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SubAccountSubAccountApiIpRestrictionIpListDelete200Response**](_sapi_v1_sub_account_subAccountApi_ipRestriction_ipList_delete_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_sub_transfer_history_get

> Vec<models::SapiV1SubAccountSubTransferHistoryGet200ResponseInner> sapi_v1_sub_account_sub_transfer_history_get(timestamp, signature, from_email, to_email, start_time, end_time, page, limit, recv_window)
Sub-account Spot Asset Transfer History (For Master Account)

- fromEmail and toEmail cannot be sent at the same time. - Return fromEmail equal master account email by default.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**from_email** | Option<**String**> | Sub-account email |  |
**to_email** | Option<**String**> | Sub-account email |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**page** | Option<**i32**> | Default 1 |  |
**limit** | Option<**i32**> | Default 1 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1SubAccountSubTransferHistoryGet200ResponseInner>**](_sapi_v1_sub_account_sub_transfer_history_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_transaction_statistics_get

> models::SapiV1SubAccountTransactionStatisticsGet200Response sapi_v1_sub_account_transaction_statistics_get(email, timestamp, signature, recv_window)
Query Sub-account Transaction Statistics (For Master Account)

Query Sub-account Transaction statistics (For Master Account).  Weight(UID): 60

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SubAccountTransactionStatisticsGet200Response**](_sapi_v1_sub_account_transaction_statistics_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_transfer_sub_to_master_post

> models::SapiV1SubAccountFuturesTransferPost200Response sapi_v1_sub_account_transfer_sub_to_master_post(asset, amount, timestamp, signature, recv_window)
Transfer to Master (For Sub-account)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SubAccountFuturesTransferPost200Response**](_sapi_v1_sub_account_futures_transfer_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_transfer_sub_to_sub_post

> models::SapiV1SubAccountFuturesTransferPost200Response sapi_v1_sub_account_transfer_sub_to_sub_post(to_email, asset, amount, timestamp, signature, recv_window)
Transfer to Sub-account of Same Master (For Sub-account)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**to_email** | **String** | Recipient email | [required] |
**asset** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SubAccountFuturesTransferPost200Response**](_sapi_v1_sub_account_futures_transfer_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_transfer_sub_user_history_get

> Vec<models::SapiV1SubAccountTransferSubUserHistoryGet200ResponseInner> sapi_v1_sub_account_transfer_sub_user_history_get(timestamp, signature, asset, r#type, start_time, end_time, limit, recv_window)
Sub-account Transfer History (For Sub-account)

- If `type` is not sent, the records of type 2: transfer out will be returned by default. - If `startTime` and `endTime` are not sent, the recent 30-day data will be returned.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> |  |  |
**r#type** | Option<**i32**> | * `1` - transfer in * `2` - transfer out |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1SubAccountTransferSubUserHistoryGet200ResponseInner>**](_sapi_v1_sub_account_transfer_subUserHistory_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_universal_transfer_get

> Vec<models::SapiV1SubAccountUniversalTransferGet200ResponseInner> sapi_v1_sub_account_universal_transfer_get(timestamp, signature, from_email, to_email, client_tran_id, start_time, end_time, page, limit, recv_window)
Universal Transfer History (For Master Account)

- `fromEmail` and `toEmail` cannot be sent at the same time. - Return `fromEmail` equal master account email by default. - The query time period must be less then 30 days. - If startTime and endTime not sent, return records of the last 30 days by default.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**from_email** | Option<**String**> | Sub-account email |  |
**to_email** | Option<**String**> | Sub-account email |  |
**client_tran_id** | Option<**String**> |  |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**page** | Option<**i32**> | Default 1 |  |
**limit** | Option<**i32**> | Default 500, Max 500 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1SubAccountUniversalTransferGet200ResponseInner>**](_sapi_v1_sub_account_universalTransfer_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_universal_transfer_post

> models::SapiV1SubAccountUniversalTransferPost200Response sapi_v1_sub_account_universal_transfer_post(from_account_type, to_account_type, asset, amount, timestamp, signature, from_email, to_email, client_tran_id, symbol, recv_window)
Universal Transfer (For Master Account)

- You need to enable \"internal transfer\" option for the api key which requests this endpoint. - Transfer from master account by default if fromEmail is not sent. - Transfer to master account by default if toEmail is not sent. - Supported transfer scenarios:   - Master account SPOT transfer to sub-account SPOT,USDT_FUTURE,COIN_FUTURE,MARGIN(Cross),ISOLATED_MARGIN   - Sub-account SPOT,USDT_FUTURE,COIN_FUTURE,MARGIN(Cross),ISOLATED_MARGIN transfer to master account SPOT   - Transfer between two sub-account SPOT accounts  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_account_type** | **String** |  | [required] |
**to_account_type** | **String** |  | [required] |
**asset** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**from_email** | Option<**String**> | Sub-account email |  |
**to_email** | Option<**String**> | Sub-account email |  |
**client_tran_id** | Option<**String**> |  |  |
**symbol** | Option<**String**> | Only supported under ISOLATED_MARGIN type |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SubAccountUniversalTransferPost200Response**](_sapi_v1_sub_account_universalTransfer_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_sub_account_virtual_sub_account_post

> models::SapiV1SubAccountVirtualSubAccountPost200Response sapi_v1_sub_account_virtual_sub_account_post(sub_account_string, timestamp, signature, recv_window)
Create a Virtual Sub-account(For Master Account)

- This request will generate a virtual sub account under your master account. - You need to enable \"trade\" option for the api key which requests this endpoint.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sub_account_string** | **String** | Please input a string. We will create a virtual email using that string for you to register | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SubAccountVirtualSubAccountPost200Response**](_sapi_v1_sub_account_virtualSubAccount_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v2_sub_account_futures_account_get

> models::SapiV2SubAccountFuturesAccountGet200Response sapi_v2_sub_account_futures_account_get(email, futures_type, timestamp, signature, recv_window)
Detail on Sub-account's Futures Account V2 (For Master Account)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Sub-account email | [required] |
**futures_type** | **i32** | * `1` - USDT Margined Futures * `2` - COIN Margined Futures | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV2SubAccountFuturesAccountGet200Response**](_sapi_v2_sub_account_futures_account_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v2_sub_account_futures_account_summary_get

> models::SapiV2SubAccountFuturesAccountSummaryGet200Response sapi_v2_sub_account_futures_account_summary_get(futures_type, timestamp, signature, page, limit, recv_window)
Summary of Sub-account's Futures Account V2 (For Master Account)

Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**futures_type** | **i32** | * `1` - USDT Margined Futures * `2` - COIN Margined Futures | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**page** | Option<**i32**> | Default 1 |  |
**limit** | Option<**i32**> | Default 10, Max 20 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV2SubAccountFuturesAccountSummaryGet200Response**](_sapi_v2_sub_account_futures_accountSummary_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v2_sub_account_futures_position_risk_get

> models::SapiV2SubAccountFuturesPositionRiskGet200Response sapi_v2_sub_account_futures_position_risk_get(email, futures_type, timestamp, signature, recv_window)
Futures Position-Risk of Sub-account V2 (For Master Account)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Sub-account email | [required] |
**futures_type** | **i32** | * `1` - USDT Margined Futures * `2` - COIN Margined Futures | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV2SubAccountFuturesPositionRiskGet200Response**](_sapi_v2_sub_account_futures_positionRisk_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v2_sub_account_sub_account_api_ip_restriction_post

> models::SapiV2SubAccountSubAccountApiIpRestrictionPost200Response sapi_v2_sub_account_sub_account_api_ip_restriction_post(email, sub_account_api_key, status, timestamp, signature, third_party_name, recv_window)
Update IP Restriction for Sub-Account API key (For Master Account)

Update IP Restriction for Sub-Account API key  Weight(UID): 3000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Sub-account email | [required] |
**sub_account_api_key** | **String** |  | [required] |
**status** | **String** | IP Restriction status. 1 = IP Unrestricted. 2 = Restrict access to trusted IPs only. 3 = Restrict access to users' trusted third party IPs only | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**third_party_name** | Option<**String**> | third party IP list name |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV2SubAccountSubAccountApiIpRestrictionPost200Response**](_sapi_v2_sub_account_subAccountApi_ipRestriction_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v3_sub_account_assets_get

> models::SapiV3SubAccountAssetsGet200Response sapi_v3_sub_account_assets_get(email, timestamp, signature, recv_window)
Sub-account Assets (For Master Account)

Fetch sub-account assets  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Sub-account email | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV3SubAccountAssetsGet200Response**](_sapi_v3_sub_account_assets_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v4_sub_account_assets_get

> models::SapiV4SubAccountAssetsGet200Response sapi_v4_sub_account_assets_get(email, timestamp, signature, recv_window)
Query Sub-account Assets (For Master Account)

Fetch sub-account assets  Weight(UID): 60

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV4SubAccountAssetsGet200Response**](_sapi_v4_sub_account_assets_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

