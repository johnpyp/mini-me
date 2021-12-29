use sqlx::{postgres, Pool, Postgres};

pub type DbConn = Pool<Postgres>;
pub async fn establish_connection(database_url: &str) -> DbConn {
    let pool = postgres::PgPool::connect(database_url)
        .await
        .expect("Postgress init error");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migration run issue");
    pool
}
