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
pub struct SapiV1MarginForceLiquidationRecGet200ResponseRowsInner {
    #[serde(rename = "avgPrice")]
    pub avg_price: String,
    #[serde(rename = "executedQty")]
    pub executed_qty: String,
    #[serde(rename = "orderId")]
    pub order_id: i64,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "qty")]
    pub qty: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    #[serde(rename = "isIsolated")]
    pub is_isolated: bool,
    #[serde(rename = "updatedTime")]
    pub updated_time: i64,
}

impl SapiV1MarginForceLiquidationRecGet200ResponseRowsInner {
    pub fn new(avg_price: String, executed_qty: String, order_id: i64, price: String, qty: String, side: String, symbol: String, time_in_force: String, is_isolated: bool, updated_time: i64) -> SapiV1MarginForceLiquidationRecGet200ResponseRowsInner {
        SapiV1MarginForceLiquidationRecGet200ResponseRowsInner {
            avg_price,
            executed_qty,
            order_id,
            price,
            qty,
            side,
            symbol,
            time_in_force,
            is_isolated,
            updated_time,
        }
    }
}

