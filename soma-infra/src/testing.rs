//! Throwaway-database harness for integration tests.
//!
//! Each [`TestDb`] creates a uniquely-named Postgres database, hands back a pool
//! to it, and drops the database when the handle goes out of scope — so tests
//! are fully isolated and self-cleaning, even on panic.

use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

/// An isolated, auto-dropped Postgres database for one test.
pub struct TestDb {
    /// A pool connected to the freshly-created database.
    pub pool: PgPool,
    name: String,
    admin_url: String,
}

impl TestDb {
    /// Create an isolated database. `base_url` points at any reachable database
    /// on the target server; the maintenance `postgres` database is used to
    /// `CREATE`/`DROP` the throwaway one.
    pub async fn create(base_url: &str) -> Result<TestDb, sqlx::Error> {
        let admin_url = swap_database(base_url, "postgres");
        let name = format!("soma_test_{}", Uuid::new_v4().simple());

        let mut admin = PgConnection::connect(&admin_url).await?;
        admin
            .execute(format!(r#"CREATE DATABASE "{name}""#).as_str())
            .await?;
        admin.close().await?;

        let test_url = swap_database(base_url, &name);
        let pool = PgPool::connect(&test_url).await?;
        Ok(TestDb {
            pool,
            name,
            admin_url,
        })
    }

    /// Create an isolated database using `TEST_DATABASE_URL`.
    pub async fn create_from_env() -> Result<TestDb, sqlx::Error> {
        let base = std::env::var("TEST_DATABASE_URL")
            .expect("TEST_DATABASE_URL must be set for TestDb::create_from_env");
        Self::create(&base).await
    }
}

impl Drop for TestDb {
    fn drop(&mut self) {
        let admin_url = self.admin_url.clone();
        let name = self.name.clone();
        // Drop on a dedicated thread with a fresh runtime: a tokio test can't
        // block_on its own runtime from within Drop.
        let _ = std::thread::spawn(move || {
            if let Ok(rt) = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                rt.block_on(async {
                    if let Ok(mut admin) = PgConnection::connect(&admin_url).await {
                        // Kick any lingering connections before dropping.
                        let _ = admin
                            .execute(
                                format!(
                                    "SELECT pg_terminate_backend(pid) FROM pg_stat_activity \
                                     WHERE datname = '{name}' AND pid <> pg_backend_pid()"
                                )
                                .as_str(),
                            )
                            .await;
                        let _ = admin
                            .execute(format!(r#"DROP DATABASE IF EXISTS "{name}""#).as_str())
                            .await;
                        let _ = admin.close().await;
                    }
                });
            }
        })
        .join();
    }
}

/// Replace the database-name path segment of a Postgres URL, preserving any
/// query string (e.g. `?sslmode=require`).
fn swap_database(url: &str, db: &str) -> String {
    match url.find('?') {
        Some(qpos) => {
            let (base, query) = url.split_at(qpos);
            let cut = base.rfind('/').map(|i| i + 1).unwrap_or(base.len());
            format!("{}{}{}", &base[..cut], db, query)
        }
        None => {
            let cut = url.rfind('/').map(|i| i + 1).unwrap_or(url.len());
            format!("{}{}", &url[..cut], db)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::swap_database;

    #[test]
    fn swap_preserves_query() {
        assert_eq!(
            swap_database("postgres://u:p@h:5432/app?sslmode=require", "postgres"),
            "postgres://u:p@h:5432/postgres?sslmode=require"
        );
    }

    #[test]
    fn swap_without_query() {
        assert_eq!(
            swap_database("postgres://u:p@h:5432/app", "soma_test_x"),
            "postgres://u:p@h:5432/soma_test_x"
        );
    }
}
