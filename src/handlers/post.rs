use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::middleware::auth::CurrentUser;
use crate::models::{Post, CreatePostRequest, UpdatePostRequest};

pub async fn create_post(
    current_user: axum::Extension<CurrentUser>,
    State(db): State<PgPool>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<Post>, StatusCode> {
    let post = sqlx::query_as::<_, Post>(
        r#"
        INSERT INTO posts (title, content, user_id)
        VALUES ($1, $2, $3)
        RETURNING id, title, content, user_id, created_at, updated_at
        "#,
    )
        .bind(&payload.title)
        .bind(&payload.content)
        .bind(current_user.user_id)
        .fetch_one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(post))
}

pub async fn get_user_posts(
    current_user: axum::Extension<CurrentUser>,
    State(db): State<PgPool>,
) -> Result<Json<Vec<Post>>, StatusCode> {
    let posts = sqlx::query_as::<_, Post>(
        "SELECT id, title, content, user_id, created_at, updated_at FROM posts WHERE user_id = $1 ORDER BY created_at DESC"
    )
        .bind(current_user.user_id)
        .fetch_all(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(posts))
}

pub async fn get_post(
    current_user: axum::Extension<CurrentUser>,
    State(db): State<PgPool>,
    Path(post_id): Path<Uuid>,
) -> Result<Json<Post>, StatusCode> {
    let post = sqlx::query_as::<_, Post>(
        "SELECT id, title, content, user_id, created_at, updated_at FROM posts WHERE id = $1 AND user_id = $2"
    )
        .bind(post_id)
        .bind(current_user.user_id)
        .fetch_optional(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(post))
}

pub async fn update_post(
    current_user: axum::Extension<CurrentUser>,
    State(db): State<PgPool>,
    Path(post_id): Path<Uuid>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<Json<Post>, StatusCode> {
    let existing_post = sqlx::query_as::<_, Post>(
        "SELECT id, title, content, user_id, created_at, updated_at FROM posts WHERE id = $1 AND user_id = $2"
    )
        .bind(post_id)
        .bind(current_user.user_id)
        .fetch_optional(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let new_title = payload.title.unwrap_or(existing_post.title);
    let new_content = payload.content.unwrap_or(existing_post.content);

    let updated_post = sqlx::query_as::<_, Post>(
        r#"
        UPDATE posts
        SET title = $1, content = $2, updated_at = NOW()
        WHERE id = $3 AND user_id = $4
        RETURNING id, title, content, user_id, created_at, updated_at
        "#,
    )
        .bind(&new_title)
        .bind(&new_content)
        .bind(post_id)
        .bind(current_user.user_id)
        .fetch_one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(updated_post))
}

pub async fn delete_post(
    current_user: axum::Extension<CurrentUser>,
    State(db): State<PgPool>,
    Path(post_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query(
        "DELETE FROM posts WHERE id = $1 AND user_id = $2"
    )
        .bind(post_id)
        .bind(current_user.user_id)
        .execute(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}