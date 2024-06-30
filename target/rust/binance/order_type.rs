// OrderType represents a OrderType model.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OrderType {
    #[serde(rename="LIMIT")]
    Limit,
    #[serde(rename="MARKET")]
    Market,
}
