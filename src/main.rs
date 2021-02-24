mod auth;
mod infrastructure;

use auth::{ports::AuthService, rest_auth_controller::configure};
use sqlx::PgPool;
use std::sync::Arc;

#[rocket::main]
async fn main() -> Result<(), rocket::error::Error> {
    if let Err(e) = dotenv::dotenv() {
        print!("not applying .env: {:?}", e);
    }

    let pg_pool = Arc::new(infrastructure::postgresql::configure().await);
    let redis_client = Arc::new(infrastructure::redis::configure().await);

    rocket::ignite()
        .manage(Box::new(configure_auth(redis_client, pg_pool)) as Box<dyn AuthService>)
        .mount("/", configure())
        .launch()
        .await
}

fn configure_auth(redis_client: Arc<redis::Client>, pg_pool: Arc<PgPool>) -> impl AuthService {
    use crate::auth::{
        auth_service_impl::AuthServiceImpl, postgres_credential_repo::PostgresCredentialRepoImpl,
        redis_token_repo::RedisTokenRepoImpl,
    };
    AuthServiceImpl {
        credential_repo: PostgresCredentialRepoImpl { pg_pool },
        token_repo: RedisTokenRepoImpl { redis_client },
    }
}
