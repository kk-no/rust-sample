pub mod greet {
    use actix_web::{get, web, HttpRequest, Responder};

    pub struct GreetState {
        app_name: String,
    }

    impl GreetState {
        pub fn new(app_name: String) -> GreetState {
            GreetState{app_name: app_name}
        }
    }

    #[get("/")]
    async fn get_greet(_: HttpRequest, state: web::Data<GreetState>) -> impl Responder {
        format!("Hello {}!", &state.app_name)
    }
}
