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
pub struct SapiV1MiningStatisticsUserListGet200ResponseDataInnerListInner {
    #[serde(rename = "time")]
    pub time: i64,
    #[serde(rename = "hashrate")]
    pub hashrate: String,
    #[serde(rename = "reject")]
    pub reject: String,
}

impl SapiV1MiningStatisticsUserListGet200ResponseDataInnerListInner {
    pub fn new(time: i64, hashrate: String, reject: String) -> SapiV1MiningStatisticsUserListGet200ResponseDataInnerListInner {
        SapiV1MiningStatisticsUserListGet200ResponseDataInnerListInner {
            time,
            hashrate,
            reject,
        }
    }
}

