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
pub struct SapiV1LendingAutoInvestIndexUserSummaryGet200Response {
    #[serde(rename = "indexId")]
    pub index_id: i64,
    #[serde(rename = "totalInvestedInUSD")]
    pub total_invested_in_usd: String,
    /// current invest
    #[serde(rename = "currentInvestedInUSD")]
    pub current_invested_in_usd: String,
    /// PNL of the plan in USD based on current amount
    #[serde(rename = "pnlInUSD")]
    pub pnl_in_usd: String,
    /// ROI of the plan based on current amount
    #[serde(rename = "roi")]
    pub roi: String,
    #[serde(rename = "assetAllocation")]
    pub asset_allocation: Vec<models::SapiV1LendingAutoInvestIndexUserSummaryGet200ResponseAssetAllocationInner>,
    #[serde(rename = "details")]
    pub details: Vec<models::SapiV1LendingAutoInvestIndexUserSummaryGet200ResponseDetailsInner>,
}

impl SapiV1LendingAutoInvestIndexUserSummaryGet200Response {
    pub fn new(index_id: i64, total_invested_in_usd: String, current_invested_in_usd: String, pnl_in_usd: String, roi: String, asset_allocation: Vec<models::SapiV1LendingAutoInvestIndexUserSummaryGet200ResponseAssetAllocationInner>, details: Vec<models::SapiV1LendingAutoInvestIndexUserSummaryGet200ResponseDetailsInner>) -> SapiV1LendingAutoInvestIndexUserSummaryGet200Response {
        SapiV1LendingAutoInvestIndexUserSummaryGet200Response {
            index_id,
            total_invested_in_usd,
            current_invested_in_usd,
            pnl_in_usd,
            roi,
            asset_allocation,
            details,
        }
    }
}

