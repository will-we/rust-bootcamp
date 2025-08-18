use crate::error::AppError;
use crate::models::User;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub fullname: String,
    pub email: String,
    pub workspace: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigninUser {
    pub email: String,
    pub password: String,
}

impl User {
    /// 查找用户
    pub async fn find_by_email(email: &str, pool: &PgPool) -> Result<Option<Self>, AppError> {
        let user = sqlx::query_as(
            "SELECT id,ws_id, fullname, email, created_at FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;
        Ok(user)
    }
}


#[cfg(test)]
mod tests {
    use crate::models::User;
    use anyhow::Result;
    use sqlx::PgPool;

    #[tokio::test]
    async fn create_and_verify_user_should_work() -> Result<()> {
        // 创建PgPool
        let pool = PgPool::connect("postgres://postgres:123456@localhost:5432/chat").await?;
        let email = "zhangsan".to_string();
        let user = User::find_by_email(&email, &pool).await?;
        println!("{:?}", user);
        assert!(Some(user).is_some());
        Ok(())
    }
}
