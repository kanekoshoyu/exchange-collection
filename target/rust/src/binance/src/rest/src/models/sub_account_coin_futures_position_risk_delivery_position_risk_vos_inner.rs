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
pub struct SubAccountCoinFuturesPositionRiskDeliveryPositionRiskVosInner {
    #[serde(rename = "entryPrice")]
    pub entry_price: String,
    #[serde(rename = "markPrice")]
    pub mark_price: String,
    #[serde(rename = "leverage")]
    pub leverage: String,
    #[serde(rename = "isolated")]
    pub isolated: String,
    #[serde(rename = "isolatedWallet")]
    pub isolated_wallet: String,
    #[serde(rename = "isolatedMargin")]
    pub isolated_margin: String,
    #[serde(rename = "isAutoAddMargin")]
    pub is_auto_add_margin: String,
    #[serde(rename = "positionSide")]
    pub position_side: String,
    #[serde(rename = "positionAmount")]
    pub position_amount: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "unrealizedProfit")]
    pub unrealized_profit: String,
}

impl SubAccountCoinFuturesPositionRiskDeliveryPositionRiskVosInner {
    pub fn new(entry_price: String, mark_price: String, leverage: String, isolated: String, isolated_wallet: String, isolated_margin: String, is_auto_add_margin: String, position_side: String, position_amount: String, symbol: String, unrealized_profit: String) -> SubAccountCoinFuturesPositionRiskDeliveryPositionRiskVosInner {
        SubAccountCoinFuturesPositionRiskDeliveryPositionRiskVosInner {
            entry_price,
            mark_price,
            leverage,
            isolated,
            isolated_wallet,
            isolated_margin,
            is_auto_add_margin,
            position_side,
            position_amount,
            symbol,
            unrealized_profit,
        }
    }
}

