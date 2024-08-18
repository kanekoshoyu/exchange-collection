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
pub struct SapiV1GiftcardCreateCodePost200ResponseData {
    #[serde(rename = "referenceNo")]
    pub reference_no: String,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "expiredTime")]
    pub expired_time: i64,
}

impl SapiV1GiftcardCreateCodePost200ResponseData {
    pub fn new(reference_no: String, code: String, expired_time: i64) -> SapiV1GiftcardCreateCodePost200ResponseData {
        SapiV1GiftcardCreateCodePost200ResponseData {
            reference_no,
            code,
            expired_time,
        }
    }
}
