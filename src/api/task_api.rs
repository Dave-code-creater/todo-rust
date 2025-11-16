use actix_web::{
    get,
    web::Path,
    web::Json,
    web
};
use serde::{
    Serialize,
    Deserialize
};

#[derive(Deserialize, Serialize)]
pub struct TaskIdentifier {
    task_id: String,

}

#[get("/task/{task_id}")]
pub async fn get_tasks(task_id: Path<TaskIdentifier>) -> Json<String> {
    return Json(task_id.into_inner().task_id)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_tasks);
}
