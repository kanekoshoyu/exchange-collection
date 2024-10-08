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
pub struct SapiV1PortfolioInterestHistoryGet200ResponseInner {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "interest")]
    pub interest: String,
    #[serde(rename = "interestAccruedTime")]
    pub interest_accrued_time: i64,
    #[serde(rename = "interestRate")]
    pub interest_rate: String,
    #[serde(rename = "principal")]
    pub principal: String,
}

impl SapiV1PortfolioInterestHistoryGet200ResponseInner {
    pub fn new(asset: String, interest: String, interest_accrued_time: i64, interest_rate: String, principal: String) -> SapiV1PortfolioInterestHistoryGet200ResponseInner {
        SapiV1PortfolioInterestHistoryGet200ResponseInner {
            asset,
            interest,
            interest_accrued_time,
            interest_rate,
            principal,
        }
    }
}

