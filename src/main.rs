pub mod libs;
use crate::libs::route::*;
use actix_web::{middleware, App, HttpServer};
use env_logger::Env;
use std::env::args;
use tracing::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let mut defualt_port: u16 = 80;
    let parsed_args: Vec<String> = args().collect();
    if parsed_args.len() > 1 {
        let port = parsed_args[1].to_owned();
        info!("Starting Service on port {port}");
        defualt_port = port.parse().unwrap();
    } else {
        info!("Starting Service on port {defualt_port}");
    }
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::new("%a %s %{User-Agent}i"))
            .service(hello)
            .service(order)
    })
    .bind(("0.0.0.0", defualt_port))?
    .run()
    .await
}
