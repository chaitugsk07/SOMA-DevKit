//! Integration test for the `db` + `testing` features. Requires a reachable
//! Postgres via `TEST_DATABASE_URL`. Run: `cargo test --features testing`.
#![cfg(feature = "testing")]

use soma_infra::testing::TestDb;

#[tokio::test]
async fn testdb_gives_isolated_working_pool() {
    let db = TestDb::create_from_env()
        .await
        .expect("create throwaway database");

    let row: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(&db.pool)
        .await
        .expect("SELECT 1");
    assert_eq!(row.0, 1);
    // Dropping `db` here removes the database (RAII).
}
