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
pub struct SapiV1LoanVipCollateralDataGet200ResponseRowsInner {
    #[serde(rename = "collateralCoin")]
    pub collateral_coin: String,
    #[serde(rename = "_1stCollateralRatio")]
    pub _1st_collateral_ratio: String,
    #[serde(rename = "_1stCollateralRange")]
    pub _1st_collateral_range: String,
    #[serde(rename = "_2ndCollateralRatio")]
    pub _2nd_collateral_ratio: String,
    #[serde(rename = "_2ndCollateralRange")]
    pub _2nd_collateral_range: String,
    #[serde(rename = "_3rdCollateralRatio")]
    pub _3rd_collateral_ratio: String,
    #[serde(rename = "_3rdCollateralRange")]
    pub _3rd_collateral_range: String,
    #[serde(rename = "_4thCollateralRatio")]
    pub _4th_collateral_ratio: String,
    #[serde(rename = "_4thCollateralRange")]
    pub _4th_collateral_range: String,
}

impl SapiV1LoanVipCollateralDataGet200ResponseRowsInner {
    pub fn new(collateral_coin: String, _1st_collateral_ratio: String, _1st_collateral_range: String, _2nd_collateral_ratio: String, _2nd_collateral_range: String, _3rd_collateral_ratio: String, _3rd_collateral_range: String, _4th_collateral_ratio: String, _4th_collateral_range: String) -> SapiV1LoanVipCollateralDataGet200ResponseRowsInner {
        SapiV1LoanVipCollateralDataGet200ResponseRowsInner {
            collateral_coin,
            _1st_collateral_ratio,
            _1st_collateral_range,
            _2nd_collateral_ratio,
            _2nd_collateral_range,
            _3rd_collateral_ratio,
            _3rd_collateral_range,
            _4th_collateral_ratio,
            _4th_collateral_range,
        }
    }
}
