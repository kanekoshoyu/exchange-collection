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
pub struct ApiV3MyPreventedMatchesGet200ResponseInner {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "preventedMatchId")]
    pub prevented_match_id: i64,
    #[serde(rename = "takerOrderId")]
    pub taker_order_id: i64,
    #[serde(rename = "makerOrderId")]
    pub maker_order_id: i64,
    #[serde(rename = "tradeGroupId")]
    pub trade_group_id: i64,
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "makerPreventedQuantity")]
    pub maker_prevented_quantity: String,
    #[serde(rename = "transactTime")]
    pub transact_time: i64,
}

impl ApiV3MyPreventedMatchesGet200ResponseInner {
    pub fn new(symbol: String, prevented_match_id: i64, taker_order_id: i64, maker_order_id: i64, trade_group_id: i64, self_trade_prevention_mode: String, price: String, maker_prevented_quantity: String, transact_time: i64) -> ApiV3MyPreventedMatchesGet200ResponseInner {
        ApiV3MyPreventedMatchesGet200ResponseInner {
            symbol,
            prevented_match_id,
            taker_order_id,
            maker_order_id,
            trade_group_id,
            self_trade_prevention_mode,
            price,
            maker_prevented_quantity,
            transact_time,
        }
    }
}

