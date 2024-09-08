# ApiV3ExchangeInfoGet200ResponseSymbolsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **String** |  | 
**status** | **String** |  | 
**base_asset** | **String** |  | 
**base_asset_precision** | **i32** |  | 
**quote_asset** | **String** |  | 
**quote_asset_precision** | **i32** |  | 
**base_commission_precision** | **i32** |  | 
**quote_commission_precision** | **i32** |  | 
**order_types** | **Vec<String>** |  | 
**iceberg_allowed** | **bool** |  | 
**oco_allowed** | **bool** |  | 
**oto_allowed** | **bool** |  | 
**quote_order_qty_market_allowed** | **bool** |  | 
**allow_trailing_stop** | **bool** |  | 
**cancel_replace_allowed** | **bool** |  | 
**is_spot_trading_allowed** | **bool** |  | 
**is_margin_trading_allowed** | **bool** |  | 
**filters** | [**Vec<models::ApiV3ExchangeInfoGet200ResponseSymbolsInnerFiltersInner>**](_api_v3_exchangeInfo_get_200_response_symbols_inner_filters_inner.md) |  | 
**permissions** | **Vec<String>** |  | 
**permission_sets** | [**Vec<Vec<String>>**](Vec.md) |  | 
**default_self_trade_prevention_mode** | **String** |  | 
**allowed_self_trade_prevention_modes** | **Vec<String>** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


