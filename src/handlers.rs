use crate::entities::user;
use askama::Template;
use axum::response::{Html, IntoResponse};
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Template)]
#[template(path = "users.html")]
struct UsersTemplate {
    users: Vec<user::Model>,
}

pub async fn list_users(db: DatabaseConnection) -> impl IntoResponse {
    let users = user::Entity::find().all(&db).await.unwrap_or_default();

    Html(UsersTemplate { users }.render().unwrap())
}
