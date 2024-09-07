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
pub struct SapiV1EthStakingEthHistoryStakingHistoryGet200ResponseRowsInner {
    #[serde(rename = "time")]
    pub time: i64,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "amount")]
    pub amount: String,
    /// PENDING, SUCCESS, FAILED
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "distributeAmount")]
    pub distribute_amount: String,
    #[serde(rename = "conversionRatio")]
    pub conversion_ratio: String,
}

impl SapiV1EthStakingEthHistoryStakingHistoryGet200ResponseRowsInner {
    pub fn new(time: i64, asset: String, amount: String, status: String, distribute_amount: String, conversion_ratio: String) -> SapiV1EthStakingEthHistoryStakingHistoryGet200ResponseRowsInner {
        SapiV1EthStakingEthHistoryStakingHistoryGet200ResponseRowsInner {
            time,
            asset,
            amount,
            status,
            distribute_amount,
            conversion_ratio,
        }
    }
}
