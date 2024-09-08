# SapiV1PayTransactionsGet200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_type** | **String** | Enum：PAY(C2B Merchant Acquiring Payment), PAY_REFUND(C2B Merchant Acquiring Payment,refund), C2C(C2C Transfer Payment),CRYPTO_BOX(Crypto box), CRYPTO_BOX_RF(Crypto Box, refund), C2C_HOLDING(Transfer to new Binance user), C2C_HOLDING_RF(Transfer to new Binance user,refund), PAYOUT(B2C Disbursement Payment) | 
**transaction_id** | **String** |  | 
**transaction_time** | **i64** |  | 
**amount** | **String** | order amount(up to 8 decimal places), positive is income, negative is expenditure | 
**currency** | **String** |  | 
**wallet_type** | **i32** |  | 
**wallet_types** | **Vec<i32>** |  | 
**funds_detail** | [**Vec<models::SapiV1PayTransactionsGet200ResponseDataInnerFundsDetailInner>**](_sapi_v1_pay_transactions_get_200_response_data_inner_fundsDetail_inner.md) |  | 
**payer_info** | [**models::SapiV1PayTransactionsGet200ResponseDataInnerPayerInfo**](_sapi_v1_pay_transactions_get_200_response_data_inner_payerInfo.md) |  | 
**receiver_info** | [**models::SapiV1PayTransactionsGet200ResponseDataInnerReceiverInfo**](_sapi_v1_pay_transactions_get_200_response_data_inner_receiverInfo.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


