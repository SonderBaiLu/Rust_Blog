use uuid::Uuid;

use crate::error::AppError;
use crate::models::dto::user::{LoginReq, LoginResp, RegisterReq, UserResp};
use crate::repository::user::UserRepository;
use crate::util::jwt::generate_token;
use crate::util::password;
use std::sync::Arc;
pub struct UserService {
    // 依赖抽象接口，而不是具体数据库
    user_repo: Arc<dyn UserRepository>,
    // 签发Token
    jwt_secret: String,
}
impl UserService {
    pub fn new(user_repo: Arc<dyn UserRepository>, jwt_secret: String) -> Self {
        Self {
            user_repo,
            jwt_secret,
        }
    }
    // 用户注册
    pub async fn register(&self, req: RegisterReq) -> Result<UserResp, AppError> {
        // A. 校验邮箱
        if let Some(ref email) = req.email {
            if self.user_repo.find_by_email(email).await?.is_some() {
                return Err(AppError::BadRequest("该邮箱已被注册".to_string()));
            }
        }
        // B. 加密密码 (调用 util)
        let password_hash = password::hash_password(&req.password)?;

        // C. 写入数据库
        let user = self.user_repo.create_user(&req, &password_hash).await?;

        // D. 返回安全 DTO
        Ok(UserResp::from(user)) // 提示：给 UserResp 实现 From<User> trait 还能更简洁！
    }
    /// 2. 用户登录
    pub async fn login(&self, req: LoginReq) -> Result<LoginResp, AppError> {
        // A. 查询用户
        let user = self
            .user_repo
            .find_by_email(&req.email)
            .await?
            .ok_or_else(|| AppError::Unauthorized("账号或密码错误".to_string()))?;

        // B. 校验状态
        if !user.is_active {
            return Err(AppError::Forbidden("账号已被停用".to_string()));
        }

        // C. 校验密码 (调用 util)
        password::verify_password(&req.password, &user.password_hash)?;
        let token = generate_token(user.id, &self.jwt_secret, 24)?;
        Ok(LoginResp {
            token,
            user: UserResp::from(user),
        })
    }
    // 根据ID获取用户信息
    pub async fn get_by_id(&self, user_id: Uuid) -> Result<UserResp, AppError> {
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;
        Ok(UserResp::from(user))
    }
}
