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
pub struct SapiV1EthStakingEthHistoryRateHistoryGet200ResponseRowsInner {
    /// BETH APR
    #[serde(rename = "annualPercentageRate")]
    pub annual_percentage_rate: String,
    /// BETH value per 1 WBETH
    #[serde(rename = "exchangeRate")]
    pub exchange_rate: String,
    #[serde(rename = "time")]
    pub time: i64,
}

impl SapiV1EthStakingEthHistoryRateHistoryGet200ResponseRowsInner {
    pub fn new(annual_percentage_rate: String, exchange_rate: String, time: i64) -> SapiV1EthStakingEthHistoryRateHistoryGet200ResponseRowsInner {
        SapiV1EthStakingEthHistoryRateHistoryGet200ResponseRowsInner {
            annual_percentage_rate,
            exchange_rate,
            time,
        }
    }
}
