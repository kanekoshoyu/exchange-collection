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
pub struct SapiV1CapitalDepositAddressListGet200ResponseInner {
    #[serde(rename = "coin")]
    pub coin: String,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "isDefault")]
    pub is_default: i32,
}

impl SapiV1CapitalDepositAddressListGet200ResponseInner {
    pub fn new(coin: String, address: String, is_default: i32) -> SapiV1CapitalDepositAddressListGet200ResponseInner {
        SapiV1CapitalDepositAddressListGet200ResponseInner {
            coin,
            address,
            is_default,
        }
    }
}

