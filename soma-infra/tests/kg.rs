//! Integration tests for the kg module.
//!
//! Skipped when `TEST_DATABASE_URL` is unset. Run with:
//! ```
//! cargo test --features "kg testing"
//! ```
#![cfg(all(feature = "kg", feature = "testing"))]

use soma_infra::kg::{
    neighbors, upsert_edge, upsert_node, vector_search_cosine, Direction, KgEdge, KgNode,
};

/// Drops the kg tables on `Drop` so every test run starts clean.
struct KgCleanup {
    pool: sqlx::PgPool,
}

impl Drop for KgCleanup {
    fn drop(&mut self) {
        // ponytail: block_in_place lets us run async cleanup in a sync Drop.
        // The tokio runtime is still live during test teardown.
        let pool = self.pool.clone();
        let _ = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async move {
                let _ = sqlx::query(
                    "DROP INDEX IF EXISTS kg_node_embeddings_cosine_idx;
                     DROP TABLE IF EXISTS kg_node_embeddings;
                     DROP INDEX IF EXISTS kg_edges_dst_idx;
                     DROP INDEX IF EXISTS kg_edges_src_idx;
                     DROP TABLE IF EXISTS kg_edges;
                     DROP TABLE IF EXISTS kg_nodes;",
                )
                .execute(&pool)
                .await;
            })
        });
    }
}

async fn setup(pool: &sqlx::PgPool) {
    // Uses vector(3) — minimal dimension for a fast, self-contained test.
    sqlx::query(
        "CREATE EXTENSION IF NOT EXISTS vector;
         CREATE TABLE IF NOT EXISTS kg_nodes (
             id    uuid  PRIMARY KEY,
             kind  text  NOT NULL,
             props jsonb NOT NULL DEFAULT '{}'
         );
         CREATE TABLE IF NOT EXISTS kg_edges (
             id    uuid PRIMARY KEY,
             src   uuid NOT NULL REFERENCES kg_nodes(id) ON DELETE CASCADE,
             dst   uuid NOT NULL REFERENCES kg_nodes(id) ON DELETE CASCADE,
             rel   text NOT NULL,
             props jsonb NOT NULL DEFAULT '{}'
         );
         CREATE INDEX IF NOT EXISTS kg_edges_src_idx ON kg_edges(src);
         CREATE INDEX IF NOT EXISTS kg_edges_dst_idx ON kg_edges(dst);
         CREATE TABLE IF NOT EXISTS kg_node_embeddings (
             node_id   uuid PRIMARY KEY REFERENCES kg_nodes(id) ON DELETE CASCADE,
             embedding vector(3) NOT NULL
         );",
    )
    .execute(pool)
    .await
    .expect("kg schema setup");
}

#[tokio::test(flavor = "multi_thread")]
async fn kg_roundtrip() {
    let url = match std::env::var("TEST_DATABASE_URL") {
        Ok(u) => u,
        Err(_) => return, // no DB available — skip gracefully
    };

    let pool = sqlx::PgPool::connect(&url)
        .await
        .expect("connect to TEST_DATABASE_URL");

    setup(&pool).await;
    // KgCleanup drops the tables when it goes out of scope at the end of the test.
    let _cleanup = KgCleanup { pool: pool.clone() };

    // ── a. Upsert two nodes ───────────────────────────────────────────────────
    let node_a_id = uuid::Uuid::new_v4();
    let node_b_id = uuid::Uuid::new_v4();

    let node_a = KgNode {
        id: node_a_id,
        kind: "test".into(),
        props: serde_json::json!({}),
    };
    let node_b = KgNode {
        id: node_b_id,
        kind: "test".into(),
        props: serde_json::json!({}),
    };

    upsert_node(&pool, &node_a).await.expect("upsert node_a");
    upsert_node(&pool, &node_b).await.expect("upsert node_b");

    // Upsert idempotency: re-upserting must not error.
    upsert_node(&pool, &node_a).await.expect("re-upsert node_a");

    // ── b. Upsert an edge from node_a → node_b ────────────────────────────────
    let edge = KgEdge {
        id: uuid::Uuid::new_v4(),
        src: node_a_id,
        dst: node_b_id,
        rel: "links".into(),
        props: serde_json::json!({}),
    };
    upsert_edge(&pool, &edge).await.expect("upsert edge");

    // ── c. neighbors: Outgoing from node_a should contain node_b ─────────────
    let outgoing = neighbors(&pool, node_a_id, None, Direction::Outgoing, 10)
        .await
        .expect("neighbors outgoing");
    assert!(
        outgoing.iter().any(|n| n.id == node_b_id),
        "outgoing neighbors of node_a must include node_b; got: {outgoing:?}",
    );

    // Incoming from node_b should also yield node_a's perspective (node_a is the src).
    let incoming = neighbors(&pool, node_b_id, None, Direction::Incoming, 10)
        .await
        .expect("neighbors incoming");
    assert!(
        incoming.iter().any(|n| n.id == node_a_id),
        "incoming neighbors of node_b must include node_a; got: {incoming:?}",
    );

    // Both from node_a should include node_b.
    let both = neighbors(&pool, node_a_id, None, Direction::Both, 10)
        .await
        .expect("neighbors both");
    assert!(
        both.iter().any(|n| n.id == node_b_id),
        "both-direction neighbors of node_a must include node_b; got: {both:?}",
    );

    // rel filter: "links" should find node_b; a different rel should not.
    let filtered = neighbors(&pool, node_a_id, Some("links"), Direction::Outgoing, 10)
        .await
        .expect("neighbors filtered");
    assert!(filtered.iter().any(|n| n.id == node_b_id));

    let empty = neighbors(
        &pool,
        node_a_id,
        Some("no-such-rel"),
        Direction::Outgoing,
        10,
    )
    .await
    .expect("neighbors no-match");
    assert!(empty.is_empty());

    // ── d. Upsert an embedding for node_a (3-dimensional) ────────────────────
    sqlx::query(
        "INSERT INTO kg_node_embeddings (node_id, embedding)
         VALUES ($1, $2)
         ON CONFLICT (node_id) DO UPDATE SET embedding = EXCLUDED.embedding",
    )
    .bind(node_a_id)
    .bind(pgvector::Vector::from(vec![0.1f32, 0.2, 0.3]))
    .execute(&pool)
    .await
    .expect("insert embedding for node_a");

    // ── e. vector_search_cosine: nearest to [0.1, 0.2, 0.3] must be node_a ──
    let matches = vector_search_cosine(&pool, pgvector::Vector::from(vec![0.1f32, 0.2, 0.3]), 5)
        .await
        .expect("vector_search_cosine");

    assert!(
        !matches.is_empty(),
        "vector search must return at least one result"
    );
    assert_eq!(
        matches[0].node_id, node_a_id,
        "closest match must be node_a; got: {:?}",
        matches[0],
    );
}
