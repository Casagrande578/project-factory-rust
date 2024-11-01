use crate::{team_services, user_services};
use actix_web::web::{scope, ServiceConfig};

pub fn configure_routes(conf: &mut ServiceConfig) {
    let scope = scope("/api")
        .service(user_services::health_check)
        .service(user_services::create_user)
        .service(user_services::get_all_users)
        .service(user_services::get_user_by_id)
        .service(user_services::delete_user)
        .service(user_services::update_user_by_id)
        .service(team_services::create_team)
        .service(team_services::get_all_teams);

    conf.service(scope);
}