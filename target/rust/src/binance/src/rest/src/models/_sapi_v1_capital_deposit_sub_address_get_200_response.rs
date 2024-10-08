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
pub struct SapiV1CapitalDepositSubAddressGet200Response {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "coin")]
    pub coin: String,
    #[serde(rename = "tag")]
    pub tag: String,
    #[serde(rename = "url")]
    pub url: String,
}

impl SapiV1CapitalDepositSubAddressGet200Response {
    pub fn new(address: String, coin: String, tag: String, url: String) -> SapiV1CapitalDepositSubAddressGet200Response {
        SapiV1CapitalDepositSubAddressGet200Response {
            address,
            coin,
            tag,
            url,
        }
    }
}

