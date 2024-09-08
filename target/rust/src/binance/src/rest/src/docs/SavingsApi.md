# \SavingsApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_lending_customized_fixed_purchase_post**](SavingsApi.md#sapi_v1_lending_customized_fixed_purchase_post) | **POST** /sapi/v1/lending/customizedFixed/purchase | Purchase Fixed/Activity Project (USER_DATA)
[**sapi_v1_lending_position_changed_post**](SavingsApi.md#sapi_v1_lending_position_changed_post) | **POST** /sapi/v1/lending/positionChanged | Change Fixed/Activity Position to Daily Position (USER_DATA)
[**sapi_v1_lending_project_list_get**](SavingsApi.md#sapi_v1_lending_project_list_get) | **GET** /sapi/v1/lending/project/list | Get Fixed/Activity Project List(USER_DATA)
[**sapi_v1_lending_project_position_list_get**](SavingsApi.md#sapi_v1_lending_project_position_list_get) | **GET** /sapi/v1/lending/project/position/list | Get Fixed/Activity Project Position (USER_DATA)



## sapi_v1_lending_customized_fixed_purchase_post

> models::SapiV1LendingCustomizedFixedPurchasePost200Response sapi_v1_lending_customized_fixed_purchase_post(project_id, lot, timestamp, signature, recv_window)
Purchase Fixed/Activity Project (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**lot** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LendingCustomizedFixedPurchasePost200Response**](_sapi_v1_lending_customizedFixed_purchase_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_position_changed_post

> models::SapiV1LendingPositionChangedPost200Response sapi_v1_lending_position_changed_post(project_id, lot, timestamp, signature, position_id, recv_window)
Change Fixed/Activity Position to Daily Position (USER_DATA)

- PositionId is mandatory parameter for fixed position.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**lot** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**position_id** | Option<**String**> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1LendingPositionChangedPost200Response**](_sapi_v1_lending_positionChanged_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_project_list_get

> Vec<models::SapiV1LendingProjectListGet200ResponseInner> sapi_v1_lending_project_list_get(r#type, timestamp, signature, asset, status, is_sort_asc, sort_by, current, size, recv_window)
Get Fixed/Activity Project List(USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> |  |  |
**status** | Option<**String**> | Default `ALL` |  |
**is_sort_asc** | Option<**bool**> | default \"true\" |  |
**sort_by** | Option<**String**> | Default `START_TIME` |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1LendingProjectListGet200ResponseInner>**](_sapi_v1_lending_project_list_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_project_position_list_get

> Vec<models::SapiV1LendingProjectPositionListGet200ResponseInner> sapi_v1_lending_project_position_list_get(asset, timestamp, signature, project_id, status, recv_window)
Get Fixed/Activity Project Position (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**project_id** | Option<**String**> |  |  |
**status** | Option<**String**> | Default `ALL` |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<models::SapiV1LendingProjectPositionListGet200ResponseInner>**](_sapi_v1_lending_project_position_list_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

