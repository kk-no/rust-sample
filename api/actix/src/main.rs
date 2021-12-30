mod handler;

pub use crate::handler::greet::*;
use actix_web::{middleware, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(greet::get_greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
