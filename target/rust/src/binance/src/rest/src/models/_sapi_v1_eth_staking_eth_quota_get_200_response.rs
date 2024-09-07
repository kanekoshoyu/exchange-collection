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
pub struct SapiV1EthStakingEthQuotaGet200Response {
    /// Show min(Daily available limit, total personal staking quota)
    #[serde(rename = "leftStakingPersonalQuota")]
    pub left_staking_personal_quota: String,
    /// Show min(Daily personal redeem quota, total redemption limit)
    #[serde(rename = "leftRedemptionPersonalQuota")]
    pub left_redemption_personal_quota: String,
}

impl SapiV1EthStakingEthQuotaGet200Response {
    pub fn new(left_staking_personal_quota: String, left_redemption_personal_quota: String) -> SapiV1EthStakingEthQuotaGet200Response {
        SapiV1EthStakingEthQuotaGet200Response {
            left_staking_personal_quota,
            left_redemption_personal_quota,
        }
    }
}
