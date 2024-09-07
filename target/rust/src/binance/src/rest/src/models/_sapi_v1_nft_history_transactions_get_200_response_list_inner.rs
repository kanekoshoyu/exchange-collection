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
pub struct SapiV1NftHistoryTransactionsGet200ResponseListInner {
    /// 0: purchase order, 1: sell order, 2: royalty income, 3: primary market order, 4: mint fee
    #[serde(rename = "orderNo")]
    pub order_no: String,
    #[serde(rename = "tokens")]
    pub tokens: Vec<models::SapiV1NftHistoryTransactionsGet200ResponseListInnerTokensInner>,
    #[serde(rename = "tradeTime")]
    pub trade_time: i64,
    #[serde(rename = "tradeAmount")]
    pub trade_amount: String,
    #[serde(rename = "tradeCurrency")]
    pub trade_currency: String,
}

impl SapiV1NftHistoryTransactionsGet200ResponseListInner {
    pub fn new(order_no: String, tokens: Vec<models::SapiV1NftHistoryTransactionsGet200ResponseListInnerTokensInner>, trade_time: i64, trade_amount: String, trade_currency: String) -> SapiV1NftHistoryTransactionsGet200ResponseListInner {
        SapiV1NftHistoryTransactionsGet200ResponseListInner {
            order_no,
            tokens,
            trade_time,
            trade_amount,
            trade_currency,
        }
    }
}
