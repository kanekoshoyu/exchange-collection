// StreamControlMethod represents a StreamControlMethod model.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum StreamControlMethod {
    #[serde(rename="SUBSCRIBE")]
    Subscribe,
    #[serde(rename="UNSUBSCRIBE")]
    Unsubscribe,
    #[serde(rename="LIST_SUBSCRIPTIONS")]
    ListSubscriptions,
    #[serde(rename="SET_PROPERTY")]
    SetProperty,
}
