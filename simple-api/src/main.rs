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
    let todo_router_id = warp::path!("api" / "todos" / String);
    let health_checker = warp::path!("api" / "healthchecker" )
        .and(warp::get())
        .and_then(handler::health_checker_handler);
    
    let todo_routes = todo_router
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::create_todo_handler)
        .or(todo_router
            .and(warp::get())
            .and(warp::query::<QueryOptions>())
            .and(with_db(db.clone()))
            .and_then(handler::todos_list_handler));
            
    let todo_routes_id = todo_router_id
        .and(warp::patch())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::edit_todo_handler)
        .or(todo_router_id
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handler::get_todo_handler));
            
    let routes = todo_routes.with(warp::log("api"))
        .or(todo_routes_id)
        .or(health_checker);
    println!("Server started!");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}