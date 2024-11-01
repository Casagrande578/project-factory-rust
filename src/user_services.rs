use actix_web::{
    delete, get, patch, post,
    web::{scope, Data, Json, Path, Query, ServiceConfig},
    HttpResponse, Responder,
};

use serde_json::json;
use uuid::Uuid;

use sqlx::{postgres::PgArguments, Arguments, Postgres};

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

#[get("/users/{id}")]
async fn get_user_by_id(path: Path<Uuid>, data: Data<AppState>) -> impl Responder {
    let user_id = path.into_inner();

    match sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(user) => {
            let user = json!({
                "status":"success",
                "user":user
            });
            return HttpResponse::Ok().json(user);
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(json!({
                "status":"error",
                "message": format!("{:?}",error)
            }));
        }
    }
}

#[delete("/users/{id}")]
async fn delete_user(path: Path<Uuid>, data: Data<AppState>) -> impl Responder {
    let user_id = path.into_inner();

    match sqlx::query_as!(User, "DELETE FROM users WHERE id = $1", user_id)
        .execute(&data.db)
        .await
    {
        Ok(_) => return HttpResponse::NoContent().finish(),
        Err(error) => {
            return HttpResponse::InternalServerError().json(json!({
                "status":"error",
                "message": format!("{:?}",error)
            }));
        }
    }
}

#[patch("/users/{id}")]
async fn update_user_by_id(
    path: Path<Uuid>,
    body: Json<CreateUserRequest>,
    data: Data<AppState>,
) -> impl Responder {
    let user_id = path.into_inner();

    match sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(user) => {
            match sqlx::query_as!(
                User,
                "UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING *",
                body.name.to_owned().unwrap_or(user.name.expect("No info for user")),
                body.email.to_owned().unwrap_or(user.email.expect("No info for user")),
                user_id
            )
            .fetch_one(&data.db)
            .await
            {
                Ok(user) => {
                    let user_response = json!({
                        "status":"success",
                        "user": user
                    });
                    return HttpResponse::Ok().json(user_response);
                }
                Err(error) => {
                    return HttpResponse::InternalServerError().json(json!({
                        "status":"error",
                        "message": format!("{:?}",error)
                    }));
                }
            }
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(json!({
                "status":"error",
                "message": format!("{:?}",error)
            }));
        }
    }
}

