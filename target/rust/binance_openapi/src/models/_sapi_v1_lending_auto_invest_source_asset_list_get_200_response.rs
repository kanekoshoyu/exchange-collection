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
pub struct SapiV1LendingAutoInvestSourceAssetListGet200Response {
    #[serde(rename = "feeRate")]
    pub fee_rate: String,
    #[serde(rename = "sourceAssets")]
    pub source_assets: Vec<models::SapiV1LendingAutoInvestSourceAssetListGet200ResponseSourceAssetsInner>,
}

impl SapiV1LendingAutoInvestSourceAssetListGet200Response {
    pub fn new(fee_rate: String, source_assets: Vec<models::SapiV1LendingAutoInvestSourceAssetListGet200ResponseSourceAssetsInner>) -> SapiV1LendingAutoInvestSourceAssetListGet200Response {
        SapiV1LendingAutoInvestSourceAssetListGet200Response {
            fee_rate,
            source_assets,
        }
    }
}
