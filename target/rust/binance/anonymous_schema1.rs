// AnonymousSchema1 represents a AnonymousSchema1 model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnonymousSchema1 {
    #[serde(rename="id")]
    pub id: String,
    #[serde(rename="method")]
    pub method: String,
    #[serde(rename="params", skip_serializing_if = "Option::is_none")]
    pub params: Option<Box<crate::AnonymousSchema4>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}
