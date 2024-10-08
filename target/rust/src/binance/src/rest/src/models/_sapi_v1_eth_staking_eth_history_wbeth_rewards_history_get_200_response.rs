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
pub struct SapiV1EthStakingEthHistoryWbethRewardsHistoryGet200Response {
    #[serde(rename = "estRewardsInETH")]
    pub est_rewards_in_eth: String,
    #[serde(rename = "rows")]
    pub rows: Vec<models::SapiV1EthStakingEthHistoryWbethRewardsHistoryGet200ResponseRowsInner>,
    #[serde(rename = "total")]
    pub total: i64,
}

impl SapiV1EthStakingEthHistoryWbethRewardsHistoryGet200Response {
    pub fn new(est_rewards_in_eth: String, rows: Vec<models::SapiV1EthStakingEthHistoryWbethRewardsHistoryGet200ResponseRowsInner>, total: i64) -> SapiV1EthStakingEthHistoryWbethRewardsHistoryGet200Response {
        SapiV1EthStakingEthHistoryWbethRewardsHistoryGet200Response {
            est_rewards_in_eth,
            rows,
            total,
        }
    }
}

