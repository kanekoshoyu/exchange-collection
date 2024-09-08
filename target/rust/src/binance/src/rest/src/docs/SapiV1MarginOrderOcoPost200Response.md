# SapiV1MarginOrderOcoPost200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_list_id** | **i64** |  | 
**contingency_type** | **String** |  | 
**list_status_type** | **String** |  | 
**list_order_status** | **String** |  | 
**list_client_order_id** | **String** |  | 
**transaction_time** | **i64** |  | 
**symbol** | **String** |  | 
**margin_buy_borrow_amount** | **String** | will not return if no margin trade happens | 
**margin_buy_borrow_asset** | **String** | will not return if no margin trade happens | 
**is_isolated** | **bool** |  | 
**orders** | [**Vec<models::SapiV1MarginOrderOcoPost200ResponseOrdersInner>**](_sapi_v1_margin_order_oco_post_200_response_orders_inner.md) |  | 
**order_reports** | [**Vec<models::SapiV1MarginOrderOcoPost200ResponseOrderReportsInner>**](_sapi_v1_margin_order_oco_post_200_response_orderReports_inner.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


