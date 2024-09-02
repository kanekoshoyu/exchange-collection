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
pub struct SapiV1CapitalContractConvertibleCoinsGet200ResponseExchangeRates {
    #[serde(rename = "USDC")]
    pub usdc: String,
    #[serde(rename = "TUSD")]
    pub tusd: String,
    #[serde(rename = "USDP")]
    pub usdp: String,
}

impl SapiV1CapitalContractConvertibleCoinsGet200ResponseExchangeRates {
    pub fn new(usdc: String, tusd: String, usdp: String) -> SapiV1CapitalContractConvertibleCoinsGet200ResponseExchangeRates {
        SapiV1CapitalContractConvertibleCoinsGet200ResponseExchangeRates {
            usdc,
            tusd,
            usdp,
        }
    }
}

