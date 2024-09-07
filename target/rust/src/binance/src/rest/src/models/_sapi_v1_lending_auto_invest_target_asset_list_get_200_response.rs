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
pub struct SapiV1LendingAutoInvestTargetAssetListGet200Response {
    #[serde(rename = "targetAssets", skip_serializing_if = "Option::is_none")]
    pub target_assets: Option<String>,
    #[serde(rename = "autoInvestAssetList", skip_serializing_if = "Option::is_none")]
    pub auto_invest_asset_list: Option<Vec<models::SapiV1LendingAutoInvestTargetAssetListGet200ResponseAutoInvestAssetListInner>>,
}

impl SapiV1LendingAutoInvestTargetAssetListGet200Response {
    pub fn new() -> SapiV1LendingAutoInvestTargetAssetListGet200Response {
        SapiV1LendingAutoInvestTargetAssetListGet200Response {
            target_assets: None,
            auto_invest_asset_list: None,
        }
    }
}
