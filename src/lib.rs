use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum Currency {
    RUB,
    USD,
    EUR,
    GBP,
    HKD,
    CHF,
    JPY,
    CNY,
    TRY,
}

#[derive(Deserialize, Debug)]
pub enum InstrumentType {
    Stock,
    Currency,
    Bond,
    Etf,
}

#[derive(Deserialize, Debug)]
pub enum OperationType {
    Buy,
    Sell,
}

#[derive(Deserialize, Debug)]
pub enum OrderStatus {
    New,
    PartiallyFill,
    Fill,
    Cancelled,
    Replaced,
    PendingCancel,
    Rejected,
    PendingReplace,
    PendingNew,
}

#[derive(Deserialize, Debug)]
pub enum OrderType {
    Limit,
    Market,
}

#[derive(Deserialize, Debug)]
pub enum BrokerAccountType {
    Tinkoff,
    TinkoffIis,
}

#[derive(Deserialize, Debug)]
pub struct TinkoffUserAccount {
    #[serde(rename(serialize = "brokerAccountId", deserialize = "brokerAccountId"))]
    pub broker_account_id: String,
    #[serde(rename(serialize = "brokerAccountType", deserialize = "brokerAccountType"))]
    pub broker_account_type: String,
}

#[derive(Deserialize, Debug)]
pub struct TinkoffUserAccounts {
    pub accounts: Vec<TinkoffUserAccount>,
}

#[derive(Deserialize, Debug)]
pub struct TinkoffMarketInstrument {
    pub figi: String,
    pub ticker: String,
    pub name: String,
    pub isin: Option<String>,
    #[serde(rename(deserialize = "minPriceIncrement"))]
    pub min_price_increment: Option<f64>,
    pub lot: u64,
    pub currency: Option<Currency>,
    #[serde(rename(deserialize = "type"))]
    pub type_: InstrumentType,
}

#[derive(Deserialize, Debug)]
pub struct TinkoffMarketInstruments {
    pub instruments: Vec<TinkoffMarketInstrument>,
}

#[derive(Deserialize, Debug)]
pub struct TinkoffResponseData<P> {
    #[serde(rename(serialize = "trackingId", deserialize = "trackingId"))]
    pub tracking_id: String,
    pub payload: P,
    pub status: String,
}

#[derive(Deserialize, Debug)]
pub struct Order {
    #[serde(rename(serialize = "orderId", deserialize = "orderId"))]
    pub order_id: String,
    pub figi: String,
    pub operation: OperationType,
    pub status: OrderStatus,
    #[serde(rename(serialize = "requestedLots", deserialize = "requestedLots"))]
    pub requested_lots: u64,
    #[serde(rename(serialize = "executedLots", deserialize = "executedLots"))]
    pub executed_lots: u64,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type_: OrderType,
    pub price: f64,
}
