use binance::account::{
    Account, CancelReplaceRequest, OrderCancellation, OrderRequest, OrderStatusRequest, OrdersQuery,
};
use binance::api::Binance;
// use binance::errors
use binance::config::Config;
use binance::errors::Error as BinanceLibError;
use binance::general::General;
use binance::market::Market;
use binance::rest_model::{AccountInformation, Balance, OrderSide, OrderType, SymbolPrice};
use tracing::{error, info};

#[derive(Clone)]
pub struct Exchange {
    pub account: Account,
    pub market: Market,
    // api_key: String,
    // api_sec: String,
}

impl Exchange {
    pub fn new(api_key: String, api_sec: String) -> Self {
        // let api_key = api_key;
        // let api_sec = api_sec;
        let config = Config::default();
        let account =
            Binance::new_with_config(Some(api_key.clone()), Some(api_sec.clone()), &config);
        let market =
            Binance::new_with_config(Some(api_key.clone()), Some(api_sec.clone()), &config);
        Self {
            account,
            market,
            // api_key,
            // api_sec,
        }
    }

    pub async fn openshort(&self, symbol: &String, amount: f64) {
        let market_buy = OrderRequest {
            symbol: symbol.to_string(),
            quantity: Some(amount),
            order_type: OrderType::Market,
            side: OrderSide::Buy,
            ..OrderRequest::default()
        };
        match self.account.place_order(market_buy).await {
            Ok(answer) => info!("{:?}", answer),
            Err(e) => error!("Error: {e}"),
        }

        let market_sell = OrderRequest {
            symbol: symbol.to_string(),
            quantity: Some(0.001),
            order_type: OrderType::Market,
            side: OrderSide::Sell,
            ..OrderRequest::default()
        };
        match self.account.place_order(market_sell).await {
            Ok(answer) => info!("{:?}", answer),
            Err(e) => error!("Error: {e}"),
        }
    }

    pub async fn openlong(&self, symbol: &String, amount: f64) {
        let market_buy = OrderRequest {
            symbol: symbol.to_string(),
            quantity: Some(amount),
            order_type: OrderType::Market,
            side: OrderSide::Buy,
            ..OrderRequest::default()
        };
        match self.account.place_order(market_buy).await {
            Ok(answer) => info!("{:?}", answer),
            Err(e) => error!("Error: {e}"),
        }
    }

    pub async fn get_bidask_price(&self, symbol: &String, side: String) -> f64 {
        // Best price/qty on the order book for ONE symbol
        let price = self.market.get_book_ticker(symbol).await.ok().unwrap();
        if side == "ask" {
            price.ask_price
        } else {
            price.bid_price
        }
    }
    pub async fn get_balance(&self) -> Vec<Balance> {
        self.account.get_account().await.unwrap().balances
    }
}
