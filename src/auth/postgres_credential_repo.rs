use crate::auth::ports::*;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

pub struct PostgresCredentialRepoImpl {
    pub pg_pool: Arc<PgPool>,
}

#[async_trait]
impl CredentialRepo for PostgresCredentialRepoImpl {
    async fn save_credential(&self, credential: &Credential) -> bool {
        sqlx::query(
            "insert into credentials (username, password) values ($1, crypt($2, gen_salt('bf')))",
        )
        .bind(&credential.username)
        .bind(&credential.password)
        .execute(&*self.pg_pool)
        .await
        .map(|done| done.rows_affected() > 0)
        .unwrap_or(false)
    }

    async fn is_credential_exists(&self, credential: &Credential) -> bool {
        let (found,): (bool,) = sqlx::query_as(
            "select true from credentials where username = $1 and password = crypt($2, password)",
        )
        .bind(&credential.username)
        .bind(&credential.password)
        .fetch_one(&*self.pg_pool)
        .await
        .unwrap_or((false,));

        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use std::sync::Arc;

    #[rocket::async_test]
    async fn test_save_and_check() {
        let pg_pool = PgPool::connect("postgresql://postgres:test@localhost:5431")
            .await
            .expect("unable to connect to DB");
        sqlx::query("drop database if exists test_credential_repo")
            .execute(&pg_pool)
            .await
            .expect("unable to drop");
        sqlx::query("create database test_credential_repo")
            .execute(&pg_pool)
            .await
            .unwrap();
        let pg_pool = crate::infrastructure::postgresql::configure_with_db_url(
            "postgresql://postgres:test@localhost:5431/test_credential_repo",
        )
        .await;

        let sut = PostgresCredentialRepoImpl {
            pg_pool: Arc::new(pg_pool),
        };

        let credential = Credential {
            username: "u".to_string(),
            password: "p".to_string(),
        };

        assert!(!sut.is_credential_exists(&credential).await);
        assert!(sut.save_credential(&credential).await);
        assert!(sut.is_credential_exists(&credential).await);
    }
}
