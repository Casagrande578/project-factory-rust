use actix_web::{
    get, post,
    web::{scope, Data, Json, Query, ServiceConfig},
    HttpResponse, Responder,
};

use serde_json::json;

use crate::{
    model::User,
    schema::{CreateUserRequest, FilterOptions},
    AppState,
};

#[get("/healthcheck")]
async fn health_check() -> impl Responder {
    const MESSAGE: &str = "Healthcheck api route up and running";

    HttpResponse::Ok().json(json!({
        "status":"success",
        "message": MESSAGE
    }))
}

#[post("/users")]
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

#[get("/users")]
async fn get_all_users(opts: Query<FilterOptions>, data: Data<AppState>) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match sqlx::query_as!(
        User,
        "SELECT * FROM users ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(users) => {
            let json_response = json!({
                "status":"success",
                "result": users.len(),
                "users": users
            });
            return HttpResponse::Ok().json(json_response);
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
    let scope = scope("/api").service(health_check).service(create_user).service(get_all_users);

    conf.service(scope);
}
