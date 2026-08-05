use axum::Router;
use dotenvy::dotenv;
use rust_blog::route;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use rust_blog::{
    repository::postgres::user_repo::PgUserRepository, service::UserService, state::AppState,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 加载 .env
    dotenv().ok();

    // 2. 初始化 tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_blog=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 3. 读取环境变量
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env file");

    info!("连接 PostgreSQL 数据库...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    info!("PostgreSQL 数据库连接成功");

    // 4. 依赖注入 (DI)
    let user_repo = Arc::new(PgUserRepository::new(pool));
    // 传入 jwt_secret
    let user_service = UserService::new(user_repo, jwt_secret.clone());
    // 传入 jwt_secret
    let state = AppState::new(user_service, jwt_secret);

    // 5. 挂载路由
    let app = Router::new()
        .nest("/api/users", route::user_route::user_routes())
        .with_state(state);

    // 6. 启动 Axum Web
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("服务已启动在 http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
