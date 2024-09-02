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
pub struct SapiV1MarginDelistScheduleGet200ResponseInner {
    #[serde(rename = "delistTime", skip_serializing_if = "Option::is_none")]
    pub delist_time: Option<i64>,
    #[serde(rename = "crossMarginAssets", skip_serializing_if = "Option::is_none")]
    pub cross_margin_assets: Option<Vec<String>>,
    #[serde(rename = "isolatedMarginSymbols", skip_serializing_if = "Option::is_none")]
    pub isolated_margin_symbols: Option<Vec<String>>,
}

impl SapiV1MarginDelistScheduleGet200ResponseInner {
    pub fn new() -> SapiV1MarginDelistScheduleGet200ResponseInner {
        SapiV1MarginDelistScheduleGet200ResponseInner {
            delist_time: None,
            cross_margin_assets: None,
            isolated_margin_symbols: None,
        }
    }
}

