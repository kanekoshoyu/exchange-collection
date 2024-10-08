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
pub struct SnapshotFuturesSnapshotVosInnerData {
    #[serde(rename = "assets")]
    pub assets: Vec<models::SnapshotFuturesSnapshotVosInnerDataAssetsInner>,
    #[serde(rename = "position")]
    pub position: Vec<models::SnapshotFuturesSnapshotVosInnerDataPositionInner>,
}

impl SnapshotFuturesSnapshotVosInnerData {
    pub fn new(assets: Vec<models::SnapshotFuturesSnapshotVosInnerDataAssetsInner>, position: Vec<models::SnapshotFuturesSnapshotVosInnerDataPositionInner>) -> SnapshotFuturesSnapshotVosInnerData {
        SnapshotFuturesSnapshotVosInnerData {
            assets,
            position,
        }
    }
}

