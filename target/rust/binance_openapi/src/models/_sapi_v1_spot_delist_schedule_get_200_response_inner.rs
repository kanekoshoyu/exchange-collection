/*
 * Binance Public Spot API
 *
 * OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapiV1SpotDelistScheduleGet200ResponseInner {
    #[serde(rename = "delistTime")]
    pub delist_time: i64,
    #[serde(rename = "symbol")]
    pub symbol: Vec<String>,
}

impl SapiV1SpotDelistScheduleGet200ResponseInner {
    pub fn new(delist_time: i64, symbol: Vec<String>) -> SapiV1SpotDelistScheduleGet200ResponseInner {
        SapiV1SpotDelistScheduleGet200ResponseInner {
            delist_time,
            symbol,
        }
    }
}
