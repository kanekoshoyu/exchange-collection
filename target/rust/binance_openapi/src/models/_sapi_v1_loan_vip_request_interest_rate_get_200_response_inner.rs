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
pub struct SapiV1LoanVipRequestInterestRateGet200ResponseInner {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "flexibleDailyInterestRate")]
    pub flexible_daily_interest_rate: String,
    #[serde(rename = "flexibleYearlyInterestRate")]
    pub flexible_yearly_interest_rate: String,
    #[serde(rename = "time")]
    pub time: i64,
}

impl SapiV1LoanVipRequestInterestRateGet200ResponseInner {
    pub fn new(asset: String, flexible_daily_interest_rate: String, flexible_yearly_interest_rate: String, time: i64) -> SapiV1LoanVipRequestInterestRateGet200ResponseInner {
        SapiV1LoanVipRequestInterestRateGet200ResponseInner {
            asset,
            flexible_daily_interest_rate,
            flexible_yearly_interest_rate,
            time,
        }
    }
}
