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
pub struct SnapshotFuturesSnapshotVosInner {
    #[serde(rename = "data")]
    pub data: Box<models::SnapshotFuturesSnapshotVosInnerData>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "updateTime")]
    pub update_time: i64,
}

impl SnapshotFuturesSnapshotVosInner {
    pub fn new(data: models::SnapshotFuturesSnapshotVosInnerData, r#type: String, update_time: i64) -> SnapshotFuturesSnapshotVosInner {
        SnapshotFuturesSnapshotVosInner {
            data: Box::new(data),
            r#type,
            update_time,
        }
    }
}

