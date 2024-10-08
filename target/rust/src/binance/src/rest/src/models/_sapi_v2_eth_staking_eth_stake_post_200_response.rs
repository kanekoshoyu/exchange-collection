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
pub struct SapiV2EthStakingEthStakePost200Response {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "wbethAmount")]
    pub wbeth_amount: String,
    /// ETH amount per 1 WBETH
    #[serde(rename = "conversionRatio")]
    pub conversion_ratio: String,
}

impl SapiV2EthStakingEthStakePost200Response {
    pub fn new(success: bool, wbeth_amount: String, conversion_ratio: String) -> SapiV2EthStakingEthStakePost200Response {
        SapiV2EthStakingEthStakePost200Response {
            success,
            wbeth_amount,
            conversion_ratio,
        }
    }
}

