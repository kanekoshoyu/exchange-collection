// SuccessResponse represents a SuccessResponse model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SuccessResponse {
    #[serde(rename="id")]
    pub id: String,
    #[serde(rename="status")]
    pub status: i32,
    #[serde(rename="result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<crate::SuccessResponseResult>>,
    #[serde(rename="rateLimits", skip_serializing_if = "Option::is_none")]
    pub rate_limits: Option<Vec<crate::RateLimit>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}
