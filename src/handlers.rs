use crate::AppError;
use askama::Template;
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use entities::user;
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Template)]
#[template(path = "users.html")]
struct UsersTemplate {
    users: Vec<user::Model>,
}

pub async fn list_users(
    State(db): State<DatabaseConnection>,
) -> Result<impl IntoResponse, AppError> {
    let users = user::Entity::find().all(&db).await.unwrap_or_default();

    Ok(Html(UsersTemplate { users }.render()?))
}
