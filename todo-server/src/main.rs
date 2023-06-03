use actix_web::{get, http::StatusCode, test, web, App, HttpResponse, HttpServer, Responder};

#[get("/healthcheck")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("It's working!")
}

#[get("/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong!")
}

#[get("/~/ready")]
pub async fn readiness() -> impl Responder {
    let process = std::process::Command::new("sh")
        .arg("-c")
        .arg("echo hello")
        .output();
    match process {
        Ok(_) => HttpResponse::Accepted(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(readiness)
            .service(healthcheck)
            .default_service(web::to(|| HttpResponse::NotFound()))
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
            .default_service(web::to(|| HttpResponse::NotFound())),
    )
    .await;

    let req = test::TestRequest::with_uri("/crazy-path").to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
