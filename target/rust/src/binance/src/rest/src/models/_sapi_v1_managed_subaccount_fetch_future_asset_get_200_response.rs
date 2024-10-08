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
pub struct SapiV1ManagedSubaccountFetchFutureAssetGet200Response {
    #[serde(rename = "code")]
    pub code: i32,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "snapshotVos")]
    pub snapshot_vos: Vec<models::SapiV1ManagedSubaccountFetchFutureAssetGet200ResponseSnapshotVosInner>,
}

impl SapiV1ManagedSubaccountFetchFutureAssetGet200Response {
    pub fn new(code: i32, message: String, snapshot_vos: Vec<models::SapiV1ManagedSubaccountFetchFutureAssetGet200ResponseSnapshotVosInner>) -> SapiV1ManagedSubaccountFetchFutureAssetGet200Response {
        SapiV1ManagedSubaccountFetchFutureAssetGet200Response {
            code,
            message,
            snapshot_vos,
        }
    }
}

