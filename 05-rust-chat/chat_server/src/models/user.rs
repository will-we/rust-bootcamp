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
    pub ws_id: i32,
    pub full_name: String,
    pub email: String,
    pub workspace: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignInUser {
    pub ws_id: i32,
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
            INSERT INTO users (ws_id, email, fullname, password_hash)
            VALUES ($1, $2, $3, $4)
            RETURNING id, ws_id, fullname, email, created_at
            "#,
        )
        .bind(&input.ws_id)
        .bind(&input.email)
        .bind(&input.full_name)
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
    /// * `input` - 包含用户登录信息的 SignInUser结构体
    /// * `pool` - 数据库连接池
    ///
    /// # 返回
    /// * `Result<Option<Self>, AppError>` - 成功则返回可能存在的用户实例，失败则返回错误
    pub async fn verify(input: &SignInUser, pool: &PgPool) -> Result<Option<Self>, AppError> {
        let user: Option<User> = sqlx::query_as(
            "SELECT ws_id, id, fullname, email, password_hash, created_at FROM users WHERE email = $1 and ws_id = $2",
        )
        .bind(&input.email)
        .bind(&input.ws_id)
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

    /// 删除用户
    /// 根据用户ID删除用户
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    /// * `pool` - 数据库连接池
    ///
    /// # 返回
    /// * `Result<bool, AppError>` - 成功则返回是否删除成功，失败则返回错误
    pub async fn delete(user_id: i64, pool: &PgPool) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// 根据邮箱删除用户
    /// 根据邮箱地址删除用户
    ///
    /// # 参数
    /// * `email` - 用户邮箱地址
    /// * `pool` - 数据库连接池
    ///
    /// # 返回
    /// * `Result<bool, AppError>` - 成功则返回是否删除成功，失败则返回错误
    pub async fn delete_by_email(email: &str, pool: &PgPool) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM users WHERE email = $1")
            .bind(email)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
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
    use super::*;
    use crate::models::User;
    use anyhow::Result;
    use sqlx::PgPool;

    // 辅助函数：创建测试数据库连接池
    async fn create_test_pool() -> Result<PgPool> {
        // 从.env文件加载环境变量
        dotenvy::dotenv().ok();
        // 获取数据库连接URL
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        // 创建数据库连接池
        let pool = PgPool::connect(&database_url).await?;
        Ok(pool)
    }

    #[tokio::test]
    async fn test_find_by_email_not_found() -> Result<()> {
        let pool = create_test_pool().await?;
        let email = "nonexistent@example.com".to_string();
        let user = User::find_by_email(&email, &pool).await?;
        println!("查找到的用户信息: {:?}", user);
        assert!(user.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_email_found() -> Result<()> {
        let pool = create_test_pool().await?;

        let email = "zhangsan@test.com".to_string();
        // 先创建一个用户
        let create_user = CreateUser {
            ws_id: 1,
            full_name: "Zhang San".to_string(),
            email: email.clone(),
            workspace: "test_workspace".to_string(),
            password: "password123".to_string(),
        };
        User::create(create_user, &pool).await?;

        // 然后查找这个用户
        let user = User::find_by_email(&email, &pool).await?;
        println!("查找到的用户信息: {:?}", user);
        assert!(Some(user).is_some());
        // 删除该用户
        let r = User::delete_by_email(&email, &pool).await?;
        println!("删除用户:{}", &email);
        assert_eq!(true, r);
        Ok(())
    }

    #[tokio::test]
    async fn test_verify_user_success() -> Result<()> {
        let pool = create_test_pool().await?;

        // 创建用户
        let create_user = CreateUser {
            ws_id: 1,
            full_name: "Verify User".to_string(),
            email: "verify@example.com".to_string(),
            workspace: "test_workspace".to_string(),
            password: "correct_password".to_string(),
        };
        User::create(create_user, &pool).await?;

        // 验证正确密码
        let signin_user = SignInUser {
            ws_id: 1,
            email: "verify@example.com".to_string(),
            password: "correct_password".to_string(),
        };

        let verified_user = User::verify(&signin_user, &pool).await?;
        assert!(&verified_user.is_some());

        // 提取用户对象（避免多次 unwrap）
        let user = verified_user.unwrap();
        assert_eq!(user.email, "verify@example.com");

        // 修复点：添加 await 和错误处理
        User::delete(user.id, &pool).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_verify_nonexistent_user() -> Result<()> {
        let pool = create_test_pool().await?;

        // 尝试验证不存在的用户
        let signin_user = SignInUser {
            ws_id: 1,
            email: "nonexistent_verify@example.com".to_string(),
            password: "any_password".to_string(),
        };

        let verified_user = User::verify(&signin_user, &pool).await?;
        assert!(verified_user.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_password_hashing() -> Result<()> {
        let password = "test_password";
        let hashed = hash_password(password)?;

        // 验证哈希不等于原始密码
        assert_ne!(hashed, password);

        // 验证可以正确验证密码
        let is_valid = verify_password(password, &hashed)?;
        assert!(is_valid);

        // 验证错误密码不能通过验证
        let is_invalid = verify_password("wrong_password", &hashed)?;
        assert!(!is_invalid);

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_user_by_id() -> Result<()> {
        let pool = create_test_pool().await?;

        // 先创建一个用户
        let create_user = CreateUser {
            ws_id: 1,
            full_name: "Delete Test User".to_string(),
            email: "delete_test@example.com".to_string(),
            workspace: "test_workspace".to_string(),
            password: "password123".to_string(),
        };
        let user = User::create(create_user, &pool).await?;

        // 验证用户存在
        let found_user = User::find_by_email("delete_test@example.com", &pool).await?;
        assert!(found_user.is_some());

        // 删除用户
        let deleted = User::delete(user.id, &pool).await?;
        assert!(deleted);

        // 验证用户已被删除
        let not_found_user = User::find_by_email("delete_test@example.com", &pool).await?;
        assert!(not_found_user.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_user_by_email() -> Result<()> {
        let pool = create_test_pool().await?;

        // 先创建一个用户
        let create_user = CreateUser {
            ws_id: 1,
            full_name: "Delete By Email User".to_string(),
            email: "delete_by_email@example.com".to_string(),
            workspace: "test_workspace".to_string(),
            password: "password123".to_string(),
        };
        User::create(create_user, &pool).await?;

        // 验证用户存在
        let found_user = User::find_by_email("delete_by_email@example.com", &pool).await?;
        assert!(found_user.is_some());

        // 通过邮箱删除用户
        let deleted = User::delete_by_email("delete_by_email@example.com", &pool).await?;
        assert!(deleted);

        // 验证用户已被删除
        let not_found_user = User::find_by_email("delete_by_email@example.com", &pool).await?;
        assert!(not_found_user.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_nonexistent_user_by_id() -> Result<()> {
        let pool = create_test_pool().await?;

        // 尝试删除不存在的用户ID
        let deleted = User::delete(99999, &pool).await?;
        assert!(!deleted);

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_nonexistent_user_by_email() -> Result<()> {
        let pool = create_test_pool().await?;

        // 尝试删除不存在的用户邮箱
        let deleted = User::delete_by_email("nonexistent@example.com", &pool).await?;
        assert!(!deleted);

        Ok(())
    }
}
