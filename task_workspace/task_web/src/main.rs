use std::sync::Mutex;

use actix_web::{App, HttpResponse, HttpServer, Responder, ResponseError, get, put, web};
use serde::{Deserialize, Serialize};
use task_library::task::TaskManager;
use utoipa::{OpenApi, openapi};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(hello, get_tasks, get_task_by_id, add_task),
    components(schemas(SimpleResponse))
)]
struct ApiDoc;

#[derive(serde::Serialize, utoipa::ToSchema)]
struct SimpleResponse {
    status: u16,
}

struct AppState {
    task_manager: TaskManager,
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
async fn get_tasks(state: web::Data<Mutex<AppState>>) -> impl Responder {
    let s = state.lock().unwrap();
    let tasks = s.task_manager.get_tasks();
    HttpResponse::Ok().json(tasks)
}

#[utoipa::path]
#[get("/get_task_by_id/{task_id}")]
async fn get_task_by_id(state: web::Data<Mutex<AppState>>, path: web::Path<i32>) -> impl Responder {
    let task_id = path.into_inner();
    HttpResponse::Ok().json(format!("{{task_id: {task_id}}}"))
}

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
struct AddTaskData {
    id: i32,
    name: String,
    Description: String,
}

#[utoipa::path]
#[put("/add_task")]
async fn add_task(
    state: web::Data<Mutex<AppState>>,
    data: web::Json<AddTaskData>,
) -> impl Responder {
    HttpResponse::Ok().json(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let openapi = ApiDoc::openapi();
    let app_state = web::Data::new(Mutex::new(AppState {
        task_manager: TaskManager::new(),
    }));

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(hello)
            .service(get_tasks)
            .service(get_task_by_id)
            .service(add_task)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
