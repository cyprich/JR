use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, put, web};
use chrono::{NaiveDate, TimeDelta};
use serde::{Deserialize, Serialize};
use task_library::{
    control::{self},
    task::Task,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(hello, get_tasks, get_task_by_id, add_task, delete_task_by_id),
    components(schemas(SimpleResponse))
)]
struct ApiDoc;

#[derive(serde::Serialize, utoipa::ToSchema)]
struct SimpleResponse {
    status: u16,
}

#[utoipa::path(
    responses(
        (status = 200, description = "API is alive", body = SimpleResponse)
    )
)]
#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(SimpleResponse { status: 200 })
}

#[utoipa::path]
#[get("/get_tasks")]
async fn get_tasks() -> impl Responder {
    let tasks = control::db::list();
    HttpResponse::Ok().json(tasks)
}

#[utoipa::path]
#[get("/get_task_by_id/{task_id}")]
async fn get_task_by_id(path: web::Path<i32>) -> impl Responder {
    let task_id = path.into_inner();
    let tasks = control::db::list_by_id(task_id);
    HttpResponse::Ok().json(tasks)
}

#[derive(Deserialize, Serialize, utoipa::ToSchema, Debug)]
struct AddTaskData {
    id: i32,
    name: String,
    description: String,
    priority: i32,
    planned_from: NaiveDate,
    planned_duration: i32,
    real_from: Option<NaiveDate>,
    real_duration: Option<i32>,
}

#[utoipa::path]
#[put("/add_task")]
async fn add_task(data: web::Json<AddTaskData>) -> impl Responder {
    let t = Task {
        id: data.id,
        name: data.name.clone(),
        description: data.description.clone(),
        priority: data.priority,
        planned_from: data.planned_from,
        planned_duration: TimeDelta::days(data.planned_duration.into()),
        real_from: None,     // TODO
        real_duration: None, // TODO
    };

    control::db::add_task(t);

    HttpResponse::Ok().json(data)
}

#[utoipa::path]
#[delete("/delete_task_by_id/{task_id}")]
async fn delete_task_by_id(path: web::Path<i32>) -> impl Responder {
    let task_id = path.into_inner();
    let t = control::db::list_by_id(task_id);
    control::db::remove_by_id(task_id);
    HttpResponse::Ok().json(t)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(hello)
            .service(get_tasks)
            .service(get_task_by_id)
            .service(add_task)
            .service(delete_task_by_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
