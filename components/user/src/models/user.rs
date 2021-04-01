use anyhow::Result;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::MySqlPool;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    id: u64,
    name: Option<String>,
    age: Option<i32>,
}

impl User {
    pub fn new(name: Option<String>, age: Option<i32>) -> User {
        User { id: 0, name, age }
    }

    pub fn new_all(id: u64, name: Option<String>, age: Option<i32>) -> User {
        User { id, name, age }
    }

    pub async fn create(user: User, pool: &MySqlPool) -> Result<u64> {
        let mut tx = pool.begin().await?;
        let user_id = sqlx::query(" INSERT INTO t_user (name, age) VALUES (?, ?)")
            .bind(user.name)
            .bind(user.age)
            .execute(&mut tx)
            .await?
            .last_insert_id();

        tx.commit().await?;
        Ok(user_id)
    }

    pub async fn update(user: User, pool: &MySqlPool) -> Result<bool> {
        let mut tx = pool.begin().await?;
        let rows_affected = sqlx::query("UPDATE t_user SET name = ?, age = ? WHERE id = ?")
            .bind(&user.name)
            .bind(user.age)
            .bind(user.id)
            .execute(&mut tx)
            .await?
            .rows_affected();

        tx.commit().await?;
        Ok(rows_affected > 0)
    }

    pub async fn delete(id: u64, pool: &MySqlPool) -> Result<bool> {
        let deleted = sqlx::query("DELETE FROM t_user WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected();

        Ok(deleted > 0)
    }

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
                id: rec.id as u64,
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
    async fn test_crud() -> Result<()> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = MySqlPool::connect(&database_url).await?;

        //create
        let user_id =
            User::create(User::new(Some(String::from("nauu1")), Some(18)), &db_pool).await?;
        println!("user_id:{} ", user_id);

        //list
        let users = User::find_all(&db_pool).await?;
        for u in users.iter() {
            println!("{:?}", u);
        }

        //update
        let updated = User::update(
            User::new_all(user_id, Some(String::from("nauu111")), Some(28)),
            &db_pool,
        )
        .await?;
        println!("updated:{}", updated);

        //delete
        let deleted = User::delete(user_id, &db_pool).await?;
        println!("deleted:{}", deleted);
        Ok(())
    }

    #[tokio::test]
    async fn test_create() -> Result<()> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = MySqlPool::connect(&database_url).await?;

        let user_id =
            User::create(User::new(Some(String::from("nauu1")), Some(18)), &db_pool).await?;
        println!("user_id:{} ", user_id);
        Ok(())
    }

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

    #[tokio::test]
    async fn test_update() -> Result<()> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = MySqlPool::connect(&database_url).await?;

        let updated = User::update(
            User::new_all(1, Some(String::from("nauu111")), Some(28)),
            &db_pool,
        )
        .await?;
        println!("updated:{}", updated);
        Ok(())
    }

    #[tokio::test]
    async fn test_delete() -> Result<()> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = MySqlPool::connect(&database_url).await?;

        let deleted = User::delete(3, &db_pool).await?;
        println!("deleted:{}", deleted);
        Ok(())
    }
}
