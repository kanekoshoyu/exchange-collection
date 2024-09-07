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
pub struct SapiV1PayTransactionsGet200Response {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "data")]
    pub data: Vec<models::SapiV1PayTransactionsGet200ResponseDataInner>,
    #[serde(rename = "success")]
    pub success: bool,
}

impl SapiV1PayTransactionsGet200Response {
    pub fn new(code: String, message: String, data: Vec<models::SapiV1PayTransactionsGet200ResponseDataInner>, success: bool) -> SapiV1PayTransactionsGet200Response {
        SapiV1PayTransactionsGet200Response {
            code,
            message,
            data,
            success,
        }
    }
}
