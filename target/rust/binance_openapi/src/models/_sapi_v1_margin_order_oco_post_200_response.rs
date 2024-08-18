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
pub struct SapiV1MarginOrderOcoPost200Response {
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    #[serde(rename = "contingencyType")]
    pub contingency_type: String,
    #[serde(rename = "listStatusType")]
    pub list_status_type: String,
    #[serde(rename = "listOrderStatus")]
    pub list_order_status: String,
    #[serde(rename = "listClientOrderId")]
    pub list_client_order_id: String,
    #[serde(rename = "transactionTime")]
    pub transaction_time: i64,
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// will not return if no margin trade happens
    #[serde(rename = "marginBuyBorrowAmount")]
    pub margin_buy_borrow_amount: String,
    /// will not return if no margin trade happens
    #[serde(rename = "marginBuyBorrowAsset")]
    pub margin_buy_borrow_asset: String,
    #[serde(rename = "isIsolated")]
    pub is_isolated: bool,
    #[serde(rename = "orders")]
    pub orders: Vec<models::SapiV1MarginOrderOcoPost200ResponseOrdersInner>,
    #[serde(rename = "orderReports")]
    pub order_reports: Vec<models::SapiV1MarginOrderOcoPost200ResponseOrderReportsInner>,
}

impl SapiV1MarginOrderOcoPost200Response {
    pub fn new(order_list_id: i64, contingency_type: String, list_status_type: String, list_order_status: String, list_client_order_id: String, transaction_time: i64, symbol: String, margin_buy_borrow_amount: String, margin_buy_borrow_asset: String, is_isolated: bool, orders: Vec<models::SapiV1MarginOrderOcoPost200ResponseOrdersInner>, order_reports: Vec<models::SapiV1MarginOrderOcoPost200ResponseOrderReportsInner>) -> SapiV1MarginOrderOcoPost200Response {
        SapiV1MarginOrderOcoPost200Response {
            order_list_id,
            contingency_type,
            list_status_type,
            list_order_status,
            list_client_order_id,
            transaction_time,
            symbol,
            margin_buy_borrow_amount,
            margin_buy_borrow_asset,
            is_isolated,
            orders,
            order_reports,
        }
    }
}
