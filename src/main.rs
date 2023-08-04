use actix_rt::Arbiter;
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use binance_tv_rs::exchange::Exchange;
use binance_tv_rs::{Order, OrderSide};
use env_logger::Env;
use middleware::Logger;
use std::env;
use web::Json;

async fn trade_signal_handler(data: Order) {
    let pass = data.passpharse;
    let word = match env::var("PASSPHARSE") {
        Ok(expr) => expr.to_string(),
        Err(_) => {
            println!("PASSPHARSE Not Found make sure to setting environments");
            "".to_string()
        }
    };
    if pass != word {
        let task = Arbiter::try_current().is_some_and(|task| task.stop());
        let text = if task { "No!" } else { ":(" };
        println!("Invalid Passpharse :{text}");
        return;
    }
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
    let api_key = match env::var("BINANCE_API") {
        Ok(apikey) => apikey.to_string(),
        Err(_) => {
            println!("Not Found BINANCE_API Key in the environments");
            "".to_string()
        }
    };
    let api_sec = match env::var("BINANCE_SEC") {
        Ok(apikey) => apikey.to_string(),
        Err(_) => {
            println!("Not Found BINANCE_SEC in the environments");
            "".to_string()
        }
    };
    let exchange = Exchange::new(api_key.to_string(), api_sec.to_string());
    let bidask = match side {
        OrderSide::OpenLong => "ask".to_string(),
        OrderSide::OpenShort => "bid".to_string(),
        OrderSide::CloseLong => "bid".to_string(),
        OrderSide::CloseShort => "ask".to_string(),
    };
    let price = exchange.get_bidask_price(&symbol, bidask).await;
    println!("BTC NOW = {price}");
    let amount: f64 = if raw_amount.starts_with("@") {
        raw_amount.replace("@", "").parse().unwrap()
    } else if raw_amount.starts_with("%") {
        raw_amount.replace("%", "").parse().unwrap()
    } else if raw_amount.starts_with("$") {
        raw_amount.replace("$", "").parse().unwrap()
    } else {
        raw_amount.parse().unwrap()
    };
    let open_order = exchange.account.get_account().await.unwrap();
    println!("{:?}", open_order.account_type);
    // println!("{:?}", open_order.balances);

    // เอาไว้ล่างสุดเพื่อจะได้ล้าง Task
    let task = Arbiter::try_current().is_some_and(|task| task.stop());
    println!(
        "ORDER INFO: symbol: {symbol} side: {:#?} amount: {amount} leverage: {lev} Success: {task}",
        side
    );
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("https://www.github.com/vazw")
}

#[post("/order")]
async fn order(data: Json<Order>) -> impl Responder {
    let task = Arbiter::new();
    Arbiter::spawn(&task, async move {
        trade_signal_handler(data.clone()).await;
    });

    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(hello)
            .service(order)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
