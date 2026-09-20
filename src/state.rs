use std::sync::Arc;

use axum::extract::FromRef;

use crate::service::UserService;

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService>,
    pub jwt_secret: String,
}

impl AppState {
    pub fn new(user_service: UserService, jwt_secret: String) -> Self {
        Self {
            user_service: Arc::new(user_service),
            jwt_secret,
        }
    }
}
impl FromRef<AppState> for String {
    fn from_ref(state: &AppState) -> Self {
        state.jwt_secret.clone()
    }
}
