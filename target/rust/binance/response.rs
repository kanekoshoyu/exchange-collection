// Response represents a union of types: SuccessResponse, ErrorResponse
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum Response {
    #[serde(rename="SuccessResponse")]
    SuccessResponse(crate::SuccessResponse),
    #[serde(rename="ErrorResponse")]
    ErrorResponse(crate::ErrorResponse),
}

