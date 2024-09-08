// KlineInterval represents a KlineInterval model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KlineInterval {
    #[serde(rename="t", skip_serializing_if = "Option::is_none")]
    pub t: Option<i64>,
    #[serde(rename="T", skip_serializing_if = "Option::is_none")]
    pub reserved_t: Option<i64>,
    #[serde(rename="s", skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    #[serde(rename="i", skip_serializing_if = "Option::is_none")]
    pub i: Option<String>,
    #[serde(rename="f", skip_serializing_if = "Option::is_none")]
    pub f: Option<i32>,
    #[serde(rename="l", skip_serializing_if = "Option::is_none")]
    pub l: Option<String>,
    #[serde(rename="o", skip_serializing_if = "Option::is_none")]
    pub o: Option<String>,
    #[serde(rename="c", skip_serializing_if = "Option::is_none")]
    pub c: Option<String>,
    #[serde(rename="h", skip_serializing_if = "Option::is_none")]
    pub h: Option<String>,
    #[serde(rename="v", skip_serializing_if = "Option::is_none")]
    pub v: Option<String>,
    #[serde(rename="n", skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    #[serde(rename="x", skip_serializing_if = "Option::is_none")]
    pub x: Option<bool>,
    #[serde(rename="q", skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(rename="V", skip_serializing_if = "Option::is_none")]
    pub reserved_v: Option<String>,
    #[serde(rename="Q", skip_serializing_if = "Option::is_none")]
    pub reserved_q: Option<String>,
    #[serde(rename="B", skip_serializing_if = "Option::is_none")]
    pub b: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}
