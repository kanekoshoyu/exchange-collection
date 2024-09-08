// AveragePrice represents a AveragePrice model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AveragePrice {
    #[serde(rename="e", skip_serializing_if = "Option::is_none")]
    pub e: Option<String>,
    #[serde(rename="E", skip_serializing_if = "Option::is_none")]
    pub reserved_e: Option<i64>,
    #[serde(rename="s", skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    #[serde(rename="i", skip_serializing_if = "Option::is_none")]
    pub i: Option<String>,
    #[serde(rename="w", skip_serializing_if = "Option::is_none")]
    pub w: Option<String>,
    #[serde(rename="T", skip_serializing_if = "Option::is_none")]
    pub t: Option<i64>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}
