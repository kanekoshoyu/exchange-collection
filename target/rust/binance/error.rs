// Error represents a Error model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Error {
    #[serde(rename="code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(rename="msg", skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}
