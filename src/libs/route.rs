use crate::libs::signal::*;
use actix_rt::Arbiter;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, HttpResponse, Responder};
use web::Json;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body("<meta http-equiv=\"Refresh\" content=\"0; \
            URL=https://github.com/vazw\" />")
}

#[post("/order")]
pub async fn order(data: Json<Order>) -> impl Responder {
    // This route recieve signal and
    //start new threads then return HTTP accept code.
    let task = Arbiter::new();
    Arbiter::spawn(&task, async move {
        trade_signal_handler(data.clone()).await;
    });

    HttpResponse::Ok()
        .status(StatusCode::from_u16(202).unwrap())
        .finish()
}
