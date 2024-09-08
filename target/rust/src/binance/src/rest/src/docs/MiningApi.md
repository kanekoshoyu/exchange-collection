# \MiningApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_mining_hash_transfer_config_cancel_post**](MiningApi.md#sapi_v1_mining_hash_transfer_config_cancel_post) | **POST** /sapi/v1/mining/hash-transfer/config/cancel | Cancel Hashrate Resale configuration (USER_DATA)
[**sapi_v1_mining_hash_transfer_config_details_list_get**](MiningApi.md#sapi_v1_mining_hash_transfer_config_details_list_get) | **GET** /sapi/v1/mining/hash-transfer/config/details/list | Hashrate Resale List (USER_DATA)
[**sapi_v1_mining_hash_transfer_config_post**](MiningApi.md#sapi_v1_mining_hash_transfer_config_post) | **POST** /sapi/v1/mining/hash-transfer/config | Hashrate Resale Request (USER_DATA)
[**sapi_v1_mining_hash_transfer_profit_details_get**](MiningApi.md#sapi_v1_mining_hash_transfer_profit_details_get) | **GET** /sapi/v1/mining/hash-transfer/profit/details | Hashrate Resale Details (USER_DATA)
[**sapi_v1_mining_payment_list_get**](MiningApi.md#sapi_v1_mining_payment_list_get) | **GET** /sapi/v1/mining/payment/list | Earnings List (USER_DATA)
[**sapi_v1_mining_payment_other_get**](MiningApi.md#sapi_v1_mining_payment_other_get) | **GET** /sapi/v1/mining/payment/other | Extra Bonus List (USER_DATA)
[**sapi_v1_mining_payment_uid_get**](MiningApi.md#sapi_v1_mining_payment_uid_get) | **GET** /sapi/v1/mining/payment/uid | Mining Account Earning (USER_DATA)
[**sapi_v1_mining_pub_algo_list_get**](MiningApi.md#sapi_v1_mining_pub_algo_list_get) | **GET** /sapi/v1/mining/pub/algoList | Acquiring Algorithm (MARKET_DATA)
[**sapi_v1_mining_pub_coin_list_get**](MiningApi.md#sapi_v1_mining_pub_coin_list_get) | **GET** /sapi/v1/mining/pub/coinList | Acquiring CoinName (MARKET_DATA)
[**sapi_v1_mining_statistics_user_list_get**](MiningApi.md#sapi_v1_mining_statistics_user_list_get) | **GET** /sapi/v1/mining/statistics/user/list | Account List (USER_DATA)
[**sapi_v1_mining_statistics_user_status_get**](MiningApi.md#sapi_v1_mining_statistics_user_status_get) | **GET** /sapi/v1/mining/statistics/user/status | Statistic List (USER_DATA)
[**sapi_v1_mining_worker_detail_get**](MiningApi.md#sapi_v1_mining_worker_detail_get) | **GET** /sapi/v1/mining/worker/detail | Request for Detail Miner List (USER_DATA)
[**sapi_v1_mining_worker_list_get**](MiningApi.md#sapi_v1_mining_worker_list_get) | **GET** /sapi/v1/mining/worker/list | Request for Miner List (USER_DATA)



## sapi_v1_mining_hash_transfer_config_cancel_post

> models::SapiV1MiningHashTransferConfigCancelPost200Response sapi_v1_mining_hash_transfer_config_cancel_post(config_id, user_name, timestamp, signature, recv_window)
Cancel Hashrate Resale configuration (USER_DATA)

Weight(IP): 5

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** | Mining ID | [required] |
**user_name** | **String** | Mining Account | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MiningHashTransferConfigCancelPost200Response**](_sapi_v1_mining_hash_transfer_config_cancel_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_mining_hash_transfer_config_details_list_get

> models::SapiV1MiningHashTransferConfigDetailsListGet200Response sapi_v1_mining_hash_transfer_config_details_list_get(timestamp, signature, page_index, page_size, recv_window)
Hashrate Resale List (USER_DATA)

Weight(IP): 5

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**page_index** | Option<**i32**> | Page number, default is first page, start form 1 |  |
**page_size** | Option<**String**> | Number of pages, minimum 10, maximum 200 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MiningHashTransferConfigDetailsListGet200Response**](_sapi_v1_mining_hash_transfer_config_details_list_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_mining_hash_transfer_config_post

> models::SapiV1MiningHashTransferConfigPost200Response sapi_v1_mining_hash_transfer_config_post(user_name, algo, to_pool_user, hash_rate, timestamp, signature, start_date, end_date, recv_window)
Hashrate Resale Request (USER_DATA)

Weight(IP): 5

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_name** | **String** | Mining Account | [required] |
**algo** | **String** | Algorithm(sha256) | [required] |
**to_pool_user** | **String** | Mining Account | [required] |
**hash_rate** | **String** | Resale hashrate h/s must be transferred (BTC is greater than 500000000000 ETH is greater than 500000) | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**start_date** | Option<**String**> | Search date, millisecond timestamp, while empty query all |  |
**end_date** | Option<**String**> | Search date, millisecond timestamp, while empty query all |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MiningHashTransferConfigPost200Response**](_sapi_v1_mining_hash_transfer_config_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_mining_hash_transfer_profit_details_get

> models::SapiV1MiningHashTransferProfitDetailsGet200Response sapi_v1_mining_hash_transfer_profit_details_get(config_id, user_name, timestamp, signature, page_index, page_size, recv_window)
Hashrate Resale Details (USER_DATA)

Weight(IP): 5

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** | Mining ID | [required] |
**user_name** | **String** | Mining Account | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**page_index** | Option<**i32**> | Page number, default is first page, start form 1 |  |
**page_size** | Option<**String**> | Number of pages, minimum 10, maximum 200 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MiningHashTransferProfitDetailsGet200Response**](_sapi_v1_mining_hash_transfer_profit_details_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_mining_payment_list_get

> models::SapiV1MiningPaymentListGet200Response sapi_v1_mining_payment_list_get(algo, user_name, timestamp, signature, coin, start_date, end_date, page_index, page_size, recv_window)
Earnings List (USER_DATA)

Weight(IP): 5

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**algo** | **String** | Algorithm(sha256) | [required] |
**user_name** | **String** | Mining Account | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**coin** | Option<**String**> | Coin name |  |
**start_date** | Option<**String**> | Search date, millisecond timestamp, while empty query all |  |
**end_date** | Option<**String**> | Search date, millisecond timestamp, while empty query all |  |
**page_index** | Option<**i32**> | Page number, default is first page, start form 1 |  |
**page_size** | Option<**String**> | Number of pages, minimum 10, maximum 200 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MiningPaymentListGet200Response**](_sapi_v1_mining_payment_list_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_mining_payment_other_get

> models::SapiV1MiningPaymentOtherGet200Response sapi_v1_mining_payment_other_get(algo, user_name, timestamp, signature, coin, start_date, end_date, page_index, page_size, recv_window)
Extra Bonus List (USER_DATA)

Weight(IP): 5

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**algo** | **String** | Algorithm(sha256) | [required] |
**user_name** | **String** | Mining Account | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**coin** | Option<**String**> | Coin name |  |
**start_date** | Option<**String**> | Search date, millisecond timestamp, while empty query all |  |
**end_date** | Option<**String**> | Search date, millisecond timestamp, while empty query all |  |
**page_index** | Option<**i32**> | Page number, default is first page, start form 1 |  |
**page_size** | Option<**String**> | Number of pages, minimum 10, maximum 200 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MiningPaymentOtherGet200Response**](_sapi_v1_mining_payment_other_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_mining_payment_uid_get

> models::SapiV1MiningPaymentUidGet200Response sapi_v1_mining_payment_uid_get(algo, timestamp, signature, start_date, end_date, page_index, page_size, recv_window)
Mining Account Earning (USER_DATA)

Weight(IP): 5

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**algo** | **String** | Algorithm(sha256) | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**start_date** | Option<**String**> | Search date, millisecond timestamp, while empty query all |  |
**end_date** | Option<**String**> | Search date, millisecond timestamp, while empty query all |  |
**page_index** | Option<**i32**> | Page number, default is first page, start form 1 |  |
**page_size** | Option<**String**> | Number of pages, minimum 10, maximum 200 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MiningPaymentUidGet200Response**](_sapi_v1_mining_payment_uid_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_mining_pub_algo_list_get

> models::SapiV1MiningPubAlgoListGet200Response sapi_v1_mining_pub_algo_list_get()
Acquiring Algorithm (MARKET_DATA)

Weight(IP): 1

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SapiV1MiningPubAlgoListGet200Response**](_sapi_v1_mining_pub_algoList_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_mining_pub_coin_list_get

> models::SapiV1MiningPubCoinListGet200Response sapi_v1_mining_pub_coin_list_get()
Acquiring CoinName (MARKET_DATA)

Weight(IP): 1

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SapiV1MiningPubCoinListGet200Response**](_sapi_v1_mining_pub_coinList_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_mining_statistics_user_list_get

> models::SapiV1MiningStatisticsUserListGet200Response sapi_v1_mining_statistics_user_list_get(algo, user_name, timestamp, signature, recv_window)
Account List (USER_DATA)

Weight(IP): 5

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**algo** | **String** | Algorithm(sha256) | [required] |
**user_name** | **String** | Mining Account | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MiningStatisticsUserListGet200Response**](_sapi_v1_mining_statistics_user_list_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_mining_statistics_user_status_get

> models::SapiV1MiningStatisticsUserStatusGet200Response sapi_v1_mining_statistics_user_status_get(algo, user_name, timestamp, signature, recv_window)
Statistic List (USER_DATA)

Weight(IP): 5

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**algo** | **String** | Algorithm(sha256) | [required] |
**user_name** | **String** | Mining Account | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MiningStatisticsUserStatusGet200Response**](_sapi_v1_mining_statistics_user_status_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_mining_worker_detail_get

> models::SapiV1MiningWorkerDetailGet200Response sapi_v1_mining_worker_detail_get(algo, user_name, worker_name, timestamp, signature, recv_window)
Request for Detail Miner List (USER_DATA)

Weight(IP): 5

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**algo** | **String** | Algorithm(sha256) | [required] |
**user_name** | **String** | Mining Account | [required] |
**worker_name** | **String** | Miner’s name | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MiningWorkerDetailGet200Response**](_sapi_v1_mining_worker_detail_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_mining_worker_list_get

> models::SapiV1MiningWorkerListGet200Response sapi_v1_mining_worker_list_get(algo, user_name, timestamp, signature, page_index, sort, sort_column, worker_status, recv_window)
Request for Miner List (USER_DATA)

Weight(IP): 5

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**algo** | **String** | Algorithm(sha256) | [required] |
**user_name** | **String** | Mining Account | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**page_index** | Option<**i32**> | Page number, default is first page, start form 1 |  |
**sort** | Option<**i32**> | sort sequence(default=0)0 positive sequence, 1 negative sequence |  |
**sort_column** | Option<**i32**> | Sort by( default 1): 1: miner name, 2: real-time computing power, 3: daily average computing power, 4: real-time rejection rate, 5: last submission time |  |
**worker_status** | Option<**i32**> | miners status(default=0)0 all, 1 valid, 2 invalid, 3 failure |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1MiningWorkerListGet200Response**](_sapi_v1_mining_worker_list_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

