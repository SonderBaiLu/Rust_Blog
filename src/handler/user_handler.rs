use axum::{Json, extract::State};

use crate::{
    error::AppError,
    handler::auth::AuthUser,
    models::dto::user::{LoginReq, LoginResp, RegisterReq, UserResp},
    state::AppState,
};

// POST /api/users/register
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterReq>,
) -> Result<Json<UserResp>, AppError> {
    // 调用Serivce 层 注册逻辑
    let user_repo = state.user_service.register(payload).await?;
    Ok(Json(user_repo))
}

// POST /api/users/login
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginReq>,
) -> Result<Json<LoginResp>, AppError> {
    let login_resp = state.user_service.login(payload).await?;
    Ok(Json(login_resp))
}
// POST /api/users/get_me
pub async fn get_me(
    State(state): State<AppState>, // ✅ 注意：这里直接用 Arc<UserService>，去除 dyn
    authuser: AuthUser,
) -> Result<Json<UserResp>, AppError> {
    // 1. 调用 service 层根据 token 中解析出的 id 查询用户
    let user_resp = state.user_service.get_by_id(authuser.id).await?;

    // 2. 返回 JSON 响应
    Ok(Json(user_resp))
}
