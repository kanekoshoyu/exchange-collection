# \SimpleEarnApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_simple_earn_account_get**](SimpleEarnApi.md#sapi_v1_simple_earn_account_get) | **GET** /sapi/v1/simple-earn/account | Simple Account (USER_DATA)
[**sapi_v1_simple_earn_flexible_history_collateral_record_get**](SimpleEarnApi.md#sapi_v1_simple_earn_flexible_history_collateral_record_get) | **GET** /sapi/v1/simple-earn/flexible/history/collateralRecord | Get Collateral Record (USER_DATA)
[**sapi_v1_simple_earn_flexible_history_rate_history_get**](SimpleEarnApi.md#sapi_v1_simple_earn_flexible_history_rate_history_get) | **GET** /sapi/v1/simple-earn/flexible/history/rateHistory | Get Rate History (USER_DATA)
[**sapi_v1_simple_earn_flexible_history_redemption_record_get**](SimpleEarnApi.md#sapi_v1_simple_earn_flexible_history_redemption_record_get) | **GET** /sapi/v1/simple-earn/flexible/history/redemptionRecord | Get Flexible Redemption Record (USER_DATA)
[**sapi_v1_simple_earn_flexible_history_rewards_record_get**](SimpleEarnApi.md#sapi_v1_simple_earn_flexible_history_rewards_record_get) | **GET** /sapi/v1/simple-earn/flexible/history/rewardsRecord | Get Flexible Rewards History (USER_DATA)
[**sapi_v1_simple_earn_flexible_history_subscription_record_get**](SimpleEarnApi.md#sapi_v1_simple_earn_flexible_history_subscription_record_get) | **GET** /sapi/v1/simple-earn/flexible/history/subscriptionRecord | Get Flexible Subscription Record (USER_DATA)
[**sapi_v1_simple_earn_flexible_list_get**](SimpleEarnApi.md#sapi_v1_simple_earn_flexible_list_get) | **GET** /sapi/v1/simple-earn/flexible/list | Get Simple Earn Flexible Product List (USER_DATA)
[**sapi_v1_simple_earn_flexible_personal_left_quota_get**](SimpleEarnApi.md#sapi_v1_simple_earn_flexible_personal_left_quota_get) | **GET** /sapi/v1/simple-earn/flexible/personalLeftQuota | Get Flexible Personal Left Quota (USER_DATA)
[**sapi_v1_simple_earn_flexible_position_get**](SimpleEarnApi.md#sapi_v1_simple_earn_flexible_position_get) | **GET** /sapi/v1/simple-earn/flexible/position | Get Flexible Product Position (USER_DATA)
[**sapi_v1_simple_earn_flexible_redeem_post**](SimpleEarnApi.md#sapi_v1_simple_earn_flexible_redeem_post) | **POST** /sapi/v1/simple-earn/flexible/redeem | Redeem Flexible Product (TRADE)
[**sapi_v1_simple_earn_flexible_set_auto_subscribe_post**](SimpleEarnApi.md#sapi_v1_simple_earn_flexible_set_auto_subscribe_post) | **POST** /sapi/v1/simple-earn/flexible/setAutoSubscribe | Set Flexible Auto Subscribe (USER_DATA)
[**sapi_v1_simple_earn_flexible_subscribe_post**](SimpleEarnApi.md#sapi_v1_simple_earn_flexible_subscribe_post) | **POST** /sapi/v1/simple-earn/flexible/subscribe | Subscribe Flexible Product (TRADE)
[**sapi_v1_simple_earn_flexible_subscription_preview_get**](SimpleEarnApi.md#sapi_v1_simple_earn_flexible_subscription_preview_get) | **GET** /sapi/v1/simple-earn/flexible/subscriptionPreview | Get Flexible Subscription Preview (USER_DATA)
[**sapi_v1_simple_earn_locked_history_redemption_record_get**](SimpleEarnApi.md#sapi_v1_simple_earn_locked_history_redemption_record_get) | **GET** /sapi/v1/simple-earn/locked/history/redemptionRecord | Get Locked Redemption Record (USER_DATA)
[**sapi_v1_simple_earn_locked_history_rewards_record_get**](SimpleEarnApi.md#sapi_v1_simple_earn_locked_history_rewards_record_get) | **GET** /sapi/v1/simple-earn/locked/history/rewardsRecord | Get Locked Rewards History (USER_DATA)
[**sapi_v1_simple_earn_locked_history_subscription_record_get**](SimpleEarnApi.md#sapi_v1_simple_earn_locked_history_subscription_record_get) | **GET** /sapi/v1/simple-earn/locked/history/subscriptionRecord | Get Locked Subscription Record (USER_DATA)
[**sapi_v1_simple_earn_locked_list_get**](SimpleEarnApi.md#sapi_v1_simple_earn_locked_list_get) | **GET** /sapi/v1/simple-earn/locked/list | Get Simple Earn Locked Product List (USER_DATA)
[**sapi_v1_simple_earn_locked_personal_left_quota_get**](SimpleEarnApi.md#sapi_v1_simple_earn_locked_personal_left_quota_get) | **GET** /sapi/v1/simple-earn/locked/personalLeftQuota | Get Locked Personal Left Quota (USER_DATA)
[**sapi_v1_simple_earn_locked_position_get**](SimpleEarnApi.md#sapi_v1_simple_earn_locked_position_get) | **GET** /sapi/v1/simple-earn/locked/position | Get Locked Product Position (USER_DATA)
[**sapi_v1_simple_earn_locked_redeem_post**](SimpleEarnApi.md#sapi_v1_simple_earn_locked_redeem_post) | **POST** /sapi/v1/simple-earn/locked/redeem | Redeem Locked Product (TRADE)
[**sapi_v1_simple_earn_locked_set_auto_subscribe_post**](SimpleEarnApi.md#sapi_v1_simple_earn_locked_set_auto_subscribe_post) | **POST** /sapi/v1/simple-earn/locked/setAutoSubscribe | Set Locked Auto Subscribe (USER_DATA)
[**sapi_v1_simple_earn_locked_subscribe_post**](SimpleEarnApi.md#sapi_v1_simple_earn_locked_subscribe_post) | **POST** /sapi/v1/simple-earn/locked/subscribe | Subscribe Locked Product (TRADE)
[**sapi_v1_simple_earn_locked_subscription_preview_get**](SimpleEarnApi.md#sapi_v1_simple_earn_locked_subscription_preview_get) | **GET** /sapi/v1/simple-earn/locked/subscriptionPreview | Get Locked Subscription Preview (USER_DATA)



## sapi_v1_simple_earn_account_get

> models::SapiV1SimpleEarnAccountGet200Response sapi_v1_simple_earn_account_get(timestamp, signature, recv_window)
Simple Account (USER_DATA)

Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SimpleEarnAccountGet200Response**](_sapi_v1_simple_earn_account_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_simple_earn_flexible_history_collateral_record_get

> models::SapiV1SimpleEarnFlexibleHistoryCollateralRecordGet200Response sapi_v1_simple_earn_flexible_history_collateral_record_get(timestamp, signature, product_id, start_time, end_time, current, size, recv_window)
Get Collateral Record (USER_DATA)

Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**product_id** | Option<**String**> |  |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SimpleEarnFlexibleHistoryCollateralRecordGet200Response**](_sapi_v1_simple_earn_flexible_history_collateralRecord_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_simple_earn_flexible_history_rate_history_get

> models::SapiV1SimpleEarnFlexibleHistoryRateHistoryGet200Response sapi_v1_simple_earn_flexible_history_rate_history_get(product_id, timestamp, signature, start_time, end_time, current, size, recv_window)
Get Rate History (USER_DATA)

Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SimpleEarnFlexibleHistoryRateHistoryGet200Response**](_sapi_v1_simple_earn_flexible_history_rateHistory_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_simple_earn_flexible_history_redemption_record_get

> models::SapiV1SimpleEarnFlexibleHistoryRedemptionRecordGet200Response sapi_v1_simple_earn_flexible_history_redemption_record_get(product_id, redeem_id, asset, start_time, end_time, current, size)
Get Flexible Redemption Record (USER_DATA)

Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | Option<**String**> |  |  |
**redeem_id** | Option<**String**> |  |  |
**asset** | Option<**String**> |  |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |

### Return type

[**models::SapiV1SimpleEarnFlexibleHistoryRedemptionRecordGet200Response**](_sapi_v1_simple_earn_flexible_history_redemptionRecord_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_simple_earn_flexible_history_rewards_record_get

> models::SapiV1SimpleEarnFlexibleHistoryRewardsRecordGet200Response sapi_v1_simple_earn_flexible_history_rewards_record_get(r#type, product_id, asset, start_time, end_time)
Get Flexible Rewards History (USER_DATA)

Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | \"BONUS\", \"REALTIME\", \"REWARDS\" | [required] |
**product_id** | Option<**String**> |  |  |
**asset** | Option<**String**> |  |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |

### Return type

[**models::SapiV1SimpleEarnFlexibleHistoryRewardsRecordGet200Response**](_sapi_v1_simple_earn_flexible_history_rewardsRecord_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_simple_earn_flexible_history_subscription_record_get

> models::SapiV1SimpleEarnFlexibleHistorySubscriptionRecordGet200Response sapi_v1_simple_earn_flexible_history_subscription_record_get(timestamp, signature, product_id, purchase_id, asset, start_time, end_time, current, size, recv_window)
Get Flexible Subscription Record (USER_DATA)

Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**product_id** | Option<**String**> |  |  |
**purchase_id** | Option<**String**> |  |  |
**asset** | Option<**String**> |  |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SimpleEarnFlexibleHistorySubscriptionRecordGet200Response**](_sapi_v1_simple_earn_flexible_history_subscriptionRecord_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_simple_earn_flexible_list_get

> models::SapiV1SimpleEarnFlexibleListGet200Response sapi_v1_simple_earn_flexible_list_get(timestamp, signature, asset, current, size, recv_window)
Get Simple Earn Flexible Product List (USER_DATA)

Get available Simple Earn flexible product list  Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> |  |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SimpleEarnFlexibleListGet200Response**](_sapi_v1_simple_earn_flexible_list_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_simple_earn_flexible_personal_left_quota_get

> models::SapiV1SimpleEarnFlexiblePersonalLeftQuotaGet200Response sapi_v1_simple_earn_flexible_personal_left_quota_get(product_id, timestamp, signature, recv_window)
Get Flexible Personal Left Quota (USER_DATA)

Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SimpleEarnFlexiblePersonalLeftQuotaGet200Response**](_sapi_v1_simple_earn_flexible_personalLeftQuota_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_simple_earn_flexible_position_get

> models::SapiV1SimpleEarnFlexiblePositionGet200Response sapi_v1_simple_earn_flexible_position_get(timestamp, signature, asset, product_id, current, size, recv_window)
Get Flexible Product Position (USER_DATA)

Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> |  |  |
**product_id** | Option<**String**> |  |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SimpleEarnFlexiblePositionGet200Response**](_sapi_v1_simple_earn_flexible_position_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_simple_earn_flexible_redeem_post

> models::SapiV1SimpleEarnFlexibleRedeemPost200Response sapi_v1_simple_earn_flexible_redeem_post(product_id, timestamp, signature, redeem_all, amount, dest_account, recv_window)
Redeem Flexible Product (TRADE)

Weight(IP): 1  Rate Limit: 1/3s per account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**redeem_all** | Option<**bool**> | true or false, default to false |  |
**amount** | Option<**f64**> | if redeemAll is false, amount is mandatory |  |
**dest_account** | Option<**String**> | SPOT,FUND,ALL, default SPOT |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SimpleEarnFlexibleRedeemPost200Response**](_sapi_v1_simple_earn_flexible_redeem_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_simple_earn_flexible_set_auto_subscribe_post

> models::SapiV1MarginMaxLeveragePost200Response sapi_v1_simple_earn_flexible_set_auto_subscribe_post(product_id, auto_subscribe, timestamp, signature, recv_window)
Set Flexible Auto Subscribe (USER_DATA)

Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |
**auto_subscribe** | **bool** | true or false | [required] |
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


## sapi_v1_simple_earn_flexible_subscribe_post

> models::SapiV1SimpleEarnFlexibleSubscribePost200Response sapi_v1_simple_earn_flexible_subscribe_post(product_id, amount, timestamp, signature, auto_subscribe, source_account, recv_window)
Subscribe Flexible Product (TRADE)

Weight(IP): 1  Rate Limit: 1/3s per account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**auto_subscribe** | Option<**bool**> | true or false, default true. |  |
**source_account** | Option<**String**> | SPOT,FUND,ALL, default SPOT |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SimpleEarnFlexibleSubscribePost200Response**](_sapi_v1_simple_earn_flexible_subscribe_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_simple_earn_flexible_subscription_preview_get

> models::SapiV1SimpleEarnFlexibleSubscriptionPreviewGet200Response sapi_v1_simple_earn_flexible_subscription_preview_get(product_id, amount, timestamp, signature, recv_window)
Get Flexible Subscription Preview (USER_DATA)

Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SimpleEarnFlexibleSubscriptionPreviewGet200Response**](_sapi_v1_simple_earn_flexible_subscriptionPreview_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_simple_earn_locked_history_redemption_record_get

> models::SapiV1SimpleEarnLockedHistoryRedemptionRecordGet200Response sapi_v1_simple_earn_locked_history_redemption_record_get(timestamp, signature, position_id, redeem_id, asset, start_time, end_time, current, size, recv_window)
Get Locked Redemption Record (USER_DATA)

Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**position_id** | Option<**String**> |  |  |
**redeem_id** | Option<**String**> |  |  |
**asset** | Option<**String**> |  |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SimpleEarnLockedHistoryRedemptionRecordGet200Response**](_sapi_v1_simple_earn_locked_history_redemptionRecord_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_simple_earn_locked_history_rewards_record_get

> models::SapiV1SimpleEarnLockedHistoryRewardsRecordGet200Response sapi_v1_simple_earn_locked_history_rewards_record_get(timestamp, signature, position_id, asset, start_time, end_time, size, recv_window)
Get Locked Rewards History (USER_DATA)

Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**position_id** | Option<**String**> |  |  |
**asset** | Option<**String**> |  |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SimpleEarnLockedHistoryRewardsRecordGet200Response**](_sapi_v1_simple_earn_locked_history_rewardsRecord_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_simple_earn_locked_history_subscription_record_get

> models::SapiV1SimpleEarnLockedHistorySubscriptionRecordGet200Response sapi_v1_simple_earn_locked_history_subscription_record_get(timestamp, signature, purchase_id, asset, start_time, end_time, current, size, recv_window)
Get Locked Subscription Record (USER_DATA)

Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**purchase_id** | Option<**String**> |  |  |
**asset** | Option<**String**> |  |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SimpleEarnLockedHistorySubscriptionRecordGet200Response**](_sapi_v1_simple_earn_locked_history_subscriptionRecord_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_simple_earn_locked_list_get

> models::SapiV1SimpleEarnLockedListGet200Response sapi_v1_simple_earn_locked_list_get(timestamp, signature, asset, current, size, recv_window)
Get Simple Earn Locked Product List (USER_DATA)

Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> |  |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SimpleEarnLockedListGet200Response**](_sapi_v1_simple_earn_locked_list_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_simple_earn_locked_personal_left_quota_get

> models::SapiV1SimpleEarnFlexiblePersonalLeftQuotaGet200Response sapi_v1_simple_earn_locked_personal_left_quota_get(project_id, timestamp, signature, recv_window)
Get Locked Personal Left Quota (USER_DATA)

Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SimpleEarnFlexiblePersonalLeftQuotaGet200Response**](_sapi_v1_simple_earn_flexible_personalLeftQuota_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_simple_earn_locked_position_get

> models::SapiV1SimpleEarnLockedPositionGet200Response sapi_v1_simple_earn_locked_position_get(timestamp, signature, asset, position_id, project_id, current, size, recv_window)
Get Locked Product Position (USER_DATA)

Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> |  |  |
**position_id** | Option<**String**> |  |  |
**project_id** | Option<**String**> |  |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SimpleEarnLockedPositionGet200Response**](_sapi_v1_simple_earn_locked_position_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_simple_earn_locked_redeem_post

> models::SapiV1SimpleEarnFlexibleRedeemPost200Response sapi_v1_simple_earn_locked_redeem_post(position_id, timestamp, signature, recv_window)
Redeem Locked Product (TRADE)

Weight(IP): 1  Rate Limit: 1/3s per account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**position_id** | **String** | 1234 | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SimpleEarnFlexibleRedeemPost200Response**](_sapi_v1_simple_earn_flexible_redeem_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_simple_earn_locked_set_auto_subscribe_post

> models::SapiV1MarginMaxLeveragePost200Response sapi_v1_simple_earn_locked_set_auto_subscribe_post(position_id, auto_subscribe, timestamp, signature, recv_window)
Set Locked Auto Subscribe (USER_DATA)

Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**position_id** | **String** |  | [required] |
**auto_subscribe** | **bool** | true or false | [required] |
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


## sapi_v1_simple_earn_locked_subscribe_post

> models::SapiV1SimpleEarnLockedSubscribePost200Response sapi_v1_simple_earn_locked_subscribe_post(project_id, amount, timestamp, signature, auto_subscribe, source_account, recv_window)
Subscribe Locked Product (TRADE)

Weight(IP): 1  Rate Limit: 1/3s per account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**auto_subscribe** | Option<**bool**> | true or false, default true. |  |
**source_account** | Option<**String**> | SPOT,FUND,ALL, default SPOT |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1SimpleEarnLockedSubscribePost200Response**](_sapi_v1_simple_earn_locked_subscribe_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_simple_earn_locked_subscription_preview_get

> Vec<models::SapiV1SimpleEarnLockedSubscriptionPreviewGet200ResponseInner> sapi_v1_simple_earn_locked_subscription_preview_get(project_id, amount, timestamp, signature, auto_subscribe, recv_window)
Get Locked Subscription Preview (USER_DATA)

Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**auto_subscribe** | Option<**bool**> | true or false, default true. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1SimpleEarnLockedSubscriptionPreviewGet200ResponseInner>**](_sapi_v1_simple_earn_locked_subscriptionPreview_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

