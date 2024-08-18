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
pub struct SapiV1AssetLedgerTransferCloudMiningQueryByPageGet200ResponseRowsInner {
    #[serde(rename = "createTime")]
    pub create_time: i64,
    #[serde(rename = "tranId")]
    pub tran_id: i64,
    #[serde(rename = "type")]
    pub r#type: i64,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "status")]
    pub status: String,
}

impl SapiV1AssetLedgerTransferCloudMiningQueryByPageGet200ResponseRowsInner {
    pub fn new(create_time: i64, tran_id: i64, r#type: i64, asset: String, amount: String, status: String) -> SapiV1AssetLedgerTransferCloudMiningQueryByPageGet200ResponseRowsInner {
        SapiV1AssetLedgerTransferCloudMiningQueryByPageGet200ResponseRowsInner {
            create_time,
            tran_id,
            r#type,
            asset,
            amount,
            status,
        }
    }
}
