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
pub struct SapiV1SimpleEarnFlexibleSubscribePost200Response {
    #[serde(rename = "purchaseId")]
    pub purchase_id: i64,
    #[serde(rename = "success")]
    pub success: bool,
}

impl SapiV1SimpleEarnFlexibleSubscribePost200Response {
    pub fn new(purchase_id: i64, success: bool) -> SapiV1SimpleEarnFlexibleSubscribePost200Response {
        SapiV1SimpleEarnFlexibleSubscribePost200Response {
            purchase_id,
            success,
        }
    }
}
