// StreamParam represents a union of types: String, bool
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum StreamParam {
    #[serde(rename="StringResponse")]
    StringResponse(String),
    #[serde(rename="BoolResponse")]
    BoolResponse(bool),
}

