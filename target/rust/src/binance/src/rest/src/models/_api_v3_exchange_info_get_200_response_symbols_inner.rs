/*
 * Binance Public Spot API
 *
 * OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiV3ExchangeInfoGet200ResponseSymbolsInner {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "baseAsset")]
    pub base_asset: String,
    #[serde(rename = "baseAssetPrecision")]
    pub base_asset_precision: i32,
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,
    #[serde(rename = "quoteAssetPrecision")]
    pub quote_asset_precision: i32,
    #[serde(rename = "baseCommissionPrecision")]
    pub base_commission_precision: i32,
    #[serde(rename = "quoteCommissionPrecision")]
    pub quote_commission_precision: i32,
    #[serde(rename = "orderTypes")]
    pub order_types: Vec<String>,
    #[serde(rename = "icebergAllowed")]
    pub iceberg_allowed: bool,
    #[serde(rename = "ocoAllowed")]
    pub oco_allowed: bool,
    #[serde(rename = "otoAllowed")]
    pub oto_allowed: bool,
    #[serde(rename = "quoteOrderQtyMarketAllowed")]
    pub quote_order_qty_market_allowed: bool,
    #[serde(rename = "allowTrailingStop")]
    pub allow_trailing_stop: bool,
    #[serde(rename = "cancelReplaceAllowed")]
    pub cancel_replace_allowed: bool,
    #[serde(rename = "isSpotTradingAllowed")]
    pub is_spot_trading_allowed: bool,
    #[serde(rename = "isMarginTradingAllowed")]
    pub is_margin_trading_allowed: bool,
    #[serde(rename = "filters")]
    pub filters: Vec<models::ApiV3ExchangeInfoGet200ResponseSymbolsInnerFiltersInner>,
    #[serde(rename = "permissions")]
    pub permissions: Vec<String>,
    #[serde(rename = "permissionSets")]
    pub permission_sets: Vec<Vec<String>>,
    #[serde(rename = "defaultSelfTradePreventionMode")]
    pub default_self_trade_prevention_mode: String,
    #[serde(rename = "allowedSelfTradePreventionModes")]
    pub allowed_self_trade_prevention_modes: Vec<String>,
}

impl ApiV3ExchangeInfoGet200ResponseSymbolsInner {
    pub fn new(symbol: String, status: String, base_asset: String, base_asset_precision: i32, quote_asset: String, quote_asset_precision: i32, base_commission_precision: i32, quote_commission_precision: i32, order_types: Vec<String>, iceberg_allowed: bool, oco_allowed: bool, oto_allowed: bool, quote_order_qty_market_allowed: bool, allow_trailing_stop: bool, cancel_replace_allowed: bool, is_spot_trading_allowed: bool, is_margin_trading_allowed: bool, filters: Vec<models::ApiV3ExchangeInfoGet200ResponseSymbolsInnerFiltersInner>, permissions: Vec<String>, permission_sets: Vec<Vec<String>>, default_self_trade_prevention_mode: String, allowed_self_trade_prevention_modes: Vec<String>) -> ApiV3ExchangeInfoGet200ResponseSymbolsInner {
        ApiV3ExchangeInfoGet200ResponseSymbolsInner {
            symbol,
            status,
            base_asset,
            base_asset_precision,
            quote_asset,
            quote_asset_precision,
            base_commission_precision,
            quote_commission_precision,
            order_types,
            iceberg_allowed,
            oco_allowed,
            oto_allowed,
            quote_order_qty_market_allowed,
            allow_trailing_stop,
            cancel_replace_allowed,
            is_spot_trading_allowed,
            is_margin_trading_allowed,
            filters,
            permissions,
            permission_sets,
            default_self_trade_prevention_mode,
            allowed_self_trade_prevention_modes,
        }
    }
}

