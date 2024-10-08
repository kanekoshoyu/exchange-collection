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
pub struct SapiV1PortfolioMarginAssetLeverageGet200ResponseInner {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(rename = "collateralRate", skip_serializing_if = "Option::is_none")]
    pub collateral_rate: Option<String>,
}

impl SapiV1PortfolioMarginAssetLeverageGet200ResponseInner {
    pub fn new() -> SapiV1PortfolioMarginAssetLeverageGet200ResponseInner {
        SapiV1PortfolioMarginAssetLeverageGet200ResponseInner {
            asset: None,
            collateral_rate: None,
        }
    }
}

