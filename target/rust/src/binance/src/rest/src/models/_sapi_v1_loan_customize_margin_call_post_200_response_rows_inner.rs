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
pub struct SapiV1LoanCustomizeMarginCallPost200ResponseRowsInner {
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "collateralCoin")]
    pub collateral_coin: String,
    #[serde(rename = "preMarginCall")]
    pub pre_margin_call: String,
    #[serde(rename = "afterMarginCall")]
    pub after_margin_call: String,
    #[serde(rename = "customizeTime")]
    pub customize_time: i64,
}

impl SapiV1LoanCustomizeMarginCallPost200ResponseRowsInner {
    pub fn new(order_id: String, collateral_coin: String, pre_margin_call: String, after_margin_call: String, customize_time: i64) -> SapiV1LoanCustomizeMarginCallPost200ResponseRowsInner {
        SapiV1LoanCustomizeMarginCallPost200ResponseRowsInner {
            order_id,
            collateral_coin,
            pre_margin_call,
            after_margin_call,
            customize_time,
        }
    }
}
