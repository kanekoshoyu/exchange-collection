# \DualInvestmentApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_dci_product_accounts_get**](DualInvestmentApi.md#sapi_v1_dci_product_accounts_get) | **GET** /sapi/v1/dci/product/accounts | Check Dual Investment accounts(USER_DATA)
[**sapi_v1_dci_product_auto_compound_edit_status_post**](DualInvestmentApi.md#sapi_v1_dci_product_auto_compound_edit_status_post) | **POST** /sapi/v1/dci/product/auto_compound/edit-status | Change Auto-Compound status(USER_DATA)
[**sapi_v1_dci_product_list_get**](DualInvestmentApi.md#sapi_v1_dci_product_list_get) | **GET** /sapi/v1/dci/product/list | Get Dual Investment product list(USER_DATA)
[**sapi_v1_dci_product_positions_get**](DualInvestmentApi.md#sapi_v1_dci_product_positions_get) | **GET** /sapi/v1/dci/product/positions | Get Dual Investment positions(USER_DATA)
[**sapi_v1_dci_product_subscribe_post**](DualInvestmentApi.md#sapi_v1_dci_product_subscribe_post) | **POST** /sapi/v1/dci/product/subscribe | Subscribe Dual Investment products(USER_DATA)



## sapi_v1_dci_product_accounts_get

> models::SapiV1DciProductAccountsGet200Response sapi_v1_dci_product_accounts_get(timestamp, signature, recv_window)
Check Dual Investment accounts(USER_DATA)

Check Dual Investment accounts  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1DciProductAccountsGet200Response**](_sapi_v1_dci_product_accounts_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_dci_product_auto_compound_edit_status_post

> models::SapiV1DciProductAutoCompoundEditStatusPost200Response sapi_v1_dci_product_auto_compound_edit_status_post(position_id, auto_compound_plan, timestamp, signature, recv_window)
Change Auto-Compound status(USER_DATA)

Change Auto-Compound status  - 15:31 ~ 16:00 UTC+8 This function is disabled  Weight(IP): 1  Rate Limit: Maximum 1 time/s per account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**position_id** | **i64** | Get positionId from /sapi/v1/dci/product/positions | [required] |
**auto_compound_plan** | **String** | NONE: switch off the plan, STANDARD: standard plan, ADVANCED: advanced plan; | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1DciProductAutoCompoundEditStatusPost200Response**](_sapi_v1_dci_product_auto_compound_edit_status_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_dci_product_list_get

> models::SapiV1DciProductListGet200Response sapi_v1_dci_product_list_get(option_type, exercised_coin, invest_coin, timestamp, signature, page_size, page_index, recv_window)
Get Dual Investment product list(USER_DATA)

Get Dual Investment product list  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**option_type** | **String** | Input CALL or PUT | [required] |
**exercised_coin** | **String** | Target exercised asset, e.g.:  if you subscribe to a high sell product (call option), you should input:    - optionType: CALL,   - exercisedCoin: USDT,   - investCoin: BNB;   if you subscribe to a low buy product (put option), you should input:   - optionType: PUT,   - exercisedCoin: BNB,   - investCoin: USDT; | [required] |
**invest_coin** | **String** | Asset used for subscribing, e.g.:  if you subscribe to a high sell product (call option), you should input:    - optionType: CALL,   - exercisedCoin: USDT,   - investCoin: BNB;   if you subscribe to a low buy product (put option), you should input:   - optionType: PUT,   - exercisedCoin: BNB,   - investCoin: USDT; | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**page_size** | Option<**String**> | MIN 1, MAX 100; Default 100 |  |
**page_index** | Option<**i32**> | Page number, default is first page, start form 1 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1DciProductListGet200Response**](_sapi_v1_dci_product_list_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_dci_product_positions_get

> models::SapiV1DciProductPositionsGet200Response sapi_v1_dci_product_positions_get(timestamp, signature, status, page_size, page_index, recv_window)
Get Dual Investment positions(USER_DATA)

Get Dual Investment positions (batch)  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**status** | Option<**String**> | - PENDING: Products are purchasing, will give results later; - PURCHASE_SUCCESS: purchase successfully; - SETTLED: Products are finish settling; - PURCHASE_FAIL: fail to purchase; - REFUNDING: refund ongoing; - REFUND_SUCCESS: refund to spot account successfully; - SETTLING: Products are settling.  If don't fill this field, will response all the position status. |  |
**page_size** | Option<**String**> | MIN 1, MAX 100; Default 100 |  |
**page_index** | Option<**i32**> | Page number, default is first page, start form 1 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1DciProductPositionsGet200Response**](_sapi_v1_dci_product_positions_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_dci_product_subscribe_post

> models::SapiV1DciProductSubscribePost200Response sapi_v1_dci_product_subscribe_post(id, order_id, deposit_amount, auto_compound_plan, timestamp, signature, recv_window)
Subscribe Dual Investment products(USER_DATA)

Subscribe Dual Investment products  - `Products are not available.` means that the APR changes to lower value, or the orders are not available. - `Failed` is a system or network errors.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | get id from /sapi/v1/dci/product/list | [required] |
**order_id** | **String** | get orderId from /sapi/v1/dci/product/list | [required] |
**deposit_amount** | **f64** |  | [required] |
**auto_compound_plan** | **String** | NONE: switch off the plan, STANDARD: standard plan, ADVANCED: advanced plan; | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1DciProductSubscribePost200Response**](_sapi_v1_dci_product_subscribe_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

