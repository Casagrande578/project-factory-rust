use ::serde::{Deserialize, Serialize};
use sqlx::types::Uuid as SqlxUuid;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeamRequest {
    pub azure_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub azure_id: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub team_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectRequest {
    pub azure_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub template: Option<String>,
    pub begin_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub team_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWorkItemRequest {
    pub azure_id: Option<String>,
    pub title: String,
    pub type_: String,
    pub state: String,
    pub project: Uuid,
    pub assigned_to_id: Option<Uuid>,
    pub created_by_id: Uuid,
    pub priority: Option<i32>,
    pub severity: Option<String>,
    pub description: Option<String>,
    pub area_path: Option<String>,
    pub iteration_path: Option<String>,
    pub parent_id: Option<Uuid>,
    pub tags: Option<Vec<String>>,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNotificationRequest {
    pub subject: Option<String>,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub message: Option<String>,
}
