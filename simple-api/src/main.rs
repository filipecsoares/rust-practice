mod handler;
mod model;
mod response;

use model::{QueryOptions, DB};
use warp::{Filter, Rejection};

type WebResult<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "api=info");
    }
    pretty_env_logger::init();
    let db = model::todo_db();
    let todo_router = warp::path!("api" / "todos");
    
    let health_checker = warp::path!("api" / "healthchecker" )
        .and(warp::get())
        .and_then(handler::health_checker_handler);
    
    let todo_routes = todo_router
            .and(warp::get())
            .and(warp::query::<QueryOptions>())
            .and(with_db(db.clone()))
            .and_then(handler::todos_list_handler);
            
    let routes = todo_routes.with(warp::log("api"))
        .or(health_checker);
    println!("Server started!");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}