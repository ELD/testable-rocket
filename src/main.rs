mod auth;
mod infrastructure;

use auth::{
    auth_service_impl::AuthServiceImpl, ports::AuthService,
    postgres_credential_repo::PostgresCredentialRepoImpl, redis_token_repo::RedisTokenRepoImpl,
    rest_auth_controller::configure,
};
use rocket::fairing::AdHoc;
use sqlx::PgPool;
use std::sync::Arc;

#[rocket::main]
async fn main() -> Result<(), rocket::error::Error> {
    if let Err(e) = dotenv::dotenv() {
        print!("not applying .env: {:?}", e);
    }

    rocket::build()
        .attach(AdHoc::on_ignite("Auth", |rocket| {
            Box::pin(async move {
                let pg_pool = Arc::new(infrastructure::postgresql::configure().await);
                let redis_client = Arc::new(infrastructure::redis::configure().await);

                rocket
                    .manage(
                        Box::new(configure_auth(redis_client.clone(), pg_pool.clone()))
                            as Box<dyn AuthService>,
                    )
                    .manage(redis_client)
                    .manage(pg_pool)
            })
        }))
        .mount("/", configure())
        .launch()
        .await
}

fn configure_auth(redis_client: Arc<redis::Client>, pg_pool: Arc<PgPool>) -> impl AuthService {
    AuthServiceImpl {
        credential_repo: PostgresCredentialRepoImpl { pg_pool },
        token_repo: RedisTokenRepoImpl { redis_client },
    }
}
