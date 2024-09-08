# \AutoInvestApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_lending_auto_invest_all_asset_get**](AutoInvestApi.md#sapi_v1_lending_auto_invest_all_asset_get) | **GET** /sapi/v1/lending/auto-invest/all/asset | Query all source asset and target asset (USER_DATA)
[**sapi_v1_lending_auto_invest_history_list_get**](AutoInvestApi.md#sapi_v1_lending_auto_invest_history_list_get) | **GET** /sapi/v1/lending/auto-invest/history/list | Query subscription transaction history
[**sapi_v1_lending_auto_invest_index_info_get**](AutoInvestApi.md#sapi_v1_lending_auto_invest_index_info_get) | **GET** /sapi/v1/lending/auto-invest/index/info | Query Index Details(USER_DATA)
[**sapi_v1_lending_auto_invest_index_user_summary_get**](AutoInvestApi.md#sapi_v1_lending_auto_invest_index_user_summary_get) | **GET** /sapi/v1/lending/auto-invest/index/user-summary | Query Index Linked Plan Position Details(USER_DATA)
[**sapi_v1_lending_auto_invest_one_off_post**](AutoInvestApi.md#sapi_v1_lending_auto_invest_one_off_post) | **POST** /sapi/v1/lending/auto-invest/one-off | One Time Transaction(TRADE)
[**sapi_v1_lending_auto_invest_one_off_status_get**](AutoInvestApi.md#sapi_v1_lending_auto_invest_one_off_status_get) | **GET** /sapi/v1/lending/auto-invest/one-off/status | Query One-Time Transaction Status (USER_DATA)
[**sapi_v1_lending_auto_invest_plan_add_post**](AutoInvestApi.md#sapi_v1_lending_auto_invest_plan_add_post) | **POST** /sapi/v1/lending/auto-invest/plan/add | Investment plan creation (USER_DATA)
[**sapi_v1_lending_auto_invest_plan_edit_post**](AutoInvestApi.md#sapi_v1_lending_auto_invest_plan_edit_post) | **POST** /sapi/v1/lending/auto-invest/plan/edit | Investment plan adjustment
[**sapi_v1_lending_auto_invest_plan_edit_status_post**](AutoInvestApi.md#sapi_v1_lending_auto_invest_plan_edit_status_post) | **POST** /sapi/v1/lending/auto-invest/plan/edit-status | Change Plan Status
[**sapi_v1_lending_auto_invest_plan_id_get**](AutoInvestApi.md#sapi_v1_lending_auto_invest_plan_id_get) | **GET** /sapi/v1/lending/auto-invest/plan/id | Query holding details of the plan
[**sapi_v1_lending_auto_invest_plan_list_get**](AutoInvestApi.md#sapi_v1_lending_auto_invest_plan_list_get) | **GET** /sapi/v1/lending/auto-invest/plan/list | Get list of plans
[**sapi_v1_lending_auto_invest_rebalance_history_get**](AutoInvestApi.md#sapi_v1_lending_auto_invest_rebalance_history_get) | **GET** /sapi/v1/lending/auto-invest/rebalance/history | Index Linked Plan Rebalance Details (USER_DATA)
[**sapi_v1_lending_auto_invest_redeem_history_get**](AutoInvestApi.md#sapi_v1_lending_auto_invest_redeem_history_get) | **GET** /sapi/v1/lending/auto-invest/redeem/history | Index Linked Plan Redemption History (USER_DATA)
[**sapi_v1_lending_auto_invest_redeem_post**](AutoInvestApi.md#sapi_v1_lending_auto_invest_redeem_post) | **POST** /sapi/v1/lending/auto-invest/redeem | Index Linked Plan Redemption (TRADE)
[**sapi_v1_lending_auto_invest_source_asset_list_get**](AutoInvestApi.md#sapi_v1_lending_auto_invest_source_asset_list_get) | **GET** /sapi/v1/lending/auto-invest/source-asset/list | Query source asset list (USER_DATA)
[**sapi_v1_lending_auto_invest_target_asset_list_get**](AutoInvestApi.md#sapi_v1_lending_auto_invest_target_asset_list_get) | **GET** /sapi/v1/lending/auto-invest/target-asset/list | Get target asset list (USER_DATA)
[**sapi_v1_lending_auto_invest_target_asset_roi_list_get**](AutoInvestApi.md#sapi_v1_lending_auto_invest_target_asset_roi_list_get) | **GET** /sapi/v1/lending/auto-invest/target-asset/roi/list | Get target asset ROI data (USER_DATA)



## sapi_v1_lending_auto_invest_all_asset_get

> models::SapiV1LendingAutoInvestAllAssetGet200Response sapi_v1_lending_auto_invest_all_asset_get(timestamp, signature, recv_window)
Query all source asset and target asset (USER_DATA)

Query all source assets and target assets  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LendingAutoInvestAllAssetGet200Response**](_sapi_v1_lending_auto_invest_all_asset_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_auto_invest_history_list_get

> Vec<models::SapiV1LendingAutoInvestHistoryListGet200ResponseInner> sapi_v1_lending_auto_invest_history_list_get(timestamp, signature, plan_id, start_time, end_time, target_asset, plan_type, size, current, recv_window)
Query subscription transaction history

Query subscription transaction history of a plan  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**plan_id** | Option<**i64**> |  |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**target_asset** | Option<**f64**> |  |  |
**plan_type** | Option<**String**> |  |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1LendingAutoInvestHistoryListGet200ResponseInner>**](_sapi_v1_lending_auto_invest_history_list_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_auto_invest_index_info_get

> models::SapiV1LendingAutoInvestIndexInfoGet200Response sapi_v1_lending_auto_invest_index_info_get(index_id, timestamp, signature, recv_window)
Query Index Details(USER_DATA)

Query index details  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index_id** | **i64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LendingAutoInvestIndexInfoGet200Response**](_sapi_v1_lending_auto_invest_index_info_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_auto_invest_index_user_summary_get

> models::SapiV1LendingAutoInvestIndexUserSummaryGet200Response sapi_v1_lending_auto_invest_index_user_summary_get(index_id, timestamp, signature, recv_window)
Query Index Linked Plan Position Details(USER_DATA)

Details on users Index-Linked plan position details  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index_id** | **i64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LendingAutoInvestIndexUserSummaryGet200Response**](_sapi_v1_lending_auto_invest_index_user_summary_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_auto_invest_one_off_post

> models::SapiV1LendingAutoInvestOneOffPost200Response sapi_v1_lending_auto_invest_one_off_post(source_type, subscription_amount, source_asset, timestamp, signature, request_id, flexible_allowed_to_use, plan_id, index_id, details, recv_window)
One Time Transaction(TRADE)

One time transaction  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_type** | **String** |  | [required] |
**subscription_amount** | **f32** |  | [required] |
**source_asset** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**request_id** | Option<**String**> |  |  |
**flexible_allowed_to_use** | Option<**bool**> |  |  |
**plan_id** | Option<**i64**> |  |  |
**index_id** | Option<**i64**> |  |  |
**details** | Option<[**Vec<models::SapiV1LendingAutoInvestOneOffPostDetailsParameterInner>**](models::SapiV1LendingAutoInvestOneOffPostDetailsParameterInner.md)> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LendingAutoInvestOneOffPost200Response**](_sapi_v1_lending_auto_invest_one_off_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_auto_invest_one_off_status_get

> models::SapiV1LendingAutoInvestOneOffStatusGet200Response sapi_v1_lending_auto_invest_one_off_status_get(transaction_id, timestamp, signature, request_id, recv_window)
Query One-Time Transaction Status (USER_DATA)

Transaction status for one-time transaction  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **i64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**request_id** | Option<**String**> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LendingAutoInvestOneOffStatusGet200Response**](_sapi_v1_lending_auto_invest_one_off_status_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_auto_invest_plan_add_post

> models::SapiV1LendingAutoInvestPlanAddPost200Response sapi_v1_lending_auto_invest_plan_add_post(source_type, plan_type, subscription_amount, subscription_cycle, subscription_start_time, source_asset, details, timestamp, signature, request_id, index_id, subscription_start_day, subscription_start_weekday, flexible_allowed_to_use, recv_window)
Investment plan creation (USER_DATA)

Post an investment plan creation  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_type** | **String** |  | [required] |
**plan_type** | **String** |  | [required] |
**subscription_amount** | **f32** |  | [required] |
**subscription_cycle** | **String** |  | [required] |
**subscription_start_time** | **i32** |  | [required] |
**source_asset** | **String** |  | [required] |
**details** | [**Vec<models::SapiV1LendingAutoInvestPlanAddPostDetailsParameterInner>**](models::SapiV1LendingAutoInvestPlanAddPostDetailsParameterInner.md) |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**request_id** | Option<**String**> |  |  |
**index_id** | Option<**i64**> |  |  |
**subscription_start_day** | Option<**i32**> |  |  |
**subscription_start_weekday** | Option<**String**> |  |  |
**flexible_allowed_to_use** | Option<**bool**> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LendingAutoInvestPlanAddPost200Response**](_sapi_v1_lending_auto_invest_plan_add_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_auto_invest_plan_edit_post

> models::SapiV1LendingAutoInvestPlanAddPost200Response sapi_v1_lending_auto_invest_plan_edit_post(plan_id, subscription_amount, subscription_cycle, subscription_start_time, source_asset, timestamp, signature, subscription_start_day, subscription_start_weekday, flexible_allowed_to_use, details, recv_window)
Investment plan adjustment

Query Source Asset to be used for investment  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **i32** |  | [required] |
**subscription_amount** | **f32** |  | [required] |
**subscription_cycle** | **String** |  | [required] |
**subscription_start_time** | **i32** |  | [required] |
**source_asset** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**subscription_start_day** | Option<**i32**> |  |  |
**subscription_start_weekday** | Option<**String**> |  |  |
**flexible_allowed_to_use** | Option<**bool**> |  |  |
**details** | Option<[**Vec<models::SapiV1LendingAutoInvestPlanAddPostDetailsParameterInner>**](models::SapiV1LendingAutoInvestPlanAddPostDetailsParameterInner.md)> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LendingAutoInvestPlanAddPost200Response**](_sapi_v1_lending_auto_invest_plan_add_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_auto_invest_plan_edit_status_post

> models::SapiV1LendingAutoInvestPlanEditStatusPost200Response sapi_v1_lending_auto_invest_plan_edit_status_post(plan_id, status, timestamp, signature, recv_window)
Change Plan Status

Change Plan Status  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **i32** |  | [required] |
**status** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LendingAutoInvestPlanEditStatusPost200Response**](_sapi_v1_lending_auto_invest_plan_edit_status_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_auto_invest_plan_id_get

> models::SapiV1LendingAutoInvestPlanIdGet200Response sapi_v1_lending_auto_invest_plan_id_get(timestamp, signature, plan_id, request_id, recv_window)
Query holding details of the plan

Query holding details of the plan  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**plan_id** | Option<**i64**> |  |  |
**request_id** | Option<**String**> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LendingAutoInvestPlanIdGet200Response**](_sapi_v1_lending_auto_invest_plan_id_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_auto_invest_plan_list_get

> models::SapiV1LendingAutoInvestPlanListGet200Response sapi_v1_lending_auto_invest_plan_list_get(plan_type, timestamp, signature, recv_window)
Get list of plans

Query plan lists  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_type** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LendingAutoInvestPlanListGet200Response**](_sapi_v1_lending_auto_invest_plan_list_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_auto_invest_rebalance_history_get

> Vec<models::SapiV1LendingAutoInvestRebalanceHistoryGet200ResponseInner> sapi_v1_lending_auto_invest_rebalance_history_get(timestamp, signature, start_time, end_time, current, size, recv_window)
Index Linked Plan Rebalance Details (USER_DATA)

Get the history of Index Linked Plan Redemption transactions  Max 30 day difference between startTime and endTime If no startTime and endTime, default to show past 30 day records  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1LendingAutoInvestRebalanceHistoryGet200ResponseInner>**](_sapi_v1_lending_auto_invest_rebalance_history_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_auto_invest_redeem_history_get

> Vec<models::SapiV1LendingAutoInvestRedeemHistoryGet200ResponseInner> sapi_v1_lending_auto_invest_redeem_history_get(request_id, timestamp, signature, start_time, end_time, current, asset, size, recv_window)
Index Linked Plan Redemption History (USER_DATA)

Get the history of Index Linked Plan Redemption transactions  Max 30 day difference between startTime and endTime If no startTime and endTime, default to show past 30 day records  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **i64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**asset** | Option<**String**> |  |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1LendingAutoInvestRedeemHistoryGet200ResponseInner>**](_sapi_v1_lending_auto_invest_redeem_history_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_auto_invest_redeem_post

> models::SapiV1LendingAutoInvestRedeemPost200Response sapi_v1_lending_auto_invest_redeem_post(index_id, redemption_percentage, timestamp, signature, request_id, recv_window)
Index Linked Plan Redemption (TRADE)

To redeem index-Linked plan holdings  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index_id** | **i64** | PORTFOLIO plan's Id | [required] |
**redemption_percentage** | **i32** | user redeem percentage,10/20/100. | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**request_id** | Option<**String**> | sourceType + unique, transactionId and requestId cannot be empty at the same time |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LendingAutoInvestRedeemPost200Response**](_sapi_v1_lending_auto_invest_redeem_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_auto_invest_source_asset_list_get

> models::SapiV1LendingAutoInvestSourceAssetListGet200Response sapi_v1_lending_auto_invest_source_asset_list_get(usage_type, timestamp, signature, target_asset, index_id, flexible_allowed_to_use, recv_window)
Query source asset list (USER_DATA)

Query Source Asset to be used for investment  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**usage_type** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**target_asset** | Option<**String**> |  |  |
**index_id** | Option<**i64**> |  |  |
**flexible_allowed_to_use** | Option<**bool**> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LendingAutoInvestSourceAssetListGet200Response**](_sapi_v1_lending_auto_invest_source_asset_list_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_auto_invest_target_asset_list_get

> models::SapiV1LendingAutoInvestTargetAssetListGet200Response sapi_v1_lending_auto_invest_target_asset_list_get(timestamp, signature, target_asset, size, current, recv_window)
Get target asset list (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**target_asset** | Option<**String**> |  |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LendingAutoInvestTargetAssetListGet200Response**](_sapi_v1_lending_auto_invest_target_asset_list_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_auto_invest_target_asset_roi_list_get

> Vec<models::SapiV1LendingAutoInvestTargetAssetRoiListGet200ResponseInner> sapi_v1_lending_auto_invest_target_asset_roi_list_get(target_asset, his_roi_type, timestamp, signature, recv_window)
Get target asset ROI data (USER_DATA)

ROI return list for target asset  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target_asset** | **String** |  | [required] |
**his_roi_type** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1LendingAutoInvestTargetAssetRoiListGet200ResponseInner>**](_sapi_v1_lending_auto_invest_target_asset_roi_list_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

