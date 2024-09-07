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
pub struct SapiV1SimpleEarnFlexibleHistoryRewardsRecordGet200ResponseRowsInner {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "rewards")]
    pub rewards: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "time")]
    pub time: i64,
}

impl SapiV1SimpleEarnFlexibleHistoryRewardsRecordGet200ResponseRowsInner {
    pub fn new(asset: String, rewards: String, project_id: String, r#type: String, time: i64) -> SapiV1SimpleEarnFlexibleHistoryRewardsRecordGet200ResponseRowsInner {
        SapiV1SimpleEarnFlexibleHistoryRewardsRecordGet200ResponseRowsInner {
            asset,
            rewards,
            project_id,
            r#type,
            time,
        }
    }
}
