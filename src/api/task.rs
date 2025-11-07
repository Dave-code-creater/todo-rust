use actix_web::{
    get,
    post,
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    http::{header::ContentType, StatusCode}
};
use serde::{
    Serialize,
    Deserialize
};

use derive_more::{Display};
#[derive(Deserialize, Serialize)]
pub struct TaskIdentifier {
    task_id: String,

}

#[get("/task/{task_id}")]
pub async fn get_tasks(task_id: Path<TaskIdentifier>, body: Json<Struct>) -> Json<String> {
    return Json(task_id.into_inner().task_id)
}