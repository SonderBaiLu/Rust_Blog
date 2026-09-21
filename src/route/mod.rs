mod user_route;

use axum::{
    Router,
    http::{Method, HeaderValue, header},
};
use tower_http::cors::CorsLayer;
use crate::state::AppState;

pub fn create_app(state: AppState) -> Router {
    // 构造 CORS 策略
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(false);
    Router::new()
        .nest("/api/v1/users", user_route::user_routes())
        .layer(cors)
        .with_state(state)
}