use anyhow::Result;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::MySqlPool;
use std::error::Error;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    id: i32,
    name: Option<String>,
    age: Option<i32>,
}

impl User {
    pub async fn find_all(pool: &MySqlPool) -> Result<Vec<User>> {
        let mut users = vec![];
        let recs = sqlx::query!(
            r#"
                SELECT id, name, age
                    FROM t_user
                ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await?;

        for rec in recs {
            users.push(User {
                id: rec.id,
                name: rec.name,
                age: rec.age,
            });
        }

        Ok(users)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_find_all() -> Result<()> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = MySqlPool::connect(&database_url).await?;

        let users = User::find_all(&db_pool).await?;
        for u in users.iter() {
            println!("{:?}", u);
        }
        Ok(())
    }
}
