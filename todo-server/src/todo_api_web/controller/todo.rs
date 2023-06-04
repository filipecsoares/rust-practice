use crate::todo_api_web::model::todo::{TodoCard, TodoIdResponse};
use actix_web::{post, web, HttpResponse, Responder};
use uuid::Uuid;

#[post("/create")]
pub async fn create_todo(info: web::Json<TodoCard>) -> impl Responder {
    println!("{:?}", info);
    let new_id = Uuid::new_v4();
    HttpResponse::Created()
        .content_type("application/json")
        .body(
            serde_json::to_string(&TodoIdResponse::new(new_id))
                .expect("failed to serialize ContactsBatchResponseId"),
        )
}
