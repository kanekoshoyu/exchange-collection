// StreamControlResponse represents a StreamControlResponse model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StreamControlResponse {
    #[serde(rename="result")]
    pub result: Vec<String>,
    #[serde(rename="id")]
    pub id: i32,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}
