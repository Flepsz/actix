use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::{Pool, Postgres};

mod db {
    pub mod connection;
}

mod services;

#[derive(Clone)]
pub struct AppState {
    client: Pool<Postgres>,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let _pool = db::connection::start_connection().await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                client: _pool.clone(),
            }))
            .service(index)
            .configure(services::users::services::users_routes)
    })
    .bind(("127.0.0.1", 7777))?
    .run()
    .await
}
