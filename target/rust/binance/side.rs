// Side represents a Side model.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Side {
    #[serde(rename="BUY")]
    Buy,
    #[serde(rename="SELL")]
    Sell,
}
