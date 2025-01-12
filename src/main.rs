mod handlers;
mod models;
mod errors;

use std::time::Duration;

use axum::{Router, routing::{get, post}, http::Method};
use crate::handlers::{ get_users, add_user, get_user, get_items, add_item, get_item_types };
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use tower_http::{ trace::{ self, TraceLayer }, cors::{ CorsLayer, Any } };

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt()
    .with_target(false)
    .compact()
    .init();

    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    .allow_origin(Any)
    .allow_headers(Any);

    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
    .max_connections(10)
    .acquire_timeout(Duration::from_secs(20))
    .connect(&db_url)
    .await
    .unwrap();
    
    let app = Router::new()
    .route("/api/users", get(get_users))
    .route("/api/adduser", post(add_user))
    .route("/api/getuser", post(get_user))
    .route("/api/getitems", get(get_items))
    .route("/api/additem", post(add_item))
    .route("/api/getitemtypes", get(get_item_types))
    .with_state(pool)
    .layer(
        TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new()
            .level(tracing::Level::INFO))
        .on_response(trace::DefaultOnResponse::new()
            .level(tracing::Level::INFO)),
    ).layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
