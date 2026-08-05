use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{dto::user::RegisterReq, entity::user::User},
    repository::user::UserRepository,
};

pub struct PgUserRepository {
    pool: PgPool,
}
impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let user =
            sqlx::query_as::<_, User>("select * from users where id = $1 and deleted_at is null")
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;
        Ok(user)
    }
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            "select * from users where email = $1 and deleted_at is null",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }
    async fn create_user(&self, req: &RegisterReq, password_hash: &str) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
        insert into users (name, password, email) values ($1, $2, $3) returning *
        "#,
        )
        .bind(&req.name)
        .bind(password_hash)
        .bind(&req.email)
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }
}
