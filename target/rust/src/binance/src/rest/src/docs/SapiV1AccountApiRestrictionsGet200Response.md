# SapiV1AccountApiRestrictionsGet200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ip_restrict** | **bool** |  | 
**create_time** | **i64** |  | 
**enable_internal_transfer** | **bool** | This option authorizes this key to transfer funds between your master account and your sub account instantly | 
**enable_futures** | **bool** | API Key created before your futures account opened does not support futures API service | 
**enable_portfolio_margin_trading** | Option<**bool**> | API Key created before your activate portfolio margin does not support portfolio margin API service | [optional]
**enable_vanilla_options** | **bool** | Authorizes this key to Vanilla options trading | 
**permits_universal_transfer** | **bool** | Authorizes this key to be used for a dedicated universal transfer API to transfer multiple supported currencies. Each business's own transfer API rights are not affected by this authorization | 
**enable_reading** | **bool** |  | 
**enable_spot_and_margin_trading** | **bool** |  | 
**enable_withdrawals** | **bool** | This option allows you to withdraw via API. You must apply the IP Access Restriction filter in order to enable withdrawals | 
**enable_margin** | **bool** | This option can be adjusted after the Cross Margin account transfer is completed | 
**trading_authority_expiration_time** | **i64** | Expiration time for spot and margin trading permission | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


