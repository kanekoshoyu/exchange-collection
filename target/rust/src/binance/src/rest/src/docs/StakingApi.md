# \StakingApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_eth_staking_eth_history_rate_history_get**](StakingApi.md#sapi_v1_eth_staking_eth_history_rate_history_get) | **GET** /sapi/v1/eth-staking/eth/history/rateHistory | Get WBETH Rate History (USER_DATA)
[**sapi_v1_eth_staking_eth_history_redemption_history_get**](StakingApi.md#sapi_v1_eth_staking_eth_history_redemption_history_get) | **GET** /sapi/v1/eth-staking/eth/history/redemptionHistory | Get ETH redemption history (USER_DATA)
[**sapi_v1_eth_staking_eth_history_rewards_history_get**](StakingApi.md#sapi_v1_eth_staking_eth_history_rewards_history_get) | **GET** /sapi/v1/eth-staking/eth/history/rewardsHistory | Get BETH rewards distribution history(USER_DATA)
[**sapi_v1_eth_staking_eth_history_staking_history_get**](StakingApi.md#sapi_v1_eth_staking_eth_history_staking_history_get) | **GET** /sapi/v1/eth-staking/eth/history/stakingHistory | Get ETH staking history (USER_DATA)
[**sapi_v1_eth_staking_eth_history_wbeth_rewards_history_get**](StakingApi.md#sapi_v1_eth_staking_eth_history_wbeth_rewards_history_get) | **GET** /sapi/v1/eth-staking/eth/history/wbethRewardsHistory | Get WBETH rewards history(USER_DATA)
[**sapi_v1_eth_staking_eth_quota_get**](StakingApi.md#sapi_v1_eth_staking_eth_quota_get) | **GET** /sapi/v1/eth-staking/eth/quota | Get current ETH staking quota (USER_DATA)
[**sapi_v1_eth_staking_eth_redeem_post**](StakingApi.md#sapi_v1_eth_staking_eth_redeem_post) | **POST** /sapi/v1/eth-staking/eth/redeem | Redeem ETH (TRADE)
[**sapi_v1_eth_staking_wbeth_history_unwrap_history_get**](StakingApi.md#sapi_v1_eth_staking_wbeth_history_unwrap_history_get) | **GET** /sapi/v1/eth-staking/wbeth/history/unwrapHistory | Get WBETH unwrap history (USER_DATA)
[**sapi_v1_eth_staking_wbeth_history_wrap_history_get**](StakingApi.md#sapi_v1_eth_staking_wbeth_history_wrap_history_get) | **GET** /sapi/v1/eth-staking/wbeth/history/wrapHistory | Get WBETH wrap history (USER_DATA)
[**sapi_v1_eth_staking_wbeth_wrap_post**](StakingApi.md#sapi_v1_eth_staking_wbeth_wrap_post) | **POST** /sapi/v1/eth-staking/wbeth/wrap | Wrap BETH(TRADE)
[**sapi_v2_eth_staking_account_get**](StakingApi.md#sapi_v2_eth_staking_account_get) | **GET** /sapi/v2/eth-staking/account | ETH Staking account V2(USER_DATA)
[**sapi_v2_eth_staking_eth_stake_post**](StakingApi.md#sapi_v2_eth_staking_eth_stake_post) | **POST** /sapi/v2/eth-staking/eth/stake | Subscribe ETH Staking V2(TRADE)



## sapi_v1_eth_staking_eth_history_rate_history_get

> models::SapiV1EthStakingEthHistoryRateHistoryGet200Response sapi_v1_eth_staking_eth_history_rate_history_get(timestamp, signature, start_time, end_time, current, size, recv_window)
Get WBETH Rate History (USER_DATA)

- The time between startTime and endTime cannot be longer than 3 months. - If startTime and endTime are both not sent, then the last 30 days' data will be returned. - If startTime is sent but endTime is not sent, the next 30 days' data beginning from startTime will be returned. - If endTime is sent but startTime is not sent, the 30 days' data before endTime will be returned.  Weight(IP): 150

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

[**models::SapiV1EthStakingEthHistoryRateHistoryGet200Response**](_sapi_v1_eth_staking_eth_history_rateHistory_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_eth_staking_eth_history_redemption_history_get

> models::SapiV1EthStakingEthHistoryRedemptionHistoryGet200Response sapi_v1_eth_staking_eth_history_redemption_history_get(timestamp, signature, start_time, end_time, current, size, recv_window)
Get ETH redemption history (USER_DATA)

- The time between startTime and endTime cannot be longer than 3 months. - If startTime and endTime are both not sent, then the last 30 days' data will be returned. - If startTime is sent but endTime is not sent, the next 30 days' data beginning from startTime will be returned. - If endTime is sent but startTime is not sent, the 30 days' data before endTime will be returned.  Weight(IP): 150

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

[**models::SapiV1EthStakingEthHistoryRedemptionHistoryGet200Response**](_sapi_v1_eth_staking_eth_history_redemptionHistory_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_eth_staking_eth_history_rewards_history_get

> models::SapiV1EthStakingEthHistoryRewardsHistoryGet200Response sapi_v1_eth_staking_eth_history_rewards_history_get(timestamp, signature, start_time, end_time, current, size, recv_window)
Get BETH rewards distribution history(USER_DATA)

- The time between startTime and endTime cannot be longer than 3 months. - If startTime and endTime are both not sent, then the last 30 days' data will be returned. - If startTime is sent but endTime is not sent, the next 30 days' data beginning from startTime will be returned. - If endTime is sent but startTime is not sent, the 30 days' data before endTime will be returned.  Weight(IP): 150

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

[**models::SapiV1EthStakingEthHistoryRewardsHistoryGet200Response**](_sapi_v1_eth_staking_eth_history_rewardsHistory_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_eth_staking_eth_history_staking_history_get

> models::SapiV1EthStakingEthHistoryStakingHistoryGet200Response sapi_v1_eth_staking_eth_history_staking_history_get(timestamp, signature, start_time, end_time, current, size, recv_window)
Get ETH staking history (USER_DATA)

- The time between startTime and endTime cannot be longer than 3 months. - If startTime and endTime are both not sent, then the last 30 days' data will be returned. - If startTime is sent but endTime is not sent, the next 30 days' data beginning from startTime will be returned. - If endTime is sent but startTime is not sent, the 30 days' data before endTime will be returned.  Weight(IP): 150

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

[**models::SapiV1EthStakingEthHistoryStakingHistoryGet200Response**](_sapi_v1_eth_staking_eth_history_stakingHistory_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_eth_staking_eth_history_wbeth_rewards_history_get

> models::SapiV1EthStakingEthHistoryWbethRewardsHistoryGet200Response sapi_v1_eth_staking_eth_history_wbeth_rewards_history_get(timestamp, signature, start_time, end_time, current, size, recv_window)
Get WBETH rewards history(USER_DATA)

- The time between startTime and endTime cannot be longer than 3 months. - If startTime and endTime are both not sent, then the last 30 days' data will be returned. - If startTime is sent but endTime is not sent, the next 30 days' data beginning from startTime will be returned. - If endTime is sent but startTime is not sent, the 30 days' data before endTime will be returned.  Weight(IP): 150

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

[**models::SapiV1EthStakingEthHistoryWbethRewardsHistoryGet200Response**](_sapi_v1_eth_staking_eth_history_wbethRewardsHistory_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_eth_staking_eth_quota_get

> models::SapiV1EthStakingEthQuotaGet200Response sapi_v1_eth_staking_eth_quota_get(timestamp, signature, recv_window)
Get current ETH staking quota (USER_DATA)

Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1EthStakingEthQuotaGet200Response**](_sapi_v1_eth_staking_eth_quota_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_eth_staking_eth_redeem_post

> models::SapiV1EthStakingEthRedeemPost200Response sapi_v1_eth_staking_eth_redeem_post(amount, timestamp, signature, asset, recv_window)
Redeem ETH (TRADE)

Redeem WBETH or BETH and get ETH  - You need to open Enable Spot & Margin Trading permission for the API Key which requests this endpoint.  Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**amount** | **f64** | Amount in BETH, limit 8 decimals | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> | WBETH or BETH, default to BETH |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1EthStakingEthRedeemPost200Response**](_sapi_v1_eth_staking_eth_redeem_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_eth_staking_wbeth_history_unwrap_history_get

> models::SapiV1EthStakingWbethHistoryUnwrapHistoryGet200Response sapi_v1_eth_staking_wbeth_history_unwrap_history_get(timestamp, signature, start_time, end_time, current, size, recv_window)
Get WBETH unwrap history (USER_DATA)

- The time between startTime and endTime cannot be longer than 3 months. - If startTime and endTime are both not sent, then the last 30 days' data will be returned. - If startTime is sent but endTime is not sent, the next 30 days' data beginning from startTime will be returned. - If endTime is sent but startTime is not sent, the 30 days' data before endTime will be returned.  Weight(IP): 150

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

[**models::SapiV1EthStakingWbethHistoryUnwrapHistoryGet200Response**](_sapi_v1_eth_staking_wbeth_history_unwrapHistory_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_eth_staking_wbeth_history_wrap_history_get

> models::SapiV1EthStakingWbethHistoryWrapHistoryGet200Response sapi_v1_eth_staking_wbeth_history_wrap_history_get(timestamp, signature, start_time, end_time, current, size, recv_window)
Get WBETH wrap history (USER_DATA)

- The time between startTime and endTime cannot be longer than 3 months. - If startTime and endTime are both not sent, then the last 30 days' data will be returned. - If startTime is sent but endTime is not sent, the next 30 days' data beginning from startTime will be returned. - If endTime is sent but startTime is not sent, the 30 days' data before endTime will be returned.  Weight(IP): 150

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

[**models::SapiV1EthStakingWbethHistoryWrapHistoryGet200Response**](_sapi_v1_eth_staking_wbeth_history_wrapHistory_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_eth_staking_wbeth_wrap_post

> models::SapiV1EthStakingWbethWrapPost200Response sapi_v1_eth_staking_wbeth_wrap_post(amount, timestamp, signature, recv_window)
Wrap BETH(TRADE)

- You need to open Enable Spot & Margin Trading permission for the API Key which requests this endpoint.  Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**amount** | **f64** | Amount in BETH, limit 4 decimals | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1EthStakingWbethWrapPost200Response**](_sapi_v1_eth_staking_wbeth_wrap_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v2_eth_staking_account_get

> models::SapiV2EthStakingAccountGet200Response sapi_v2_eth_staking_account_get(timestamp, signature, recv_window)
ETH Staking account V2(USER_DATA)

Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV2EthStakingAccountGet200Response**](_sapi_v2_eth_staking_account_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v2_eth_staking_eth_stake_post

> models::SapiV2EthStakingEthStakePost200Response sapi_v2_eth_staking_eth_stake_post(amount, timestamp, signature, recv_window)
Subscribe ETH Staking V2(TRADE)

Stake ETH to get WBETH  - You need to open Enable Spot & Margin Trading permission for the API Key which requests this endpoint.  Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**amount** | **f64** | Amount in ETH, limit 4 decimals | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV2EthStakingEthStakePost200Response**](_sapi_v2_eth_staking_eth_stake_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

