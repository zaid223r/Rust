mod config;
mod database;
mod handlers;
mod middleware;
mod models;

use axum::{
    middleware as axum_middleware,
    routing::{delete, get, post, put},
    Router,
};
use tower_http::cors::CorsLayer;
use tracing_subscriber::fmt::init;
use axum::Extension;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init();
    dotenvy::dotenv().ok();

    let config = config::Config::from_env()?;

    let db_pool = database::create_connection_pool(&config.database_url).await?;

    let app_state = db_pool.clone();
    let auth_state = config.jwt_secret.clone();

    let protected_routes = Router::new()
        .route("/api/auth/me", get(handlers::auth::get_current_user))
        .route("/api/posts", get(handlers::post::get_user_posts))
        .route("/api/posts", post(handlers::post::create_post))
        .route("/api/posts/{id}", get(handlers::post::get_post))
        .route("/api/posts/{id}", put(handlers::post::update_post))
        .route("/api/posts/{id}", delete(handlers::post::delete_post))
        .route_layer(axum_middleware::from_fn_with_state(
            auth_state.clone(),
            middleware::auth::auth_middleware,
        ));

    let app = Router::new()
        .route("/api/auth/register", post(handlers::auth::register))
        .route("/api/auth/login", post(handlers::auth::login))
        .merge(protected_routes)
        .layer(CorsLayer::permissive())
        .layer(Extension(auth_state.clone()))
        .with_state(app_state);

    let addr = format!("0.0.0.0:{}", config.server_port);
    println!("🚀 Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}