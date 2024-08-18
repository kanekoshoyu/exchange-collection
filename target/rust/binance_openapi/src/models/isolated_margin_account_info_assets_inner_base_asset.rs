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
pub struct IsolatedMarginAccountInfoAssetsInnerBaseAsset {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "borrowEnabled")]
    pub borrow_enabled: bool,
    #[serde(rename = "borrowed")]
    pub borrowed: String,
    #[serde(rename = "free")]
    pub free: String,
    #[serde(rename = "interest")]
    pub interest: String,
    #[serde(rename = "locked")]
    pub locked: String,
    #[serde(rename = "netAsset")]
    pub net_asset: String,
    #[serde(rename = "netAssetOfBtc")]
    pub net_asset_of_btc: String,
    #[serde(rename = "repayEnabled")]
    pub repay_enabled: bool,
    #[serde(rename = "totalAsset")]
    pub total_asset: String,
}

impl IsolatedMarginAccountInfoAssetsInnerBaseAsset {
    pub fn new(asset: String, borrow_enabled: bool, borrowed: String, free: String, interest: String, locked: String, net_asset: String, net_asset_of_btc: String, repay_enabled: bool, total_asset: String) -> IsolatedMarginAccountInfoAssetsInnerBaseAsset {
        IsolatedMarginAccountInfoAssetsInnerBaseAsset {
            asset,
            borrow_enabled,
            borrowed,
            free,
            interest,
            locked,
            net_asset,
            net_asset_of_btc,
            repay_enabled,
            total_asset,
        }
    }
}
