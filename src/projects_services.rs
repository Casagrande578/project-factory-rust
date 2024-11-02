use actix_web::{
    get, post,
    web::{Data, Json, Query},
    HttpResponse, Responder,
};

use crate::{
    model::{ProjectModel, Team},
    schema::{CreateProjectRequest, FilterOptions},
    AppState,
};
use serde_json::json;

#[post("/projects")]
async fn create_project(body: Json<CreateProjectRequest>, data: Data<AppState>) -> impl Responder {
    let mut tx = match data.db.begin().await {
        Ok(tx) => tx,
        Err(error) => {
            return HttpResponse::InternalServerError().json(json!({
                "status":"error",
                "message": format!("Failed to start transaction: {}",error)
            }));
        }
    };

    let team = match sqlx::query_as!(Team, "SELECT * FROM teams WHERE id = $1", body.team_id)
        .fetch_one(&mut tx)
        .await
    {
        Ok(team) => team,
        Err(error) => {
            return HttpResponse::InternalServerError().json(json!({
                "status":"error",
                "message": format!("{:?}",error)
            }));
        }
    };

    //insert project
    let project = match sqlx::query_as!(
        ProjectModel,
        "INSERT INTO projects (azure_id, name, description, url, template,team_id) VALUES ($1,$2,$3,$4,$5,$6) RETURNING *",
        body.azure_id,
        body.name,
        body.description,
        body.url,
        body.template,
        team.id
    ).fetch_one(&mut tx)
    .await{
        Ok(project)=> project,
        Err(error) =>{
            return HttpResponse::InternalServerError().json(json!({
                "status":"error",
                "message": format!("{:?}",error)
            }));
        }
    };

    if let Err(e) = tx.commit().await {
        return HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to commit transaction: {}", e)
        }));
    }

    let project_response = ProjectModel {
        id: project.id,
        azure_id: project.azure_id,
        name: project.name,
        description: project.description,
        url: project.url,
        template: project.template,
        team_id: Some(team.id),
    };

    HttpResponse::Ok().json(json!({"status":"success","project":project_response}))
}

#[get("/project")]
async fn get_all_projects(opts: Query<FilterOptions>, data: Data<AppState>) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match sqlx::query_as!(
        ProjectModel,
        "SELECT * FROM projects ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(projects) => {
            let projects_response = json!({
                "status":"success",
                "result": projects.len(),
                "projects": projects
            });
            return HttpResponse::Ok().json(projects_response);
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(json!({
                "status":"error",
                "message":format!("{:?}",error)
            }));
        }
    }
}
