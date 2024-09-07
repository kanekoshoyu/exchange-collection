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
pub struct SapiV1MarginAccountGet200ResponseUserAssetsInner {
    #[serde(rename = "asset")]
    pub asset: String,
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
}

impl SapiV1MarginAccountGet200ResponseUserAssetsInner {
    pub fn new(asset: String, borrowed: String, free: String, interest: String, locked: String, net_asset: String) -> SapiV1MarginAccountGet200ResponseUserAssetsInner {
        SapiV1MarginAccountGet200ResponseUserAssetsInner {
            asset,
            borrowed,
            free,
            interest,
            locked,
            net_asset,
        }
    }
}
