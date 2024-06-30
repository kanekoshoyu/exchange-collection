// LightMeasuredPayload represents a LightMeasuredPayload model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LightMeasuredPayload {
    #[serde(rename="lumens", skip_serializing_if = "Option::is_none")]
    pub lumens: Option<i32>,
    #[serde(rename="sentAt", skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}
