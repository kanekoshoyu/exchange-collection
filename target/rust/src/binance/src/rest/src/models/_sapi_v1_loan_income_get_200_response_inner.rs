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
pub struct SapiV1LoanIncomeGet200ResponseInner {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "tranId")]
    pub tran_id: String,
}

impl SapiV1LoanIncomeGet200ResponseInner {
    pub fn new(asset: String, r#type: String, amount: String, timestamp: i64, tran_id: String) -> SapiV1LoanIncomeGet200ResponseInner {
        SapiV1LoanIncomeGet200ResponseInner {
            asset,
            r#type,
            amount,
            timestamp,
            tran_id,
        }
    }
}

