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
pub struct SapiV1SimpleEarnLockedHistoryRedemptionRecordGet200ResponseRowsInner {
    #[serde(rename = "positionId")]
    pub position_id: String,
    #[serde(rename = "redeemId")]
    pub redeem_id: i64,
    #[serde(rename = "time")]
    pub time: i64,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "lockPeriod")]
    pub lock_period: String,
    #[serde(rename = "amount")]
    pub amount: String,
    /// MATURE for redeem to Spot Wallet, NEW_TRANSFERRED for redeem to Flexible product, AHEAD for early redemption
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "deliverDate")]
    pub deliver_date: String,
    #[serde(rename = "status")]
    pub status: String,
}

impl SapiV1SimpleEarnLockedHistoryRedemptionRecordGet200ResponseRowsInner {
    pub fn new(position_id: String, redeem_id: i64, time: i64, asset: String, lock_period: String, amount: String, r#type: String, deliver_date: String, status: String) -> SapiV1SimpleEarnLockedHistoryRedemptionRecordGet200ResponseRowsInner {
        SapiV1SimpleEarnLockedHistoryRedemptionRecordGet200ResponseRowsInner {
            position_id,
            redeem_id,
            time,
            asset,
            lock_period,
            amount,
            r#type,
            deliver_date,
            status,
        }
    }
}

