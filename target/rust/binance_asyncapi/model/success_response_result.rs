// SuccessResponseResult represents a SuccessResponseResult model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SuccessResponseResult {
    #[serde(rename="symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename="orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i32>,
    #[serde(rename="orderListId", skip_serializing_if = "Option::is_none")]
    pub order_list_id: Option<i32>,
    #[serde(rename="clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename="transactTime", skip_serializing_if = "Option::is_none")]
    pub transact_time: Option<i32>,
    #[serde(rename="price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename="origQty", skip_serializing_if = "Option::is_none")]
    pub orig_qty: Option<String>,
    #[serde(rename="executedQty", skip_serializing_if = "Option::is_none")]
    pub executed_qty: Option<String>,
    #[serde(rename="cummulativeQuoteQty", skip_serializing_if = "Option::is_none")]
    pub cummulative_quote_qty: Option<String>,
    #[serde(rename="status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename="timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(rename="side", skip_serializing_if = "Option::is_none")]
    pub side: Option<Box<crate::Side>>,
    #[serde(rename="type", skip_serializing_if = "Option::is_none")]
    pub reserved_type: Option<Box<crate::OrderType>>,
    #[serde(rename="workingTime", skip_serializing_if = "Option::is_none")]
    pub working_time: Option<i32>,
    #[serde(rename="selfTradePreventionMode", skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}
