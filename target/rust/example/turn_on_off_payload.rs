// TurnOnOffPayload represents a TurnOnOffPayload model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TurnOnOffPayload {
    #[serde(rename="command", skip_serializing_if = "Option::is_none")]
    pub command: Option<Box<crate::AnonymousSchema6>>,
    #[serde(rename="sentAt", skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}
