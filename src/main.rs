#![allow(unused)]
mod controllers;
mod database;
mod errors;
mod models;
mod routes;
mod utils;
mod validators;

use axum::{routing::get_service, Router};
use database::db;
use solana_sdk::signature::{Keypair, Signer};
use std::{net::SocketAddr, sync::Arc};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let pool = db::connect().await;
    let migrations = db::migrate(&pool).await;

    let app_state = Arc::new(db::AppState { db: pool.clone() });

    let app = Router::new()
        .merge(routes::route_user::user_routes(app_state.clone()))
        .merge(routes::route_auth::auth_routes(app_state.clone()))
        .fallback_service(static_routes());

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(5000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("Listening on {}", addr);
}

fn static_routes() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
