use sqlx::MySqlPool;

pub struct Context {
    pub mysql_pool: MySqlPool,
}

// impl juniper::Context for Context {}
