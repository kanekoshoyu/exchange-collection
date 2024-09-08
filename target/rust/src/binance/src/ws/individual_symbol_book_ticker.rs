// IndividualSymbolBookTicker represents a IndividualSymbolBookTicker model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndividualSymbolBookTicker {
    #[serde(rename="u", skip_serializing_if = "Option::is_none")]
    pub u: Option<i32>,
    #[serde(rename="s", skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    #[serde(rename="b", skip_serializing_if = "Option::is_none")]
    pub b: Option<String>,
    #[serde(rename="B", skip_serializing_if = "Option::is_none")]
    pub reserved_b: Option<String>,
    #[serde(rename="a", skip_serializing_if = "Option::is_none")]
    pub a: Option<String>,
    #[serde(rename="A", skip_serializing_if = "Option::is_none")]
    pub reserved_a: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}
