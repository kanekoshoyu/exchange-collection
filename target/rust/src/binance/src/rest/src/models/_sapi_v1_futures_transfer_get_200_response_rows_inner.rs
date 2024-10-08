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
pub struct SapiV1FuturesTransferGet200ResponseRowsInner {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "tranId")]
    pub tran_id: i64,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    /// one of PENDING (pending to execution), CONFIRMED (successfully transfered), FAILED (execution failed, nothing happened to your account);
    #[serde(rename = "status")]
    pub status: String,
}

impl SapiV1FuturesTransferGet200ResponseRowsInner {
    pub fn new(asset: String, tran_id: i64, amount: String, r#type: String, timestamp: i64, status: String) -> SapiV1FuturesTransferGet200ResponseRowsInner {
        SapiV1FuturesTransferGet200ResponseRowsInner {
            asset,
            tran_id,
            amount,
            r#type,
            timestamp,
            status,
        }
    }
}

