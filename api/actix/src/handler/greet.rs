pub mod greet {
    use actix_web::{get, HttpRequest, Responder};

    #[get("/")]
    async fn get_greet(_: HttpRequest) -> impl Responder {
        format!("Hello World!")
    }
}
