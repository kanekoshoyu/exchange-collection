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
pub struct SapiV1LoanCollateralDataGet200Response {
    #[serde(rename = "rows")]
    pub rows: Vec<models::SapiV1LoanCollateralDataGet200ResponseRowsInner>,
    #[serde(rename = "total")]
    pub total: i32,
}

impl SapiV1LoanCollateralDataGet200Response {
    pub fn new(rows: Vec<models::SapiV1LoanCollateralDataGet200ResponseRowsInner>, total: i32) -> SapiV1LoanCollateralDataGet200Response {
        SapiV1LoanCollateralDataGet200Response {
            rows,
            total,
        }
    }
}

