# SapiV1SimpleEarnLockedHistorySubscriptionRecordGet200ResponseRowsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**position_id** | **String** |  | 
**purchase_id** | **i64** |  | 
**time** | **i64** |  | 
**asset** | **String** |  | 
**amount** | **String** |  | 
**lock_period** | **String** |  | 
**r#type** | **String** | NORMAL for normal subscription, AUTO for auto-subscription order, ACTIVITY for activity order, TRIAL for trial fund order, RESTAKE for restake order | 
**source_account** | **String** | SPOT, FUNDING, SPOTANDFUNDING | 
**amt_from_spot** | **String** | Display if sourceAccount is SPOTANDFUNDING  | 
**amt_from_funding** | **String** | Display if sourceAccount is SPOTANDFUNDING | 
**status** | **String** | PURCHASING/SUCCESS/FAILED | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


