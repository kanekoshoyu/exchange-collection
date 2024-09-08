// IndividualSymbolMiniTicker represents a IndividualSymbolMiniTicker model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndividualSymbolMiniTicker {
    #[serde(rename="e", skip_serializing_if = "Option::is_none")]
    pub e: Option<String>,
    #[serde(rename="E", skip_serializing_if = "Option::is_none")]
    pub reserved_e: Option<i64>,
    #[serde(rename="s", skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    #[serde(rename="c", skip_serializing_if = "Option::is_none")]
    pub c: Option<String>,
    #[serde(rename="o", skip_serializing_if = "Option::is_none")]
    pub o: Option<String>,
    #[serde(rename="h", skip_serializing_if = "Option::is_none")]
    pub h: Option<String>,
    #[serde(rename="l", skip_serializing_if = "Option::is_none")]
    pub l: Option<String>,
    #[serde(rename="v", skip_serializing_if = "Option::is_none")]
    pub v: Option<String>,
    #[serde(rename="q", skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}
