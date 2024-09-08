# ApiV3OpenOrdersDelete200ResponseInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **String** |  | 
**orig_client_order_id** | **String** |  | 
**order_id** | **i64** |  | 
**order_list_id** | **i64** |  | 
**client_order_id** | **String** |  | 
**transact_time** | **i64** |  | 
**price** | **String** |  | 
**orig_qty** | **String** |  | 
**executed_qty** | **String** |  | 
**cummulative_quote_qty** | **String** |  | 
**status** | **String** |  | 
**time_in_force** | **String** |  | 
**r#type** | **String** |  | 
**side** | **String** |  | 
**self_trade_prevention_mode** | **String** |  | 
**contingency_type** | **String** |  | 
**list_status_type** | **String** |  | 
**list_order_status** | **String** |  | 
**list_client_order_id** | **String** |  | 
**transaction_time** | **i64** |  | 
**orders** | [**Vec<models::SapiV1MarginOrderOcoPost200ResponseOrdersInner>**](_sapi_v1_margin_order_oco_post_200_response_orders_inner.md) |  | 
**order_reports** | [**Vec<models::OcoOrderOrderReportsInner>**](ocoOrder_orderReports_inner.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


