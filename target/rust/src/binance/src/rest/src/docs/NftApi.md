# \NftApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_nft_history_deposit_get**](NftApi.md#sapi_v1_nft_history_deposit_get) | **GET** /sapi/v1/nft/history/deposit | Get NFT Deposit History(USER_DATA)
[**sapi_v1_nft_history_transactions_get**](NftApi.md#sapi_v1_nft_history_transactions_get) | **GET** /sapi/v1/nft/history/transactions | Get NFT Transaction History (USER_DATA)
[**sapi_v1_nft_history_withdraw_get**](NftApi.md#sapi_v1_nft_history_withdraw_get) | **GET** /sapi/v1/nft/history/withdraw | Get NFT Withdraw History (USER_DATA)
[**sapi_v1_nft_user_get_asset_get**](NftApi.md#sapi_v1_nft_user_get_asset_get) | **GET** /sapi/v1/nft/user/getAsset | Get NFT Asset (USER_DATA)



## sapi_v1_nft_history_deposit_get

> models::SapiV1NftHistoryDepositGet200Response sapi_v1_nft_history_deposit_get(timestamp, signature, start_time, end_time, limit, page, recv_window)
Get NFT Deposit History(USER_DATA)

- The max interval between startTime and endTime is 90 days. - If startTime and endTime are not sent, the recent 7 days' data will be returned.  Weight(UID): 3000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i32**> | Default 50, Max 50 |  |
**page** | Option<**i32**> | Default 1 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1NftHistoryDepositGet200Response**](_sapi_v1_nft_history_deposit_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_nft_history_transactions_get

> models::SapiV1NftHistoryTransactionsGet200Response sapi_v1_nft_history_transactions_get(order_type, timestamp, signature, start_time, end_time, limit, page, recv_window)
Get NFT Transaction History (USER_DATA)

- The max interval between startTime and endTime is 90 days. - If startTime and endTime are not sent, the recent 7 days' data will be returned.  Weight(UID): 3000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_type** | **i32** | 0: purchase order, 1: sell order, 2: royalty income, 3: primary market order, 4: mint fee | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i32**> | Default 50, Max 50 |  |
**page** | Option<**i32**> | Default 1 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1NftHistoryTransactionsGet200Response**](_sapi_v1_nft_history_transactions_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_nft_history_withdraw_get

> models::SapiV1NftHistoryWithdrawGet200Response sapi_v1_nft_history_withdraw_get(timestamp, signature, start_time, end_time, limit, page, recv_window)
Get NFT Withdraw History (USER_DATA)

- The max interval between startTime and endTime is 90 days. - If startTime and endTime are not sent, the recent 7 days' data will be returned.  Weight(UID): 3000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i32**> | Default 50, Max 50 |  |
**page** | Option<**i32**> | Default 1 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1NftHistoryWithdrawGet200Response**](_sapi_v1_nft_history_withdraw_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_nft_user_get_asset_get

> models::SapiV1NftUserGetAssetGet200Response sapi_v1_nft_user_get_asset_get(timestamp, signature, limit, page, recv_window)
Get NFT Asset (USER_DATA)

Weight(UID): 3000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**limit** | Option<**i32**> | Default 50, Max 50 |  |
**page** | Option<**i32**> | Default 1 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1NftUserGetAssetGet200Response**](_sapi_v1_nft_user_getAsset_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

