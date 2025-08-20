use crate::error::AppError;
use crate::models::User;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::mem;
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
    /// 根据邮箱查找用户
    ///
    /// # 参数
    /// * `email` - 用户邮箱地址
    /// * `pool` - 数据库连接池
    ///
    /// # 返回
    /// * `Result<Option<Self>, AppError>` - 成功则返回可能存在的用户实例，失败则返回错误
    pub async fn find_by_email(email: &str, pool: &PgPool) -> Result<Option<Self>, AppError> {
        let user = sqlx::query_as(
            "SELECT id,ws_id, fullname, email, created_at FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;
        Ok(user)
    }

    /// 新建用户
    /// 创建新用户
    ///
    /// # 参数
    /// * `input` - 包含用户注册信息的CreateUser结构体
    /// * `pool` - 数据库连接池
    ///
    /// # 返回
    /// * `Result<Self, AppError>` - 成功则返回新创建的用户实例，失败则返回错误
    pub async fn create(input: CreateUser, pool: &PgPool) -> Result<Self, AppError> {
        let password_hash = hash_password(&input.password)?;
        let db_user = Self::find_by_email(&input.email, pool).await?;
        if db_user.is_some() {
            return Err(AppError::EmailAlreadyExists(input.email.clone()));
        }
        let user = sqlx::query_as(
            r#"
            INSERT INTO users (email, fullname, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, fullname, email, created_at
            "#,
        )
        .bind(&input.email)
        .bind(&input.fullname)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    ///
    /// 验证用户
    /// 验证用户登录信息
    ///
    /// # 参数
    /// * `input` - 包含用户登录信息的SigninUser结构体
    /// * `pool` - 数据库连接池
    ///
    /// # 返回
    /// * `Result<Option<Self>, AppError>` - 成功则返回可能存在的用户实例，失败则返回错误
    pub async fn verify(input: &SigninUser, pool: &PgPool) -> Result<Option<Self>, AppError> {
        let user: Option<User> = sqlx::query_as(
            "SELECT id, fullname, email, password_hash, created_at FROM users WHERE email = $1",
        )
        .bind(&input.email)
        .fetch_optional(pool)
        .await?;
        match user {
            Some(mut user) => {
                let password_hash = mem::take(&mut user.password_hash);
                let is_valid =
                    verify_password(&input.password, &password_hash.unwrap_or_default())?;
                if is_valid {
                    Ok(Some(user))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}

fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let argon2 = Argon2::default();
    let password_hash = PasswordHash::new(password_hash)?;

    // Verify password
    let is_valid = argon2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok();
    Ok(is_valid)
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
