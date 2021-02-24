use sqlx::postgres::PgPool;
use std::env;

pub async fn configure() -> PgPool {
    let db_url = env::var("DB_URL").expect("DB_URL env var needs to be set");
    eprintln!("{:?}", db_url);
    configure_with_db_url(&db_url).await
}

pub async fn configure_with_db_url(db_url: &str) -> PgPool {
    let pool = PgPool::connect(&db_url)
        .await
        .expect("unable to connect to Postgresql");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("unable to migrate database");

    pool
}
