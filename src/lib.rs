use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InstrumentType {
    Stock,
    Currency,
    Bond,
    Etf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OperationType {
    Buy,
    Sell,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OrderType {
    Limit,
    Market,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BrokerAccountType {
    Tinkoff,
    TinkoffIis,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MoneyAmount {
    pub currency: Currency,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAccount {
    #[serde(rename(serialize = "brokerAccountId", deserialize = "brokerAccountId"))]
    pub broker_account_id: String,
    #[serde(rename(serialize = "brokerAccountType", deserialize = "brokerAccountType"))]
    pub broker_account_type: BrokerAccountType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAccountsPayload {
    pub accounts: Vec<UserAccount>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketInstrument {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketInstrumentsPayload {
    pub instruments: Vec<MarketInstrument>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlacedOrder {
    #[serde(rename(serialize = "orderId", deserialize = "orderId"))]
    pub order_id: String,
    pub operation: OperationType,
    pub status: OrderStatus,
    #[serde(rename(serialize = "rejectReason", deserialize = "rejectReason"))]
    pub reject_reason: Option<String>,
    pub message: Option<String>,
    #[serde(rename(serialize = "requestedLots", deserialize = "requestedLots"))]
    pub requested_lots: u64,
    #[serde(rename(serialize = "executedLots", deserialize = "executedLots"))]
    pub executed_lots: u64,
    pub commission: Option<MoneyAmount>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorPayload {
    pub code: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseData<P> {
    #[serde(rename(serialize = "trackingId", deserialize = "trackingId"))]
    pub tracking_id: String,
    pub payload: P,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PortfolioPosition {
    pub figi: String,
    pub ticker: Option<String>,
    pub isin: Option<String>,
    #[serde(rename(serialize = "instrumentType", deserialize = "instrumentType"))]
    pub instrument_type: Option<InstrumentType>,
    pub balance: f64,
    pub blocked: Option<f64>,
    #[serde(rename(serialize = "expectedYield", deserialize = "expectedYield"))]
    pub expected_yield: Option<MoneyAmount>,
    pub lots: u64,
    #[serde(rename(
        serialize = "averagePositionPrice",
        deserialize = "averagePositionPrice"
    ))]
    pub average_position_price: Option<MoneyAmount>,
    #[serde(rename(
        serialize = "averagePositionPriceNoNkd",
        deserialize = "averagePositionPriceNoNkd"
    ))]
    pub average_position_price_no_nkd: Option<MoneyAmount>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PortfolioPayload {
    pub positions: Vec<PortfolioPosition>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyPortfolioPosition {
    pub currency: Currency,
    pub balance: f64,
    pub blocked: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyPortfolioPayload {
    pub currencies: Vec<CurrencyPortfolioPosition>,
}
