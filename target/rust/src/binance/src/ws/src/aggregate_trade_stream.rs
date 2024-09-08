// AggregateTradeStream represents a AggregateTradeStream model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AggregateTradeStream {
    #[serde(rename="e", skip_serializing_if = "Option::is_none")]
    pub e: Option<String>,
    #[serde(rename="E", skip_serializing_if = "Option::is_none")]
    pub reserved_e: Option<i64>,
    #[serde(rename="s", skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    #[serde(rename="a", skip_serializing_if = "Option::is_none")]
    pub a: Option<i32>,
    #[serde(rename="p", skip_serializing_if = "Option::is_none")]
    pub p: Option<String>,
    #[serde(rename="q", skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(rename="f", skip_serializing_if = "Option::is_none")]
    pub f: Option<i32>,
    #[serde(rename="l", skip_serializing_if = "Option::is_none")]
    pub l: Option<i32>,
    #[serde(rename="T", skip_serializing_if = "Option::is_none")]
    pub t: Option<i64>,
    #[serde(rename="m", skip_serializing_if = "Option::is_none")]
    pub m: Option<bool>,
    #[serde(rename="M", skip_serializing_if = "Option::is_none")]
    pub reserved_m: Option<bool>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}
