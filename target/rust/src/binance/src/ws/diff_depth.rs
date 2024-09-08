// DiffDepth represents a DiffDepth model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DiffDepth {
    #[serde(rename="e", skip_serializing_if = "Option::is_none")]
    pub e: Option<String>,
    #[serde(rename="E", skip_serializing_if = "Option::is_none")]
    pub reserved_e: Option<i64>,
    #[serde(rename="s", skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    #[serde(rename="u", skip_serializing_if = "Option::is_none")]
    pub u: Option<i64>,
    #[serde(rename="b", skip_serializing_if = "Option::is_none")]
    pub b: Option<Vec<Vec<Vec<String>>>>,
    #[serde(rename="a", skip_serializing_if = "Option::is_none")]
    pub a: Option<Vec<Vec<Vec<String>>>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}
