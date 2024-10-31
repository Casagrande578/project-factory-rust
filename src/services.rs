use actix_web::{
    get,
    web::{scope, ServiceConfig},
    HttpResponse, Responder,
};

use serde_json::json;

#[get("/healthcheck")]
async fn health_check() -> impl Responder {
    const MESSAGE: &str = "Healthcheck api route up and running";

    HttpResponse::Ok().json(json!({
        "status":"success",
        "message": MESSAGE
    }))
}

pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/api").service(health_check);

    conf.service(scope);
}
