// AnonymousSchema12 represents a AnonymousSchema12 model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnonymousSchema12 {
    #[serde(rename="id")]
    pub id: String,
    #[serde(rename="status")]
    pub status: i32,
    #[serde(rename="result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<crate::AnonymousSchema15>>,
    #[serde(rename="rateLimits", skip_serializing_if = "Option::is_none")]
    pub rate_limits: Option<Vec<crate::AnonymousSchema32>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}
