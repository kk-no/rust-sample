mod handler;

pub use crate::handler::greet::*;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(greet::GreetState::new(String::from("test_server")))
            .service(greet::get_greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
