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
pub struct SubAccountCoinFuturesSummary {
    #[serde(rename = "deliveryAccountSummaryResp")]
    pub delivery_account_summary_resp: Box<models::SubAccountCoinFuturesSummaryDeliveryAccountSummaryResp>,
}

impl SubAccountCoinFuturesSummary {
    pub fn new(delivery_account_summary_resp: models::SubAccountCoinFuturesSummaryDeliveryAccountSummaryResp) -> SubAccountCoinFuturesSummary {
        SubAccountCoinFuturesSummary {
            delivery_account_summary_resp: Box::new(delivery_account_summary_resp),
        }
    }
}

