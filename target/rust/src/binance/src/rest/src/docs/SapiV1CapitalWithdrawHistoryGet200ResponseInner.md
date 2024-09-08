# SapiV1CapitalWithdrawHistoryGet200ResponseInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | **String** |  | 
**amount** | **String** |  | 
**apply_time** | **String** |  | 
**coin** | **String** |  | 
**id** | **String** |  | 
**withdraw_order_id** | **String** | will not be returned if there's no withdrawOrderId for this withdraw. | 
**network** | **String** |  | 
**transfer_type** | **i32** | 1 for internal transfer, 0 for external transfer | 
**status** | **i32** |  | 
**transaction_fee** | **String** |  | 
**confirm_no** | Option<**i32**> |  | [optional]
**info** | Option<**String**> | Reason for withdrawal failure | [optional]
**tx_id** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


