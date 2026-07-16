//! Knowledge-graph query helpers over a Postgres + pgvector pool.
//!
//! Pure plumbing: thin query helpers for storing and traversing a property
//! graph (nodes, directed edges) and performing cosine-distance vector
//! similarity searches via pgvector. No business logic, no ingestion, no
//! embedding generation — the caller supplies all vectors.
//!
//! # Schema
//!
//! The consuming service owns the schema and migration. Copy the SQL blocks
//! below into your service's `migrations/` directory (soma-schema UP/DOWN
//! format). soma-infra ships only the Rust query helpers; the advisory lock
//! key and migration runner belong to the service.
//!
//! ## UP
//!
//! ```sql
//! -- UP
//! CREATE EXTENSION IF NOT EXISTS vector;
//!
//! CREATE TABLE kg_nodes (
//!     id    uuid    PRIMARY KEY,
//!     kind  text    NOT NULL,
//!     props jsonb   NOT NULL DEFAULT '{}'
//! );
//!
//! CREATE TABLE kg_edges (
//!     id    uuid PRIMARY KEY,
//!     src   uuid NOT NULL REFERENCES kg_nodes(id) ON DELETE CASCADE,
//!     dst   uuid NOT NULL REFERENCES kg_nodes(id) ON DELETE CASCADE,
//!     rel   text NOT NULL,
//!     props jsonb NOT NULL DEFAULT '{}'
//! );
//!
//! CREATE INDEX kg_edges_src_idx ON kg_edges(src);
//! CREATE INDEX kg_edges_dst_idx ON kg_edges(dst);
//!
//! -- 1536 is a PLACEHOLDER — choose the dimension your embedding model produces.
//! -- This value cannot change after rows exist without dropping and recreating this table.
//! CREATE TABLE kg_node_embeddings (
//!     node_id   uuid PRIMARY KEY REFERENCES kg_nodes(id) ON DELETE CASCADE,
//!     embedding vector(1536) NOT NULL
//! );
//!
//! -- operator class MUST match <=> (cosine distance) used by vector_search_cosine.
//! -- lists is a tuning parameter; adjust for dataset size.
//! CREATE INDEX kg_node_embeddings_cosine_idx
//!     ON kg_node_embeddings USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100);
//! ```
//!
//! ## DOWN
//!
//! ```sql
//! -- DOWN
//! DROP INDEX IF EXISTS kg_node_embeddings_cosine_idx;
//! DROP TABLE IF EXISTS kg_node_embeddings;
//! DROP INDEX IF EXISTS kg_edges_dst_idx;
//! DROP INDEX IF EXISTS kg_edges_src_idx;
//! DROP TABLE IF EXISTS kg_edges;
//! DROP TABLE IF EXISTS kg_nodes;
//! -- Intentionally NOT dropping the vector extension (destructive across a shared DB).
//! ```

// ponytail: all query strings are static — no string-concat of user values,
// no format! in hot paths. Direction branching uses separate query strings.

/// A graph node row.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct KgNode {
    pub id: uuid::Uuid,
    pub kind: String,
    pub props: serde_json::Value,
}

/// A directed graph edge row.
#[derive(Debug, Clone)]
pub struct KgEdge {
    pub id: uuid::Uuid,
    pub src: uuid::Uuid,
    pub dst: uuid::Uuid,
    pub rel: String,
    pub props: serde_json::Value,
}

/// Result of a vector similarity search.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct VectorMatch {
    pub node_id: uuid::Uuid,
    pub distance: f32,
}

/// Edge traversal direction for neighbor queries.
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Outgoing,
    Incoming,
    Both,
}

