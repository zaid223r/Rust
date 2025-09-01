use axum::{extract::State, http::StatusCode, response::Json, Extension};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::PgPool;
use tracing::{error, info, debug};

use crate::middleware::auth::{create_jwt_token, CurrentUser};
use crate::models::{AuthResponse, CreateUserRequest, LoginRequest, User, UserResponse};

pub async fn register(
    State(db): State<PgPool>,
    Extension(jwt_secret): Extension<String>,
    Json(body): Json<CreateUserRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    info!("Registration attempt for email: {}", body.email);
    
    let password_hash = hash(&body.password, DEFAULT_COST).map_err(|e| {
        error!("Failed to hash password: {:?}", e);
        StatusCode::UNAUTHORIZED
    })?;

    debug!("Password hashed successfully");

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, password_hash, name)
        VALUES ($1, $2, $3)
        RETURNING *
      "#,
    )
    .bind(&body.email)
    .bind(&password_hash)
    .bind(&body.name)
    .fetch_one(&db)
    .await
    .map_err(|e| {
        error!("Database error during user creation: {:?}", e);
        match e {
            sqlx::Error::Database(db_err) if db_err.constraint().is_some() => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    })?;

    info!("User created successfully with ID: {}", user.id);

    let token = create_jwt_token(user.id, user.email.clone(), &jwt_secret)
        .map_err(|e| {
            error!("Failed to create JWT token: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    debug!("JWT token created successfully");

    let res = AuthResponse {
        token,
        user: user.into(),
    };

    info!("Registration successful for user: {}", body.email);
    Ok(Json(res))
}

pub async fn login(
    State(db): State<PgPool>,
    Extension(jwt_secret): Extension<String>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&body.email)
        .fetch_optional(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let password_valid =
        verify(&body.password, &user.password_hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

    if ! password_valid? {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = create_jwt_token(user.id, user.email.clone(), &jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = AuthResponse {
        token,
        user: user.into(),
    };

    Ok(Json(response))
}

pub async fn get_current_user(
    current_user: axum::Extension<CurrentUser>,
    State(db): State<PgPool>,
) -> Result<Json<UserResponse>, StatusCode> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1 AND email = $2"
    )
        .bind(current_user.user_id)
        .bind(&current_user.email)
        .fetch_one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user.into()))
}