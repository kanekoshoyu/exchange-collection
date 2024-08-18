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
pub struct SapiV2LoanFlexibleLoanableDataGet200ResponseRowsInner {
    #[serde(rename = "loanCoin")]
    pub loan_coin: String,
    #[serde(rename = "flexibleInterestRate")]
    pub flexible_interest_rate: String,
    #[serde(rename = "flexibleMinLimit")]
    pub flexible_min_limit: String,
    #[serde(rename = "flexibleMaxLimit")]
    pub flexible_max_limit: String,
}

impl SapiV2LoanFlexibleLoanableDataGet200ResponseRowsInner {
    pub fn new(loan_coin: String, flexible_interest_rate: String, flexible_min_limit: String, flexible_max_limit: String) -> SapiV2LoanFlexibleLoanableDataGet200ResponseRowsInner {
        SapiV2LoanFlexibleLoanableDataGet200ResponseRowsInner {
            loan_coin,
            flexible_interest_rate,
            flexible_min_limit,
            flexible_max_limit,
        }
    }
}