/// Errors from kg operations.
#[derive(Debug, thiserror::Error)]
pub enum KgError {
    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

// ── upsert_node ───────────────────────────────────────────────────────────────

/// Upsert a node by id. On conflict updates kind and props.
pub async fn upsert_node(pool: &sqlx::PgPool, node: &KgNode) -> Result<(), KgError> {
    sqlx::query(
        "INSERT INTO kg_nodes (id, kind, props) VALUES ($1, $2, $3)
         ON CONFLICT (id) DO UPDATE SET kind = EXCLUDED.kind, props = EXCLUDED.props",
    )
    .bind(node.id)
    .bind(&node.kind)
    .bind(&node.props)
    .execute(pool)
    .await?;
    Ok(())
}

// ── upsert_edge ───────────────────────────────────────────────────────────────

/// Upsert an edge by id. On conflict updates src, dst, rel, props.
pub async fn upsert_edge(pool: &sqlx::PgPool, edge: &KgEdge) -> Result<(), KgError> {
    sqlx::query(
        "INSERT INTO kg_edges (id, src, dst, rel, props) VALUES ($1, $2, $3, $4, $5)
         ON CONFLICT (id) DO UPDATE
             SET src = EXCLUDED.src, dst = EXCLUDED.dst,
                 rel = EXCLUDED.rel, props = EXCLUDED.props",
    )
    .bind(edge.id)
    .bind(edge.src)
    .bind(edge.dst)
    .bind(&edge.rel)
    .bind(&edge.props)
    .execute(pool)
    .await?;
    Ok(())
}

// ── neighbors ─────────────────────────────────────────────────────────────────

/// Return nodes reachable in one hop from `node_id`.
///
/// - `direction=Outgoing` — edges where `src = node_id`, returns `dst` nodes.
/// - `direction=Incoming` — edges where `dst = node_id`, returns `src` nodes.
/// - `direction=Both` — union of the above, deduplicated.
/// - `rel` — optional filter on the edge `rel` column; `None` returns all relations.
/// - `limit` — max rows to return.
pub async fn neighbors(
    pool: &sqlx::PgPool,
    node_id: uuid::Uuid,
    rel: Option<&str>,
    direction: Direction,
    limit: i64,
) -> Result<Vec<KgNode>, KgError> {
    // ponytail: six static query strings cover the full decision matrix
    // (3 directions × 2 rel filter states). No string concat, no user value
    // interpolation — all parameters bound positionally.
    match (direction, rel) {
        (Direction::Outgoing, None) => {
            sqlx::query_as::<_, KgNode>(
                "SELECT n.id, n.kind, n.props
                 FROM kg_nodes n
                 JOIN kg_edges e ON e.dst = n.id
                 WHERE e.src = $1
                 LIMIT $2",
            )
            .bind(node_id)
            .bind(limit)
            .fetch_all(pool)
            .await
        }
        (Direction::Outgoing, Some(r)) => {
            sqlx::query_as::<_, KgNode>(
                "SELECT n.id, n.kind, n.props
                 FROM kg_nodes n
                 JOIN kg_edges e ON e.dst = n.id
                 WHERE e.src = $1 AND e.rel = $3
                 LIMIT $2",
            )
            .bind(node_id)
            .bind(limit)
            .bind(r)
            .fetch_all(pool)
            .await
        }
        (Direction::Incoming, None) => {
            sqlx::query_as::<_, KgNode>(
                "SELECT n.id, n.kind, n.props
                 FROM kg_nodes n
                 JOIN kg_edges e ON e.src = n.id
                 WHERE e.dst = $1
                 LIMIT $2",
            )
            .bind(node_id)
            .bind(limit)
            .fetch_all(pool)
            .await
        }
        (Direction::Incoming, Some(r)) => {
            sqlx::query_as::<_, KgNode>(
                "SELECT n.id, n.kind, n.props
                 FROM kg_nodes n
                 JOIN kg_edges e ON e.src = n.id
                 WHERE e.dst = $1 AND e.rel = $3
                 LIMIT $2",
            )
            .bind(node_id)
            .bind(limit)
            .bind(r)
            .fetch_all(pool)
            .await
        }
        (Direction::Both, None) => {
            sqlx::query_as::<_, KgNode>(
                "SELECT DISTINCT n.id, n.kind, n.props
                 FROM kg_nodes n
                 JOIN kg_edges e ON (e.dst = n.id AND e.src = $1)
                    OR (e.src = n.id AND e.dst = $1)
                 LIMIT $2",
            )
            .bind(node_id)
            .bind(limit)
            .fetch_all(pool)
            .await
        }
        (Direction::Both, Some(r)) => {
            sqlx::query_as::<_, KgNode>(
                "SELECT DISTINCT n.id, n.kind, n.props
                 FROM kg_nodes n
                 JOIN kg_edges e ON (e.dst = n.id AND e.src = $1)
                    OR (e.src = n.id AND e.dst = $1)
                 WHERE e.rel = $3
                 LIMIT $2",
            )
            .bind(node_id)
            .bind(limit)
            .bind(r)
            .fetch_all(pool)
            .await
        }
    }
    .map_err(KgError::Sqlx)
}

// ── vector_search_cosine ──────────────────────────────────────────────────────

/// Cosine-distance vector search. Returns top-`k` nodes by `embedding <=> query`.
///
/// Smaller distance = more similar. The caller supplies the query vector;
/// no embedding generation happens here.
pub async fn vector_search_cosine(
    pool: &sqlx::PgPool,
    embedding: pgvector::Vector,
    k: i64,
) -> Result<Vec<VectorMatch>, KgError> {
    // ponytail: cast to float4 explicitly so sqlx maps the result to f32
    // without a second round-trip through f64.
    sqlx::query_as::<_, VectorMatch>(
        "SELECT node_id, (embedding <=> $1)::float4 AS distance
         FROM kg_node_embeddings
         ORDER BY embedding <=> $1
         LIMIT $2",
    )
    .bind(embedding)
    .bind(k)
    .fetch_all(pool)
    .await
    .map_err(KgError::Sqlx)
}

// ── unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kg_node_is_clone_and_debug() {
        let node = KgNode {
            id: uuid::Uuid::nil(),
            kind: "test".into(),
            props: serde_json::json!({}),
        };
        let cloned = node.clone();
        assert_eq!(cloned.kind, node.kind);
        // Debug impl must not panic
        let _ = format!("{:?}", node);
    }

    #[test]
    fn kg_edge_is_clone_and_debug() {
        let edge = KgEdge {
            id: uuid::Uuid::nil(),
            src: uuid::Uuid::nil(),
            dst: uuid::Uuid::nil(),
            rel: "links".into(),
            props: serde_json::json!({"weight": 1}),
        };
        let cloned = edge.clone();
        assert_eq!(cloned.rel, edge.rel);
        let _ = format!("{:?}", edge);
    }

    #[test]
    fn vector_match_is_clone_and_debug() {
        let vm = VectorMatch {
            node_id: uuid::Uuid::nil(),
            distance: 0.5,
        };
        let cloned = vm.clone();
        assert!((cloned.distance - 0.5).abs() < f32::EPSILON);
        let _ = format!("{:?}", vm);
    }

    #[test]
    fn direction_variants_are_copy() {
        let d = Direction::Outgoing;
        let _d2 = d; // copy, not move
        let _ = format!("{:?}", d);
        let _ = format!("{:?}", Direction::Incoming);
        let _ = format!("{:?}", Direction::Both);
    }

    #[test]
    fn kg_error_display() {
        let e = KgError::Sqlx(sqlx::Error::RowNotFound);
        assert!(e.to_string().contains("database error"));
    }
}
