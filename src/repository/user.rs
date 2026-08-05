use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{dto::user::RegisterReq, entity::user::User},
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    // 根据 ID 查询用户（排除已软删除用户）
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
    // 根据 Email 查询用户（用于登录校验）
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    // 创建新用户
    async fn create_user(&self, req: &RegisterReq, password_hash: &str) -> Result<User, AppError>;
}
