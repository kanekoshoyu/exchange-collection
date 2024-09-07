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
pub struct IsolatedMarginAccountInfoAssetsInner {
    #[serde(rename = "baseAsset")]
    pub base_asset: Box<models::IsolatedMarginAccountInfoAssetsInnerBaseAsset>,
    #[serde(rename = "quoteAsset")]
    pub quote_asset: Box<models::IsolatedMarginAccountInfoAssetsInnerQuoteAsset>,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "isolatedCreated")]
    pub isolated_created: bool,
    /// true-enabled, false-disabled
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "marginLevel")]
    pub margin_level: String,
    /// \"EXCESSIVE\", \"NORMAL\", \"MARGIN_CALL\", \"PRE_LIQUIDATION\", \"FORCE_LIQUIDATION\"
    #[serde(rename = "marginLevelStatus")]
    pub margin_level_status: String,
    #[serde(rename = "marginRatio")]
    pub margin_ratio: String,
    #[serde(rename = "indexPrice")]
    pub index_price: String,
    #[serde(rename = "liquidatePrice")]
    pub liquidate_price: String,
    #[serde(rename = "liquidateRate")]
    pub liquidate_rate: String,
    #[serde(rename = "tradeEnabled")]
    pub trade_enabled: bool,
}

impl IsolatedMarginAccountInfoAssetsInner {
    pub fn new(base_asset: models::IsolatedMarginAccountInfoAssetsInnerBaseAsset, quote_asset: models::IsolatedMarginAccountInfoAssetsInnerQuoteAsset, symbol: String, isolated_created: bool, enabled: bool, margin_level: String, margin_level_status: String, margin_ratio: String, index_price: String, liquidate_price: String, liquidate_rate: String, trade_enabled: bool) -> IsolatedMarginAccountInfoAssetsInner {
        IsolatedMarginAccountInfoAssetsInner {
            base_asset: Box::new(base_asset),
            quote_asset: Box::new(quote_asset),
            symbol,
            isolated_created,
            enabled,
            margin_level,
            margin_level_status,
            margin_ratio,
            index_price,
            liquidate_price,
            liquidate_rate,
            trade_enabled,
        }
    }
}
