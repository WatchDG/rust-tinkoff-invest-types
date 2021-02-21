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
    pub instrument_type: InstrumentType,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OperationTypeWithCommission {
    Buy,
    BuyCard,
    Sell,
    BrokerCommission,
    ExchangeCommission,
    ServiceCommission,
    MarginCommission,
    OtherCommission,
    PayIn,
    PayOut,
    Tax,
    TaxLucre,
    TaxDividend,
    TaxCoupon,
    TaxBack,
    Repayment,
    PartRepayment,
    Coupon,
    Dividend,
    SecurityIn,
    SecurityOut,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OperationStatus {
    Done,
    Decline,
    Progress,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OperationTrade {
    #[serde(rename(serialize = "tradeId", deserialize = "tradeId"))]
    pub trade_id: String,
    pub date: String,
    pub price: f64,
    pub quantity: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Operation {
    pub id: String,
    pub status: OperationStatus,
    pub trades: Option<Vec<OperationTrade>>,
    pub commission: Option<MoneyAmount>,
    pub currency: Currency,
    pub payment: f64,
    pub price: Option<f64>,
    pub quantity: Option<u64>,
    pub figi: Option<String>,
    #[serde(rename(serialize = "instrumentType", deserialize = "instrumentType"))]
    pub instrument_type: Option<InstrumentType>,
    #[serde(rename(serialize = "isMarginCall", deserialize = "isMarginCall"))]
    pub is_margin_call: bool,
    pub date: String,
    #[serde(rename(serialize = "operationType", deserialize = "operationType"))]
    pub operation_type: Option<OperationTypeWithCommission>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OperationsPayload {
    pub operations: Vec<Operation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CandlestickResolution {
    #[serde(rename(serialize = "1min", deserialize = "1min"))]
    MIN1,
    #[serde(rename(serialize = "2min", deserialize = "2min"))]
    MIN2,
    #[serde(rename(serialize = "3min", deserialize = "3min"))]
    MIN3,
    #[serde(rename(serialize = "5min", deserialize = "5min"))]
    MIN5,
    #[serde(rename(serialize = "10min", deserialize = "10min"))]
    MIN10,
    #[serde(rename(serialize = "15min", deserialize = "15min"))]
    MIN15,
    #[serde(rename(serialize = "30min", deserialize = "30min"))]
    MIN30,
    #[serde(rename(serialize = "hour", deserialize = "hour"))]
    HOUR,
    #[serde(rename(serialize = "day", deserialize = "day"))]
    DAY,
    #[serde(rename(serialize = "week", deserialize = "week"))]
    WEEK,
    #[serde(rename(serialize = "month", deserialize = "month"))]
    MONTH,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Candlestick {
    pub figi: String,
    pub interval: CandlestickResolution,
    #[serde(rename(serialize = "time", deserialize = "time"))]
    pub datetime: String,
    #[serde(rename(serialize = "o", deserialize = "o"))]
    pub open: f64,
    #[serde(rename(serialize = "c", deserialize = "c"))]
    pub close: f64,
    #[serde(rename(serialize = "h", deserialize = "h"))]
    pub high: f64,
    #[serde(rename(serialize = "l", deserialize = "l"))]
    pub low: f64,
    #[serde(rename(serialize = "v", deserialize = "v"))]
    pub volume: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CandlesticksPayload {
    pub figi: String,
    pub interval: CandlestickResolution,
    pub candles: Vec<Candlestick>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderResponse {
    pub price: f64,
    pub quantity: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TradeStatus {
    NormalTrading,
    NotAvailableForTrading,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Orderbook {
    pub figi: String,
    pub depth: u8,
    pub bids: Vec<OrderResponse>,
    pub asks: Vec<OrderResponse>,
    #[serde(rename(serialize = "tradeStatus", deserialize = "tradeStatus"))]
    pub trade_status: TradeStatus,
    #[serde(rename(serialize = "minPriceIncrement", deserialize = "minPriceIncrement"))]
    pub min_price_increment: f64,
    #[serde(rename(serialize = "faceValue", deserialize = "faceValue"))]
    pub face_value: Option<f64>,
    #[serde(rename(serialize = "lastPrice", deserialize = "lastPrice"))]
    pub last_price: Option<f64>,
    #[serde(rename(serialize = "closePrice", deserialize = "closePrice"))]
    pub close_price: Option<f64>,
    #[serde(rename(serialize = "limitUp", deserialize = "limitUp"))]
    pub limit_up: Option<f64>,
    #[serde(rename(serialize = "limitDown", deserialize = "limitDown"))]
    pub limit_down: Option<f64>,
}
