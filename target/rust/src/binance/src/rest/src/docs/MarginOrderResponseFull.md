# MarginOrderResponseFull

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **String** |  | 
**order_id** | **i64** |  | 
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
**margin_buy_borrow_amount** | **f64** | will not return if no margin trade happens | 
**margin_buy_borrow_asset** | **String** | will not return if no margin trade happens | 
**is_isolated** | **bool** |  | 
**fills** | [**Vec<models::OrderResponseFullFillsInner>**](orderResponseFull_fills_inner.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


