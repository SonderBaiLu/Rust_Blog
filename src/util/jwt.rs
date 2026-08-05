// FRF 7519
// sub (Subject): 用户唯一标识（通常为用户 ID 的 UUID 字符串）
// exp (Expiration Time): 令牌过期时间（Unix 时间戳）
// iat (Issued At): 令牌签发时间（Unix 时间戳）

use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

// 定义JWT 结构
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // 用户ID Uiid
    pub exp: usize,  // 过期时间 Unix时间戳
    pub iat: usize,  // 签发时间 Unix时间戳
}

/// 生成 JWT Token
///
/// # 参数
/// - `user_id`: 用户 UUID
/// - `secret`: JWT 密钥
/// - `exp_hours`: Token 有效期（小时）
pub fn generate_token(user_id: Uuid, secret: &str, exp_hours: i64) -> Result<String, AppError> {
    let now = Utc::now();
    let expire = now + Duration::hours(exp_hours); // 死亡时间
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expire.timestamp() as usize,
        iat: now.timestamp() as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("JWT 签发失败： {}", e)))
}
// 验证JWT Token 并解析 Claims
pub fn verify_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|token_data| token_data.claims)
    .map_err(|_| AppError::Unauthorized("无效或已过期的 Token".to_string()))
}
