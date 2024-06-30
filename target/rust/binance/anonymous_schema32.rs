// AnonymousSchema32 represents a AnonymousSchema32 model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnonymousSchema32 {
    #[serde(rename="rateLimitType", skip_serializing_if = "Option::is_none")]
    pub rate_limit_type: Option<String>,
    #[serde(rename="interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(rename="intervalNum", skip_serializing_if = "Option::is_none")]
    pub interval_num: Option<i32>,
    #[serde(rename="limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename="count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}
