// StreamControlMessagePayload represents a StreamControlMessagePayload model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StreamControlMessagePayload {
    #[serde(rename="method")]
    pub method: Box<crate::StreamControlMethod>,
    #[serde(rename="params", skip_serializing_if = "Option::is_none")]
    pub params: Option<Box<crate::StreamParam>>,
    #[serde(rename="id")]
    pub id: i32,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}
