use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/healthcheck")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("It's working!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .service(healthcheck)
            .default_service(web::to(||HttpResponse::NotFound()))
    })
    .workers(6)
    .bind(("localhost", 4000))?
    .run()
    .await
}

#[actix_web::test]
async fn not_found_route() {
    let mut app = test::init_service(
        App::new()
        .service(healthcheck)
        .default_service(web::to(|| HttpResponse::NotFound()))
    ).await;

    let req = test::TestRequest::with_uri("/crazy-path").to_request();

    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}