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
pub struct SapiV1LendingAutoInvestRedeemHistoryGet200ResponseInner {
    #[serde(rename = "indexId")]
    pub index_id: i64,
    #[serde(rename = "indexName")]
    pub index_name: String,
    #[serde(rename = "redemptionId")]
    pub redemption_id: i64,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "redemptionDateTime")]
    pub redemption_date_time: i64,
    #[serde(rename = "transactionFee")]
    pub transaction_fee: String,
    #[serde(rename = "transactionFeeUnit")]
    pub transaction_fee_unit: String,
}

impl SapiV1LendingAutoInvestRedeemHistoryGet200ResponseInner {
    pub fn new(index_id: i64, index_name: String, redemption_id: i64, status: String, asset: String, amount: String, redemption_date_time: i64, transaction_fee: String, transaction_fee_unit: String) -> SapiV1LendingAutoInvestRedeemHistoryGet200ResponseInner {
        SapiV1LendingAutoInvestRedeemHistoryGet200ResponseInner {
            index_id,
            index_name,
            redemption_id,
            status,
            asset,
            amount,
            redemption_date_time,
            transaction_fee,
            transaction_fee_unit,
        }
    }
}
