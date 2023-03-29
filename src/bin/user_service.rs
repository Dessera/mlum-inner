use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

use mlum_inner::app_state::AppState;
use mlum_inner::routers::*;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL cannot be empty");
    let database = mongodb::Client::with_uri_str(&database_url)
        .await
        .expect("Failed to connect to database");

    let shared_data = web::Data::new(AppState {
        health_check_response: "App Service is OK.".to_string(),
        visit_count: Mutex::new(0),
        database,
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(users::user_routers)
            .configure(general::general_routers)
    };

    HttpServer::new(app).bind("127.0.0.1:9999")?.run().await
}
