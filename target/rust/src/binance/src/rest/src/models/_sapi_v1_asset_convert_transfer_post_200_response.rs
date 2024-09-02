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
pub struct SapiV1AssetConvertTransferPost200Response {
    #[serde(rename = "tranId")]
    pub tran_id: i64,
    #[serde(rename = "status")]
    pub status: String,
}

impl SapiV1AssetConvertTransferPost200Response {
    pub fn new(tran_id: i64, status: String) -> SapiV1AssetConvertTransferPost200Response {
        SapiV1AssetConvertTransferPost200Response {
            tran_id,
            status,
        }
    }
}

