use axum::{
    Router,
    routing::{get, post},
};

use crate::{handler::user_handler, state::AppState};

/// 暴露用户模块的所有路由映射
pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(user_handler::register))
        .route("/login", post(user_handler::login))
        .route(
            "/me",
            get(user_handler::get_me).patch(user_handler::update_user_info),
        )
}
