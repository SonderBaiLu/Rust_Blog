use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::models::entity::user::User;

// 用户注册请求
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterReq {
    #[validate(
        length(min = 2, max = 20, message = "用户名长度须在2-20个字符之间"),
        custom(function = "crate::util::validator::validate_username")
    )]
    pub name: String,
    #[validate(length(min = 6, message = "密码长度至少需要 6 位"))]
    pub password: String, //接受前端的明文密码
    #[validate(
        email(message = "邮箱格式不正确"),
        custom(function = "crate::util::validator::validate_email_format")
    )]
    pub email: Option<String>,
}
// 用户登录请求
#[derive(Debug, Deserialize)]
pub struct LoginReq {
    pub email: String,
    pub password: String,
    // TODO: 后期添加 邮箱验证码登录 手机号验证码登录
}
// 用户信息响应
#[derive(Debug, Serialize)]
pub struct UserResp {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub phone: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}
#[derive(Debug, Serialize)]
pub struct LoginResp {
    pub token: String,
    pub user: UserResp,
}
impl From<User> for UserResp {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            email_verified: user.email_verified,
            phone: user.phone,
            is_active: user.is_active,
            created_at: user.created_at,
        }
    }
}
