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
pub struct SapiV1MarginAllAssetsGet200ResponseInner {
    #[serde(rename = "assetFullName")]
    pub asset_full_name: String,
    #[serde(rename = "assetName")]
    pub asset_name: String,
    #[serde(rename = "isBorrowable")]
    pub is_borrowable: bool,
    #[serde(rename = "isMortgageable")]
    pub is_mortgageable: bool,
    #[serde(rename = "userMinBorrow")]
    pub user_min_borrow: String,
    #[serde(rename = "userMinRepay")]
    pub user_min_repay: String,
}

impl SapiV1MarginAllAssetsGet200ResponseInner {
    pub fn new(asset_full_name: String, asset_name: String, is_borrowable: bool, is_mortgageable: bool, user_min_borrow: String, user_min_repay: String) -> SapiV1MarginAllAssetsGet200ResponseInner {
        SapiV1MarginAllAssetsGet200ResponseInner {
            asset_full_name,
            asset_name,
            is_borrowable,
            is_mortgageable,
            user_min_borrow,
            user_min_repay,
        }
    }
}

