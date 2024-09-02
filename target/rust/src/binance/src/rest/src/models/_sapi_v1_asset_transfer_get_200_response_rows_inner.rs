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
pub struct SapiV1AssetTransferGet200ResponseRowsInner {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "tranId")]
    pub tran_id: i64,
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
}

impl SapiV1AssetTransferGet200ResponseRowsInner {
    pub fn new(asset: String, amount: String, r#type: String, status: String, tran_id: i64, timestamp: i64) -> SapiV1AssetTransferGet200ResponseRowsInner {
        SapiV1AssetTransferGet200ResponseRowsInner {
            asset,
            amount,
            r#type,
            status,
            tran_id,
            timestamp,
        }
    }
}

