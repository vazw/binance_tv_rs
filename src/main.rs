use actix_web::{middleware, App, HttpServer};
use binance_tv_rs::route::*;
use env_logger::Env;
use middleware::Logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(hello)
            .service(order)
    })
    .bind(("0.0.0.0", 443))?
    .run()
    .await
}
