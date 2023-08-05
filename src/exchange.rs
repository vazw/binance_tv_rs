use binance::api::*;
use binance::config::Config;
use binance::futures::account::{FuturesAccount, OrderRequest};
use binance::futures::general::FuturesGeneral;
use binance::futures::market::FuturesMarket;
use binance::futures::rest_model::*;
use tracing::{error, info};

#[derive(Clone)]
pub struct Exchange {
    pub account: FuturesAccount,
    pub market: FuturesMarket,
    pub general: FuturesGeneral,
    pub exchange_info: ExchangeInformation,
    pub account_info: AccountInformation,
    pub dual_postition: bool,
}

impl Exchange {
    pub async fn new(api_key: String, api_sec: String) -> Self {
        let config = Config::default();
        let account: FuturesAccount =
            Binance::new_with_config(Some(api_key.clone()), Some(api_sec.clone()), &config);
        let market: FuturesMarket =
            Binance::new_with_config(Some(api_key.clone()), Some(api_sec.clone()), &config);
        let general: FuturesGeneral =
            Binance::new_with_config(Some(api_key.clone()), Some(api_sec.clone()), &config);
        let exchange_info = general.exchange_info().await.unwrap();
        let account_info = account.account_information().await.unwrap();
        let dual_postition = match account_info.positions.iter().next().unwrap().position_side {
            PositionSide::Both => false,
            PositionSide::Long => true,
            PositionSide::Short => true,
        };

        Self {
            account,
            market,
            general,
            exchange_info,
            account_info,
            dual_postition,
        }
    }

    pub fn amount_to_precision(&self, num: f64, precision: i32) -> f64 {
        let multiplier = 10f64.powi(precision);
        (num.abs() * multiplier).round() / multiplier
    }

    pub async fn openshort(&self, symbol: &String, amount: f64) {
        let market_sell = OrderRequest {
            symbol: symbol.to_string(),
            quantity: Some(amount),
            order_type: OrderType::Market,
            side: OrderSide::Sell,
            position_side: match self.dual_postition {
                true => Some(PositionSide::Short),
                false => Some(PositionSide::Both),
            },
            ..OrderRequest::default()
        };
        match self.account.place_order(market_sell).await {
            Ok(answer) => info!("{:?}", answer),
            Err(e) => error!("Error: {e}"),
        }
    }

    pub async fn closeshort(&self, symbol: &String, amount: f64) {
        let market_sell = OrderRequest {
            symbol: symbol.to_string(),
            quantity: Some(amount),
            order_type: OrderType::Market,
            side: OrderSide::Buy,
            position_side: match self.dual_postition {
                true => Some(PositionSide::Short),
                false => Some(PositionSide::Both),
            },
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
            position_side: match self.dual_postition {
                true => Some(PositionSide::Long),
                false => Some(PositionSide::Both),
            },
            ..OrderRequest::default()
        };
        match self.account.place_order(market_buy).await {
            Ok(answer) => info!("{:?}", answer),
            Err(e) => error!("Error: {e}"),
        }
    }

    pub async fn closelong(&self, symbol: &String, amount: f64) {
        let market_buy = OrderRequest {
            symbol: symbol.to_string(),
            quantity: Some(amount),
            order_type: OrderType::Market,
            side: OrderSide::Sell,
            position_side: match self.dual_postition {
                true => Some(PositionSide::Long),
                false => Some(PositionSide::Both),
            },
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

    pub async fn get_balance(&self) -> Vec<AccountBalance> {
        self.account.account_balance().await.unwrap()
    }

    pub async fn update_account(&mut self) {
        self.account_info = self.account.account_information().await.unwrap();
    }

    pub fn get_position_mode(&self) -> bool {
        let account_info = self.account_info.clone();
        match account_info.positions.iter().next().unwrap().position_side {
            PositionSide::Both => false,
            PositionSide::Long => true,
            PositionSide::Short => true,
        }
    }

    pub async fn get_symbol_info(&self, filters: String) -> Symbol {
        let symbols = &self.exchange_info.symbols;
        let symbol = symbols
            .iter()
            .filter(|sym| sym.symbol == filters)
            .next()
            .unwrap();
        symbol.clone()
    }
    pub async fn get_current_position(&self, symbol: String) -> AccountPosition {
        let open_order = self.account_info.clone();
        open_order
            .positions
            .iter()
            .filter(|posi| posi.symbol.eq(&symbol))
            .next()
            .unwrap()
            .clone()
    }
}
