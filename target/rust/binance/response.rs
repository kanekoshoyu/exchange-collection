// Response represents a union of types: AnonymousSchema12, AnonymousSchema38
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum Response {
    #[serde(rename="AnonymousSchema12")]
    AnonymousSchema12(crate::AnonymousSchema12),
    #[serde(rename="AnonymousSchema38")]
    AnonymousSchema38(crate::AnonymousSchema38),
}

