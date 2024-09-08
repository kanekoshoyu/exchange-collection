// PartialBookDepth represents a PartialBookDepth model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PartialBookDepth {
    #[serde(rename="lastUpdateId", skip_serializing_if = "Option::is_none")]
    pub last_update_id: Option<i64>,
    #[serde(rename="bids", skip_serializing_if = "Option::is_none")]
    pub bids: Option<Vec<Vec<String>>>,
    #[serde(rename="asks", skip_serializing_if = "Option::is_none")]
    pub asks: Option<Vec<Vec<Vec<String>>>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}
