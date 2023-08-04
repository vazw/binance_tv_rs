pub mod exchange;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum OrderSide {
    OpenLong,
    OpenShort,
    CloseLong,
    CloseShort,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Order {
    pub passpharse: String,
    pub symbol: String,
    pub side: OrderSide,
    pub amount: String,
    pub leverage: String,
}
