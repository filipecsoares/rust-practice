pub mod todo_api_web;

use actix_web::{App, HttpServer};
use todo_api_web::routes::app_routes;

use num_cpus;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(app_routes))
        .workers(num_cpus::get() + 2)
        .bind(("localhost", 4004))
        .unwrap()
        .run()
        .await
}
