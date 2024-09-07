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
pub struct SapiV1RebateTaxQueryGet200Response {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "data")]
    pub data: Box<models::SapiV1RebateTaxQueryGet200ResponseData>,
}

impl SapiV1RebateTaxQueryGet200Response {
    pub fn new(status: String, r#type: String, code: String, data: models::SapiV1RebateTaxQueryGet200ResponseData) -> SapiV1RebateTaxQueryGet200Response {
        SapiV1RebateTaxQueryGet200Response {
            status,
            r#type,
            code,
            data: Box::new(data),
        }
    }
}
