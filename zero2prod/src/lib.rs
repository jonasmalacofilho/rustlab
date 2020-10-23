use actix_web::dev::Server;
use actix_web::{web, App, HttpServer, Responder};
use serde::Deserialize;
use std::net::TcpListener;

#[derive(Deserialize, Debug)]
struct SubscriberData {
    name: String,
    email: String,
}

async fn health_check() -> impl Responder {
    ""
}

async fn subscribe(form: web::Form<SubscriberData>) -> impl Responder {
    dbg!(form);
    ""
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
