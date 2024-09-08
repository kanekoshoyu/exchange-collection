// SlashStream represents a union of types: StreamControlResponse, Error, AggregateTradeStream, TradeStream, KlineStreamWithTimezoneOffset, KlineStreamWithUtc, IndividualSymbolMiniTicker, Vec<crate::AllMarketMiniTicker>, IndividualSymbolTicker, Vec<crate::AllMarketTicker>, IndividualSymbolRollingWindowStatistics, Vec<crate::AllMarketRollingWindowStat>, IndividualSymbolBookTicker, AveragePrice, PartialBookDepth, DiffDepth
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum SlashStream {
    #[serde(rename="StreamControlResponse")]
    StreamControlResponse(crate::StreamControlResponse),
    #[serde(rename="Error")]
    Error(crate::Error),
    #[serde(rename="AggregateTradeStream")]
    AggregateTradeStream(crate::AggregateTradeStream),
    #[serde(rename="TradeStream")]
    TradeStream(crate::TradeStream),
    #[serde(rename="KlineStreamWithTimezoneOffset")]
    KlineStreamWithTimezoneOffset(crate::KlineStreamWithTimezoneOffset),
    #[serde(rename="KlineStreamWithUtc")]
    KlineStreamWithUtc(crate::KlineStreamWithUtc),
    #[serde(rename="IndividualSymbolMiniTicker")]
    IndividualSymbolMiniTicker(crate::IndividualSymbolMiniTicker),
    #[serde(rename="AllMarketMiniTickers")]
    AllMarketMiniTickers(Vec<crate::AllMarketMiniTicker>),
    #[serde(rename="IndividualSymbolTicker")]
    IndividualSymbolTicker(crate::IndividualSymbolTicker),
    #[serde(rename="AllMarketTickers")]
    AllMarketTickers(Vec<crate::AllMarketTicker>),
    #[serde(rename="IndividualSymbolRollingWindowStatistics")]
    IndividualSymbolRollingWindowStatistics(crate::IndividualSymbolRollingWindowStatistics),
    #[serde(rename="AllMarketRollingWindowStatistics")]
    AllMarketRollingWindowStatistics(Vec<crate::AllMarketRollingWindowStat>),
    #[serde(rename="IndividualSymbolBookTicker")]
    IndividualSymbolBookTicker(crate::IndividualSymbolBookTicker),
    #[serde(rename="AveragePrice")]
    AveragePrice(crate::AveragePrice),
    #[serde(rename="PartialBookDepth")]
    PartialBookDepth(crate::PartialBookDepth),
    #[serde(rename="DiffDepth")]
    DiffDepth(crate::DiffDepth),
}

