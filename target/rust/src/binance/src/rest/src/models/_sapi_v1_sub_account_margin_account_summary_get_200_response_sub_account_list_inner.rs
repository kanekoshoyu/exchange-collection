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
pub struct SapiV1SubAccountMarginAccountSummaryGet200ResponseSubAccountListInner {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "totalAssetOfBtc")]
    pub total_asset_of_btc: String,
    #[serde(rename = "totalLiabilityOfBtc")]
    pub total_liability_of_btc: String,
    #[serde(rename = "totalNetAssetOfBtc")]
    pub total_net_asset_of_btc: String,
}

impl SapiV1SubAccountMarginAccountSummaryGet200ResponseSubAccountListInner {
    pub fn new(email: String, total_asset_of_btc: String, total_liability_of_btc: String, total_net_asset_of_btc: String) -> SapiV1SubAccountMarginAccountSummaryGet200ResponseSubAccountListInner {
        SapiV1SubAccountMarginAccountSummaryGet200ResponseSubAccountListInner {
            email,
            total_asset_of_btc,
            total_liability_of_btc,
            total_net_asset_of_btc,
        }
    }
}

