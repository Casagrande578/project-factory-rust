use actix_web::{
    get, post,
    web::{Data, Json, Query},
    HttpResponse, Responder,
};

use log::{info,error};
use serde_json::json;

use crate::{
    model::{Team, TeamResponse, User},
    schema::{CreateTeamRequest, FilterOptions},
    AppState,
};

#[post("/teams")]
async fn create_team(body: Json<CreateTeamRequest>, data: Data<AppState>) -> impl Responder {
    //start transaction
    let mut tx = match data.db.begin().await {
        Ok(tx) => tx,
        Err(error) => {
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("Failed to start transaction: {}", error)
            }));
        }
    };
    info!("{:?}", &body.user_ids[..]);
    //find team members
    let users = match sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE azure_id = ANY($1::varchar[])",
        &body.user_ids
    )
    .fetch_all(&mut tx)
    .await
    {
        Ok(users) => {
            if users.len() != body.user_ids.len() {
                return HttpResponse::BadRequest().json(json!({
                    "status": "error",
                    "message": "One or more users not found"
                }));
            }
            users
        }
        Err(error) => {
            error!("{}", error);
            return HttpResponse::InternalServerError().json(json!({
                "status":"error",
                "message": format!("{:?}",error)
            }));
        }
    };

    //insert team
    let team = match sqlx::query_as!(
        Team,
        "INSERT INTO teams (name, description, azure_id) VALUES ($1, $2, $3) RETURNING *",
        body.name,
        body.description,
        body.azure_id
    )
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

    for user in &users {
        if let Err(e) = sqlx::query!(
            "INSERT INTO team_users (team_id, user_id) VALUES ($1,$2)",
            team.id,
            user.id
        )
        .execute(&mut tx)
        .await
        {
            let _ = tx.rollback().await;
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message":format!("Failed to associate users: {}",e)
            }));
        }
    }

    if let Err(e) = tx.commit().await {
        return HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to commit transaction: {}", e)
        }));
    }

    let response = TeamResponse {
        id: team.id,
        name: team.name,
        azure_id: team.azure_id,
        description: team.description,
        users: Some(users),
    };

    HttpResponse::Created().json(json!({"status":"success", "data":response}))
}

#[get("/teams")]
async fn get_all_teams(opts: Query<FilterOptions>, data: Data<AppState>) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match sqlx::query_as!(
        Team,
        "SELECT * FROM teams ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(teams) => {
            let json_response = json!({
                "status":"success",
                "result": teams.len(),
                "users": teams
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
