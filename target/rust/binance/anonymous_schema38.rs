// AnonymousSchema38 represents a AnonymousSchema38 model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnonymousSchema38 {
    #[serde(rename="id")]
    pub id: String,
    #[serde(rename="status")]
    pub status: i32,
    #[serde(rename="error")]
    pub error: Box<crate::AnonymousSchema41>,
    #[serde(rename="rateLimits", skip_serializing_if = "Option::is_none")]
    pub rate_limits: Option<Vec<crate::AnonymousSchema45>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}
