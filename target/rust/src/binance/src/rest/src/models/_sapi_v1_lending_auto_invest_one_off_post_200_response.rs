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
pub struct SapiV1LendingAutoInvestOneOffPost200Response {
    #[serde(rename = "transactionId")]
    pub transaction_id: i64,
    #[serde(rename = "waitSecond")]
    pub wait_second: i32,
}

impl SapiV1LendingAutoInvestOneOffPost200Response {
    pub fn new(transaction_id: i64, wait_second: i32) -> SapiV1LendingAutoInvestOneOffPost200Response {
        SapiV1LendingAutoInvestOneOffPost200Response {
            transaction_id,
            wait_second,
        }
    }
}
