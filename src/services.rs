use actix_web::{
    get, post,
    web::{scope, Data, Json, ServiceConfig},
    HttpResponse, Responder,
};

use serde_json::json;

use crate::{model::User, schema::CreateUserRequest, AppState};

#[get("/healthcheck")]
async fn health_check() -> impl Responder {
    const MESSAGE: &str = "Healthcheck api route up and running";

    HttpResponse::Ok().json(json!({
        "status":"success",
        "message": MESSAGE
    }))
}

#[post("/user")]
async fn create_user(body: Json<CreateUserRequest>, data: Data<AppState>) -> impl Responder {
    match sqlx::query_as!(
        User,
        "INSERT INTO users (name, azure_id, email) VALUES ($1,$2,$3)
             RETURNING *",
        body.name.as_deref(),
        body.azure_id.as_deref(),
        body.email.as_deref()
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(user) => {
            let note_response = json!({
                "status":"success",
                "user": json!({
                    "user": user
                })
            });
            return HttpResponse::Ok().json(note_response);
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(json!({
                "status":"error",
                "message": format!("{:?}",error)
            }));
        }
    }
}

pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/api").service(health_check).service(create_user);

    conf.service(scope);
}
