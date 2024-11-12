use actix_web::{
    get, post,
    web::{Data, Json, Query},
    HttpResponse, Responder,
};
use chrono::offset;

use log::{error, info};

use crate::{
    model::{ProjectModel, Team, User, WorkItem},
    schema::{CreateProjectRequest, CreateWorkItemRequest, FilterOptions},
    AppState,
};
use serde_json::json;

#[post("/workitems")]
async fn create_workitem(
    body: Json<CreateWorkItemRequest>,
    data: Data<AppState>,
) -> impl Responder {
    //begin transaction
    println!("{:?}", body.azure_id);
    let mut tx = match data.db.begin().await {
        Ok(tx) => tx,
        Err(error) => {
            return HttpResponse::InternalServerError().json(json!({"status":"error", "message": format!("Failed to begin transaction: {}",error)}));
        }
    };
    //find relations
    let project = match sqlx::query_as!(
        ProjectModel,
        "SELECT * FROM projects WHERE name = $1",
        body.project
    )
    .fetch_one(&mut tx)
    .await
    {
        Ok(project) => project,
        Err(error) => {
            error!("Failed here on fiding project");
            return HttpResponse::InternalServerError()
                .json(json!({"status":"error", "message": format!("{:?}",error)}));
        }
    };

    let assigned_user = match sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE azure_id = $1",
        body.assigned_to_id
    )
    .fetch_one(&mut tx)
    .await
    {
        Ok(user) => user,
        Err(error) => {
            error!("failed to find user: {:?}", body.assigned_to_id);
            return HttpResponse::InternalServerError()
                .json(json!({"status":"error", "message": format!("{:?}",error)}));
        }
    };

    let created_by_user = match sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        body.created_by_id
    )
    .fetch_one(&mut tx)
    .await
    {
        Ok(user) => user,
        Err(error) => {
            error!("Failed to find creation user: {:?}", body.created_by_id);
            return HttpResponse::InternalServerError()
                .json(json!({"status":"error", "message": format!("{:?}",error)}));
        }
    };

    let parent_workitem = sqlx::query_as!(
        WorkItem,
        "SELECT * FROM work_items WHERE azure_id = $1",
        body.parent_id
    )
    .fetch_optional(&mut tx)
    .await
    .ok()
    .flatten();
    match parent_workitem {
        Some(parent) => {
            let workitem = match sqlx::query_as!(WorkItem,"INSERT INTO work_items (azure_id, title, w_type, state, project,assigned_to_id,created_by_id,priority, 
            severity, description, area_path, iteration_path, parent_id, tags, url) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15) RETURNING *",
            body.azure_id,
            body.title,
            body.w_type,
            body.state,
            project.id,
            assigned_user.id,
            created_by_user.id,
            body.priority,
            body.severity,
            body.description,
            body.area_path,
            body.iteration_path,
            parent.id,
            body.tags.as_deref(),
            body.url
        ).fetch_one(&mut tx)
        .await{
            Ok(wi) => wi,
            Err(error)=>{
                return HttpResponse::InternalServerError().json(json!({
                    "status":"error",
                    "message":format!("{:?}",error)
                }));
            }
        };
            if let Err(e) = tx.commit().await {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": format!("Failed to commit transaction: {}", e)
                }));
            }

            HttpResponse::Created().json(json!({"status":"success", "data":workitem}))
        }
        None => {
            let workitem = match sqlx::query_as!(WorkItem,"INSERT INTO work_items (azure_id, title, w_type, state, project,assigned_to_id,created_by_id,priority, 
            severity, description, area_path, iteration_path, tags, url) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14) RETURNING *",
            body.azure_id,
            body.title,
            body.w_type,
            body.state,
            project.id,
            assigned_user.id,
            created_by_user.id,
            body.priority,
            body.severity,
            body.description,
            body.area_path,
            body.iteration_path,
            body.tags.as_deref(),
            body.url
        ).fetch_one(&mut tx)
        .await{
            Ok(wi) => wi,
            Err(error)=>{
                return HttpResponse::InternalServerError().json(json!({
                    "status":"error",
                    "message":format!("{:?}",error)
                }));
            }
        };
            if let Err(e) = tx.commit().await {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": format!("Failed to commit transaction: {}", e)
                }));
            }

            HttpResponse::Created().json(json!({"status":"success", "data":workitem}))
        }
    }
}

#[get("/workitem")]
async fn get_all_workitem(opts: Query<FilterOptions>, data: Data<AppState>) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match sqlx::query_as!(
        WorkItem,
        "SELECT * FROM work_items ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(wi) => {
            let json_response = json!({
                "status":"success",
                "result":wi.len(),
                "workitems":wi
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
