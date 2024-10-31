use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Team {
    pub id: Uuid,
    pub azure_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    // Relationships
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<Project>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub azure_id: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<Uuid>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: Uuid,
    pub azure_id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub template: Option<String>,
    pub begin_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub team_id: Option<Uuid>,
    // Relationships
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<Team>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkItem {
    pub id: Uuid,
    pub azure_id: Option<String>,
    pub title: String,
    pub type_: String, // Using type_ because 'type' is a reserved word
    pub state: String,
    pub project: Uuid,
    pub assigned_to_id: Option<Uuid>,
    pub created_by_id: Uuid,
    pub created_date: DateTime<Utc>,
    pub changed_date: Option<DateTime<Utc>>,
    pub priority: Option<i32>,
    pub severity: Option<String>,
    pub description: Option<String>,
    pub area_path: Option<String>,
    pub iteration_path: Option<String>,
    pub parent_id: Option<Uuid>,
    pub tags: Option<Vec<String>>,
    pub url: String,
    // Relationships
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<User>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Notification {
    pub id: i32,
    pub subject: Option<String>,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub message: Option<String>,
    pub creation_time: DateTime<Utc>,
    pub closed: bool,
    // Relationships
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<User>,
}
