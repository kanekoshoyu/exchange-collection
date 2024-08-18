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
pub struct SapiV1SubAccountFuturesEnablePost200Response {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "isFuturesEnabled")]
    pub is_futures_enabled: bool,
}

impl SapiV1SubAccountFuturesEnablePost200Response {
    pub fn new(email: String, is_futures_enabled: bool) -> SapiV1SubAccountFuturesEnablePost200Response {
        SapiV1SubAccountFuturesEnablePost200Response {
            email,
            is_futures_enabled,
        }
    }
}
