use crate::libs::{exchange::Exchange, notify::notify_send};
use actix_rt::Arbiter;
use serde::Deserialize;
use std::env;
use tracing::info;

#[derive(Debug, Clone, Deserialize)]
pub enum NewOrderSide {
    OpenLong,
    OpenShort,
    CloseLong,
    CloseShort,
}
impl NewOrderSide {
    pub fn is_close_order(&self) -> bool {
        match self {
            Self::OpenLong | Self::OpenShort => false,
            Self::CloseLong | Self::CloseShort => true,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Order {
    pub passpharse: String,
    pub symbol: String,
    pub side: NewOrderSide,
    pub amount: String,
    pub leverage: String,
}

pub async fn trade_signal_handler(data: Order) {
    // Start with compare passpharse and reject if invalid.
    let pass = data.passpharse;
    let word = match env::var("PASSPHARSE") {
        Ok(expr) => expr.to_string(),
        Err(_) => {
            info!("PASSPHARSE Not Found make sure to setting environments");
            "".to_string()
        }
    };
    if pass != word {
        Arbiter::try_current().unwrap().stop();
        info!("Invalid Passpharse");
        return;
    }

    // Extract data from signal
    let raw_symbol = data.symbol;
    let symbol = if raw_symbol.ends_with(".P") {
        raw_symbol.replace(".P", "")
    } else if raw_symbol.ends_with("PERP") {
        raw_symbol.replace("PERP", "")
    } else {
        raw_symbol
    };
    let side = data.side;
    let raw_amount = data.amount;
    let lev = data.leverage;

    // Initialize exchange infomations.
    let api_key = match env::var("BINANCE_API") {
        Ok(apikey) => apikey.to_string(),
        Err(_) => {
            info!("Not Found BINANCE_API Key in the environments");
            "".to_string()
        }
    };
    let api_sec = match env::var("BINANCE_SEC") {
        Ok(apikey) => apikey.to_string(),
        Err(_) => {
            info!("Not Found BINANCE_SEC in the environments");
            "".to_string()
        }
    };
    let mut exchange = Exchange::new(api_key.to_string(), api_sec.to_string()).await;

    // Business Logic begin here
    let symbol_info = exchange.get_symbol_info(symbol.clone()).await;
    let bidask = match side {
        NewOrderSide::OpenLong | NewOrderSide::CloseShort => "ask".to_string(),
        NewOrderSide::OpenShort | NewOrderSide::CloseLong => "bid".to_string(),
    };
    let open_positions = exchange.get_current_position(symbol.clone()).await;
    let price = exchange.get_bidask_price(&symbol, bidask).await;

    // Unwarp amount from signal
    let amount: f64 = if raw_amount.starts_with("@") {
        raw_amount.replace("@", "").parse().unwrap()
    } else if raw_amount.starts_with("%") && side.is_close_order() {
        (raw_amount.replace("%", "").parse::<f64>().unwrap() / 100 as f64)
            * &open_positions.position_amount
    } else if raw_amount.starts_with("$") {
        raw_amount.replace("$", "").parse::<f64>().unwrap() / price as f64
    } else {
        raw_amount.parse().unwrap()
    };
    let unpnl = open_positions.unrealized_profit;
    let unpnl = exchange.price_to_precision(unpnl);
    let balance = exchange.account_info.total_wallet_balance.clone();
    let balance = exchange.price_to_precision(balance);
    let margin = amount * price / lev.parse::<f64>().unwrap();
    let margin = exchange.price_to_precision(margin);
    let amount = exchange.amount_to_precision(amount, symbol_info.quantity_precision as i32);
    let sum_margin = exchange.get_all_position_margin();
    if amount != 0.0 {
        match side {
            NewOrderSide::OpenLong => {
                match exchange
                    .account
                    .change_initial_leverage(&symbol, lev.parse::<u8>().unwrap())
                    .await
                {
                    Ok(respone) => info!("Changed Leverage : {:?}", respone.leverage),
                    Err(error) => info!("Can not Changed Leverage: {:?}", error),
                };
                if sum_margin < balance * 3.0 {
                    exchange.openlong(&symbol, amount).await;
                    exchange.update_account().await;
                    let balance = exchange.account_info.total_wallet_balance.clone();
                    let balance = exchange.price_to_precision(balance);
                    notify_send(format!(
                        "ORDER INFO:\nSymbol: {symbol}\nPrice: {price}\nSignal: {:#?}\n\
Amount: {amount}\nLeverage: {lev}\nMargin: {margin} $\nBalance : {balance} $",
                        side
                    ))
                    .await;
                };
            }
            NewOrderSide::OpenShort => {
                match exchange
                    .account
                    .change_initial_leverage(&symbol, lev.parse::<u8>().unwrap())
                    .await
                {
                    Ok(respone) => info!("Changed Leverage : {:?}", respone.leverage),
                    Err(error) => info!("Can not Changed Leverage: {:?}", error),
                };
                if sum_margin < balance * 3.0 {
                    exchange.openshort(&symbol, amount).await;
                    exchange.update_account().await;
                    let balance = exchange.account_info.total_wallet_balance.clone();
                    let balance = exchange.price_to_precision(balance);
                    notify_send(format!(
                        "ORDER INFO:\nSymbol: {symbol}\nPrice: {price}\nSignal: {:#?}\n\
Amount: {amount}\nLeverage: {lev}\nMargin: {margin} $\nBalance : {balance} $",
                        side
                    ))
                    .await;
                };
            }
            NewOrderSide::CloseLong => {
                exchange.closelong(&symbol, amount).await;
                exchange.update_account().await;
                let balance = exchange.account_info.total_wallet_balance.clone();
                let balance = exchange.price_to_precision(balance);
                notify_send(format!(
                    "ORDER INFO:\nSymbol: {symbol}\nPrice: {price}\nSignal: {:#?}\n\
Amount: {amount}\nLeverage: {lev}\nClosed P/L: {unpnl} $\nBalance : {balance} $",
                    side
                ))
                .await;
            }
            NewOrderSide::CloseShort => {
                exchange.closeshort(&symbol, amount).await;
                exchange.update_account().await;
                let balance = exchange.account_info.total_wallet_balance.clone();
                let balance = exchange.price_to_precision(balance);
                notify_send(format!(
                    "ORDER INFO:\nSymbol: {symbol}\nPrice: {price}\nSignal: {:#?}\n\
Amount: {amount}\nLeverage: {lev}\nClosed P/L: {unpnl} $\nBalance : {balance} $",
                    side
                ))
                .await;
            }
        }
    }

    info!(
        "ORDER INFO: symbol: {symbol} Price: {price} side: {:#?} amount: {amount} leverage: {lev}",
        side
    );
    // เอาไว้ล่างสุดเพื่อจะได้ล้าง Task
    Arbiter::try_current().unwrap().stop();
}
