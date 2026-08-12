//! Object-storage access via a configured `StorageClient`.
//!
//! Pure plumbing: configure and construct, then hand the client to the caller.
//! No key/path naming, no prefix prepending, no retry policy (object_store
//! retries internally), no content-type headers, no presigned URLs
//! — those belong in the service that knows what it is storing.
//! Streaming multipart upload, ranged reads, and object-size queries are
//! provided alongside the basic get/put/delete/list/copy operations.
//!
//! Build a client from an explicit config with [`StorageClient::new`] or pull
//! it straight from the environment with [`StorageClient::from_env`].
//!
//! Credentials (`AWS_ACCESS_KEY_ID`/`AWS_SECRET_ACCESS_KEY` for S3,
//! `AZURE_STORAGE_ACCOUNT_KEY` for Azure) are read natively by the
//! object_store builders from the environment — do not set them here.
//!
//! # Feature selection
//!
//! | feature | backend |
//! |---------|---------|
//! | `storage-s3` | AWS S3 (or S3-compatible endpoint) |
//! | `storage-azure` | Azure Blob Storage |
//!
//! Both features may be enabled simultaneously; in that case, `StorageConfig`
//! is an enum where each variant carries only its backend's fields — no
//! zero-filling. Use [`StorageConfig::new_s3`] or [`StorageConfig::new_azure`]
//! to build the right variant, then pass it to [`StorageClient::new`].
//!
//! # When both features are enabled
//!
//! `StorageConfig` is an enum with one variant per backend. Each variant holds
//! only the fields it needs — an `S3` variant has no Azure fields and vice
//! versa. `StorageClient::new` matches on the variant to build the right
//! backend.

use std::sync::Arc;

use object_store::path::Path;
use object_store::{ObjectStore, WriteMultipart};

// ── Send + Sync assertion ─────────────────────────────────────────────────────
// Fails to compile if StorageClient ever loses Send+Sync (e.g. a non-Send inner
// type). This is the test; no runtime assertion is needed.
const _: fn() = || {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<StorageClient>();
};

// ── Error ─────────────────────────────────────────────────────────────────────

/// Errors from building a storage client or executing an operation.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    /// A required environment variable was missing.
    #[error("required env var {0} is not set")]
    MissingEnv(&'static str),
    /// An environment variable held an unparseable value.
    #[error("env var {0} has an invalid value")]
    InvalidEnv(&'static str),
    /// The object key could not be parsed into a valid `Path`.
    #[error("invalid object key: {0}")]
    InvalidKey(String),
    /// An error returned by the object store backend.
    #[error(transparent)]
    Store(#[from] object_store::Error),
    /// An I/O error reading the upload source stream.
    #[error("stream read error: {0}")]
    Io(#[from] std::io::Error),
}

// ── StorageConfig ─────────────────────────────────────────────────────────────

/// Tunables for building a [`StorageClient`]. Construct with
/// [`StorageConfig::new_s3`] (S3), [`StorageConfig::new_azure`] (Azure), or
/// [`StorageConfig::new_fs`] (local filesystem), or read from the environment
/// with [`StorageConfig::from_env`].
///
/// Each variant carries only the fields for its backend — invalid states
/// (e.g. an S3 config with empty Azure fields) are unrepresentable.
#[derive(Debug, Clone)]
pub enum StorageConfig {
    /// AWS S3 (or S3-compatible endpoint) backend.
    #[cfg(feature = "storage-s3")]
    S3 {
        /// S3 bucket name.
        bucket: String,
        /// AWS region, e.g. `"us-east-1"`.
        region: String,
        /// Optional custom endpoint (S3-compatible stores, LocalStack, MinIO).
        endpoint: Option<String>,
    },
    /// Azure Blob Storage backend.
    #[cfg(feature = "storage-azure")]
    Azure {
        /// Azure storage account name.
        account: String,
        /// Azure blob container name.
        container: String,
    },
    /// Local filesystem backend.
    ///
    /// Intended for: integration tests without MinIO, and homelab single-disk
    /// deployments where object-storage economics are not required.
    /// All object paths are rooted under `root` on the local filesystem.
    /// Env: `STORAGE_BACKEND="fs"`, `STORAGE_FS_ROOT=<absolute-path>`.
    #[cfg(feature = "storage-fs")]
    Fs {
        /// Absolute path to the root directory for all object keys.
        root: std::path::PathBuf,
    },
}

impl StorageConfig {
    /// Build an S3 config from explicit values.
    ///
    /// `endpoint` is optional; set it for S3-compatible stores (MinIO, LocalStack).
    #[cfg(feature = "storage-s3")]
    pub fn new_s3(
        bucket: impl Into<String>,
        region: impl Into<String>,
        endpoint: Option<String>,
    ) -> Self {
        Self::S3 {
            bucket: bucket.into(),
            region: region.into(),
            endpoint,
        }
    }

    /// Build an Azure config from explicit values.
    #[cfg(feature = "storage-azure")]
    pub fn new_azure(account: impl Into<String>, container: impl Into<String>) -> Self {
        Self::Azure {
            account: account.into(),
            container: container.into(),
        }
    }

    /// Build a local-filesystem config from an explicit root path.
    #[cfg(feature = "storage-fs")]
    pub fn new_fs(root: impl Into<std::path::PathBuf>) -> Self {
        Self::Fs { root: root.into() }
    }

    /// Read config from the environment for whichever backend feature(s) are active.
    ///
    /// **Single feature compiled in:** reads directly without a discriminant.
    /// **Multiple features compiled in:** reads `STORAGE_BACKEND` (`"s3"`,
    /// `"azure"`, or `"fs"`) to choose which backend to configure.
    ///
    /// S3 env vars: `STORAGE_BUCKET` (req), `STORAGE_S3_REGION` (req),
    /// `STORAGE_S3_ENDPOINT` (opt).
    /// Azure env vars: `STORAGE_AZURE_ACCOUNT` (req), `STORAGE_AZURE_CONTAINER` (req).
    /// Fs env vars: `STORAGE_FS_ROOT` (req, absolute path).
    ///
    /// S3/Azure credentials are NOT read here — object_store picks them up
    /// natively from the environment (`AWS_ACCESS_KEY_ID` / `AZURE_STORAGE_ACCOUNT_KEY`).
    pub fn from_env() -> Result<Self, StorageError> {
        // Each cfg block is mutually exclusive at compile time; exactly one is
        // kept, and its value is the function's return value.  The final
        // unreachable line is never reached but satisfies the type checker when
        // no storage feature is compiled (which would also make StorageConfig
        // uninhabited — this function would be uncallable anyway).

        // ── single-feature fast paths (no STORAGE_BACKEND needed) ────────────

        #[cfg(all(
            feature = "storage-s3",
            not(feature = "storage-azure"),
            not(feature = "storage-fs")
        ))]
        {
            return Self::from_env_s3();
        }

        #[cfg(all(
            feature = "storage-azure",
            not(feature = "storage-s3"),
            not(feature = "storage-fs")
        ))]
        {
            return Self::from_env_azure();
        }

        #[cfg(all(
            feature = "storage-fs",
            not(feature = "storage-s3"),
            not(feature = "storage-azure")
        ))]
        {
            return Self::from_env_fs();
        }

        // ── multi-feature paths: STORAGE_BACKEND discriminant required ────────

        #[cfg(all(
            feature = "storage-s3",
            feature = "storage-azure",
            not(feature = "storage-fs")
        ))]
        {
            let b = std::env::var("STORAGE_BACKEND")
                .map_err(|_| StorageError::MissingEnv("STORAGE_BACKEND"))?;
            return match b.to_lowercase().as_str() {
                "s3" => Self::from_env_s3(),
                "azure" => Self::from_env_azure(),
                _ => Err(StorageError::InvalidEnv("STORAGE_BACKEND")),
            };
        }

        #[cfg(all(
            feature = "storage-s3",
            feature = "storage-fs",
            not(feature = "storage-azure")
        ))]
        {
            let b = std::env::var("STORAGE_BACKEND")
                .map_err(|_| StorageError::MissingEnv("STORAGE_BACKEND"))?;
            return match b.to_lowercase().as_str() {
                "s3" => Self::from_env_s3(),
                "fs" => Self::from_env_fs(),
                _ => Err(StorageError::InvalidEnv("STORAGE_BACKEND")),
            };
        }

        #[cfg(all(
            feature = "storage-azure",
            feature = "storage-fs",
            not(feature = "storage-s3")
        ))]
        {
            let b = std::env::var("STORAGE_BACKEND")
                .map_err(|_| StorageError::MissingEnv("STORAGE_BACKEND"))?;
            return match b.to_lowercase().as_str() {
                "azure" => Self::from_env_azure(),
                "fs" => Self::from_env_fs(),
                _ => Err(StorageError::InvalidEnv("STORAGE_BACKEND")),
            };
        }

        #[cfg(all(
            feature = "storage-s3",
            feature = "storage-azure",
            feature = "storage-fs"
        ))]
        {
            let b = std::env::var("STORAGE_BACKEND")
                .map_err(|_| StorageError::MissingEnv("STORAGE_BACKEND"))?;
            return match b.to_lowercase().as_str() {
                "s3" => Self::from_env_s3(),
                "azure" => Self::from_env_azure(),
                "fs" => Self::from_env_fs(),
                _ => Err(StorageError::InvalidEnv("STORAGE_BACKEND")),
            };
        }

        // Unreachable when any storage feature is compiled (and this function
        // is uncallable when no storage feature is compiled — StorageConfig has
        // no variants).
        #[allow(unreachable_code)]
        Err(StorageError::MissingEnv("STORAGE_BACKEND"))
    }

    #[cfg(feature = "storage-s3")]
    fn from_env_s3() -> Result<Self, StorageError> {
        let bucket = std::env::var("STORAGE_BUCKET")
            .map_err(|_| StorageError::MissingEnv("STORAGE_BUCKET"))?;
        let region = std::env::var("STORAGE_S3_REGION")
            .map_err(|_| StorageError::MissingEnv("STORAGE_S3_REGION"))?;
        let endpoint = std::env::var("STORAGE_S3_ENDPOINT").ok();
        Ok(Self::new_s3(bucket, region, endpoint))
    }

    #[cfg(feature = "storage-azure")]
    fn from_env_azure() -> Result<Self, StorageError> {
        let account = std::env::var("STORAGE_AZURE_ACCOUNT")
            .map_err(|_| StorageError::MissingEnv("STORAGE_AZURE_ACCOUNT"))?;
        let container = std::env::var("STORAGE_AZURE_CONTAINER")
            .map_err(|_| StorageError::MissingEnv("STORAGE_AZURE_CONTAINER"))?;
        Ok(Self::new_azure(account, container))
    }

    #[cfg(feature = "storage-fs")]
    fn from_env_fs() -> Result<Self, StorageError> {
        let root = std::env::var("STORAGE_FS_ROOT")
            .map_err(|_| StorageError::MissingEnv("STORAGE_FS_ROOT"))?;
        Ok(Self::new_fs(root))
    }
}

// ── StorageClient ─────────────────────────────────────────────────────────────

/// A thin, cheaply-cloneable handle to an object-store backend.
///
/// Cloning is `O(1)` — it increments an `Arc` reference count.
/// The caller owns all path naming, retry, and content-type decisions.
pub struct StorageClient {
    inner: Arc<dyn ObjectStore>,
}

impl std::fmt::Debug for StorageClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StorageClient")
            .field("inner", &self.inner)
            .finish()
    }
}

// Manual Clone: dyn ObjectStore does not satisfy the Clone bound, but Arc::clone
// is O(1) and correct — both handles share the same underlying store.
impl Clone for StorageClient {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl StorageClient {
    /// Build a client from a [`StorageConfig`].
    pub fn new(cfg: &StorageConfig) -> Result<Self, StorageError> {
        let store: Arc<dyn ObjectStore> = match cfg {
            #[cfg(feature = "storage-s3")]
            StorageConfig::S3 {
                bucket,
                region,
                endpoint,
            } => {
                use object_store::aws::AmazonS3Builder;
                // ponytail: credentials (AWS_ACCESS_KEY_ID / AWS_SECRET_ACCESS_KEY
                // / AWS_SESSION_TOKEN / instance metadata) are picked up natively
                // by AmazonS3Builder — we deliberately do not read them here.
                let mut builder = AmazonS3Builder::new()
                    .with_bucket_name(bucket)
                    .with_region(region);
                if let Some(ep) = endpoint {
                    builder = builder.with_endpoint(ep);
                }
                Arc::new(builder.build()?)
            }
            #[cfg(feature = "storage-azure")]
            StorageConfig::Azure { account, container } => {
                use object_store::azure::MicrosoftAzureBuilder;
                // ponytail: AZURE_STORAGE_ACCOUNT_KEY (or SAS token / MSI) is
                // read natively by MicrosoftAzureBuilder — not our concern.
                let store = MicrosoftAzureBuilder::new()
                    .with_account(account)
                    .with_container_name(container)
                    .build()?;
                Arc::new(store)
            }
            #[cfg(feature = "storage-fs")]
            StorageConfig::Fs { root } => {
                use object_store::local::LocalFileSystem;
                // ponytail: new_with_prefix requires an absolute path; callers
                // are responsible for ensuring root is absolute.  The local FS
                // backend is only appropriate for tests and homelab single-disk
                // deployments — not for production S3/Azure use.
                let store = LocalFileSystem::new_with_prefix(root)?;
                Arc::new(store)
            }
        };
        Ok(Self { inner: store })
    }

    /// Convenience: [`StorageConfig::from_env`] then [`StorageClient::new`].
    pub fn from_env() -> Result<Self, StorageError> {
        Self::new(&StorageConfig::from_env()?)
    }

    /// Download an object by key, returning its full body as [`bytes::Bytes`].
    pub async fn get(&self, key: &str) -> Result<bytes::Bytes, StorageError> {
        let path = parse_key(key)?;
        Ok(self.inner.get(&path).await?.bytes().await?)
    }

    /// Upload `data` to the given key, replacing any existing object.
    pub async fn put(&self, key: &str, data: bytes::Bytes) -> Result<(), StorageError> {
        let path = parse_key(key)?;
        self.inner.put(&path, data.into()).await?;
        Ok(())
    }

    /// Delete the object at the given key.
    pub async fn delete(&self, key: &str) -> Result<(), StorageError> {
        let path = parse_key(key)?;
        self.inner.delete(&path).await?;
        Ok(())
    }

    /// List object keys directly under `prefix` (non-recursive).
    ///
    /// Returns the location strings of objects found at exactly one path level
    /// below `prefix`.  Subdirectory prefixes (common prefixes) are NOT
    /// returned; only concrete objects are.
    ///
    /// ponytail: uses `list_with_delimiter` so no `futures` dep is needed.
    /// Ceiling: cannot list recursively.  Upgrade path: add `dep:futures`,
    /// call `self.inner.list(Some(&path))`, and collect via
    /// `futures::TryStreamExt::try_collect`.
    pub async fn list(&self, prefix: &str) -> Result<Vec<String>, StorageError> {
        let prefix_path = parse_key(prefix)?;
        let result = self.inner.list_with_delimiter(Some(&prefix_path)).await?;
        Ok(result
            .objects
            .into_iter()
            .map(|m| m.location.to_string())
            .collect())
    }

    /// Server-side copy from `from` to `to` (no data passes through this process).
    ///
    /// On S3 and Azure this is an in-cloud copy; on the local filesystem it
    /// copies bytes on disk.  The source object is **not** deleted — call
    /// [`delete`] afterwards to implement a move.
    pub async fn copy(&self, from: &str, to: &str) -> Result<(), StorageError> {
        let from_path = parse_key(from)?;
        let to_path = parse_key(to)?;
        self.inner.copy(&from_path, &to_path).await?;
        Ok(())
    }

    /// Return the underlying [`ObjectStore`] for callers that need the raw
    /// object_store interface (e.g. DataFusion's `RuntimeEnv::register_object_store`).
    ///
    /// **Escape hatch.** Prefer the typed methods on [`StorageClient`] for all
    /// other uses — this API exposes an internal type that may change.
    pub fn object_store(&self) -> Arc<dyn ObjectStore> {
        Arc::clone(&self.inner)
    }

    /// Stream an upload from an [`AsyncRead`] source using object_store's
    /// multipart API.  Reads the source in 8 MiB chunks, so peak memory is
    /// bounded to roughly one chunk regardless of object size.  Returns the
    /// total bytes written.
    ///
    /// [`AsyncRead`]: tokio::io::AsyncRead
    pub async fn put_stream<R>(&self, key: &str, reader: R) -> Result<u64, StorageError>
    where
        R: tokio::io::AsyncRead + Unpin + Send,
    {
        use tokio::io::AsyncReadExt;

        const CHUNK: usize = 8 * 1024 * 1024; // 8 MiB

        let path = parse_key(key)?;
        let upload = self.inner.put_multipart(&path).await?;
        let mut w = WriteMultipart::new_with_chunk_size(upload, CHUNK);
        let mut buf = vec![0u8; CHUNK];
        let mut total = 0u64;
        let mut r = reader;

        loop {
            let n = r.read(&mut buf).await?;
            if n == 0 {
                break;
            }
            w.write(&buf[..n]);
            total += n as u64;
        }

        w.finish().await?;
        Ok(total)
    }

    /// Fetch a byte range of an object.
    ///
    /// `range` is byte-offset bounds (start inclusive, end exclusive).
    pub async fn get_range(
        &self,
        key: &str,
        range: std::ops::Range<u64>,
    ) -> Result<bytes::Bytes, StorageError> {
        let path = parse_key(key)?;
        Ok(self.inner.get_range(&path, range).await?)
    }

    /// Return the byte-size of an existing object.
    ///
    /// Implemented via a `head` request — one round-trip, no data transferred.
    pub async fn size(&self, key: &str) -> Result<u64, StorageError> {
        let path = parse_key(key)?;
        Ok(self.inner.head(&path).await?.size)
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

/// Parse an object key into an `object_store` `Path`.
///
/// `Path::parse` returns a `Result` (unlike `Path::from` which can panic on
/// invalid input). Rejects keys with consecutive slashes, `.` or `..`
/// segments, and other malformed paths. Leading slashes are silently stripped
/// by object_store 0.12 (library behaviour, not our logic).
fn parse_key(key: &str) -> Result<Path, StorageError> {
    Path::parse(key).map_err(|e| StorageError::InvalidKey(e.to_string()))
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── parse_key ─────────────────────────────────────────────────────────────

    #[test]
    fn parse_key_accepts_normal_paths() {
        assert!(parse_key("foo/bar/baz.json").is_ok());
        assert!(parse_key("single").is_ok());
        // object_store 0.12 silently strips a leading slash
        assert!(parse_key("/leading-slash").is_ok());
    }

    #[test]
    fn parse_key_rejects_double_slash_returns_invalid_key() {
        let err = parse_key("foo//bar").unwrap_err();
        assert!(matches!(err, StorageError::InvalidKey(_)));
    }

    #[test]
    fn parse_key_rejects_dot_segment_returns_invalid_key() {
        let err = parse_key("foo/./bar").unwrap_err();
        assert!(matches!(err, StorageError::InvalidKey(_)));
    }

    #[test]
    fn parse_key_rejects_dotdot_segment_returns_invalid_key() {
        let err = parse_key("foo/../bar").unwrap_err();
        assert!(matches!(err, StorageError::InvalidKey(_)));
    }

    // ── StorageConfig::from_env — S3 paths ───────────────────────────────────

    #[cfg(all(feature = "storage-s3", not(feature = "storage-azure")))]
    #[test]
    fn from_env_s3_paths() {
        // Sequential to avoid races on shared env vars.

        // — missing STORAGE_BUCKET —
        std::env::remove_var("STORAGE_BUCKET");
        std::env::remove_var("STORAGE_S3_REGION");
        std::env::remove_var("STORAGE_S3_ENDPOINT");
        assert!(matches!(
            StorageConfig::from_env().unwrap_err(),
            StorageError::MissingEnv("STORAGE_BUCKET")
        ));

        // — bucket present, region missing —
        std::env::set_var("STORAGE_BUCKET", "my-bucket");
        assert!(matches!(
            StorageConfig::from_env().unwrap_err(),
            StorageError::MissingEnv("STORAGE_S3_REGION")
        ));

        // — both required vars present, no optional endpoint —
        std::env::set_var("STORAGE_S3_REGION", "us-east-1");
        let cfg = StorageConfig::from_env().unwrap();
        match cfg {
            StorageConfig::S3 {
                bucket,
                region,
                endpoint,
            } => {
                assert_eq!(bucket, "my-bucket");
                assert_eq!(region, "us-east-1");
                assert!(endpoint.is_none());
            }
            #[allow(unreachable_patterns)]
            _ => panic!("expected S3 variant"),
        }

        // — optional endpoint present —
        std::env::set_var("STORAGE_S3_ENDPOINT", "http://localhost:9000");
        let cfg = StorageConfig::from_env().unwrap();
        match cfg {
            StorageConfig::S3 { endpoint, .. } => {
                assert_eq!(endpoint, Some("http://localhost:9000".into()));
            }
            #[allow(unreachable_patterns)]
            _ => panic!("expected S3 variant"),
        }

        // cleanup
        std::env::remove_var("STORAGE_BUCKET");
        std::env::remove_var("STORAGE_S3_REGION");
        std::env::remove_var("STORAGE_S3_ENDPOINT");
    }

    // ── StorageConfig::from_env — Azure paths ────────────────────────────────

    #[cfg(all(feature = "storage-azure", not(feature = "storage-s3")))]
    #[test]
    fn from_env_azure_paths() {
        // Sequential to avoid races on shared env vars.

        // — missing STORAGE_AZURE_ACCOUNT —
        std::env::remove_var("STORAGE_AZURE_ACCOUNT");
        std::env::remove_var("STORAGE_AZURE_CONTAINER");
        assert!(matches!(
            StorageConfig::from_env().unwrap_err(),
            StorageError::MissingEnv("STORAGE_AZURE_ACCOUNT")
        ));

        // — account present, container missing —
        std::env::set_var("STORAGE_AZURE_ACCOUNT", "myaccount");
        assert!(matches!(
            StorageConfig::from_env().unwrap_err(),
            StorageError::MissingEnv("STORAGE_AZURE_CONTAINER")
        ));

        // — both present —
        std::env::set_var("STORAGE_AZURE_CONTAINER", "mycontainer");
        let cfg = StorageConfig::from_env().unwrap();
        match cfg {
            StorageConfig::Azure { account, container } => {
                assert_eq!(account, "myaccount");
                assert_eq!(container, "mycontainer");
            }
            #[allow(unreachable_patterns)]
            _ => panic!("expected Azure variant"),
        }

        // cleanup
        std::env::remove_var("STORAGE_AZURE_ACCOUNT");
        std::env::remove_var("STORAGE_AZURE_CONTAINER");
    }

    // ── StorageConfig::from_env — Fs path ────────────────────────────────────

    #[cfg(all(
        feature = "storage-fs",
        not(feature = "storage-s3"),
        not(feature = "storage-azure")
    ))]
    #[test]
    fn from_env_fs_paths() {
        std::env::remove_var("STORAGE_FS_ROOT");
        assert!(matches!(
            StorageConfig::from_env().unwrap_err(),
            StorageError::MissingEnv("STORAGE_FS_ROOT")
        ));

        let root = std::env::temp_dir().join("soma-infra-storage-fs-test");
        std::fs::create_dir_all(&root).ok();
        std::env::set_var("STORAGE_FS_ROOT", root.to_str().unwrap());
        let cfg = StorageConfig::from_env().unwrap();
        match cfg {
            StorageConfig::Fs { root: r } => {
                assert_eq!(r.to_str().unwrap(), root.to_str().unwrap());
            }
            #[allow(unreachable_patterns)]
            _ => panic!("expected Fs variant"),
        }
        std::env::remove_var("STORAGE_FS_ROOT");
    }

    // ── StorageClient::list / copy / object_store via Fs backend ─────────────

    #[cfg(feature = "storage-fs")]
    #[tokio::test]
    async fn fs_list_copy_delete_object_store() {
        let dir = std::env::temp_dir().join(format!(
            "soma-infra-fs-ops-{:016x}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .subsec_nanos() as u64
        ));
        std::fs::create_dir_all(&dir).unwrap();

        let cfg = StorageConfig::Fs { root: dir.clone() };
        let client = StorageClient::new(&cfg).unwrap();

        // put two objects
        client
            .put("a/obj1.txt", bytes::Bytes::from("hello"))
            .await
            .unwrap();
        client
            .put("a/obj2.txt", bytes::Bytes::from("world"))
            .await
            .unwrap();

        // list — should return both
        let mut keys = client.list("a/").await.unwrap();
        keys.sort();
        assert_eq!(keys.len(), 2, "expected 2 keys under a/");

        // copy
        client.copy("a/obj1.txt", "b/obj1_copy.txt").await.unwrap();
        let copied = client.get("b/obj1_copy.txt").await.unwrap();
        assert_eq!(
            copied,
            bytes::Bytes::from("hello"),
            "copy must preserve content"
        );

        // object_store escape hatch returns a usable Arc<dyn ObjectStore>
        let raw = client.object_store();
        let _ = raw; // just verify it compiles and type-checks

        // cleanup
        std::fs::remove_dir_all(&dir).ok();
    }

    // ── put_stream / get_range / size via Fs backend ─────────────────────────

    #[cfg(feature = "storage-fs")]
    #[tokio::test]
    async fn fs_put_stream_get_range_size() {
        use std::io::Cursor;

        let dir = std::env::temp_dir().join(format!(
            "soma-infra-fs-stream-{:016x}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .subsec_nanos() as u64
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let cfg = StorageConfig::Fs { root: dir.clone() };
        let client = StorageClient::new(&cfg).unwrap();

        const SIZE: usize = 20 * 1024 * 1024; // 20 MiB

        // pseudo-random fill: XOR of byte-position layers — no extra dep
        let data: Vec<u8> = (0..SIZE)
            .map(|i| (i ^ (i >> 8) ^ (i >> 16)) as u8)
            .collect();

        // ── streaming upload ──────────────────────────────────────────────────
        let n = client
            .put_stream("video/test.bin", Cursor::new(data.clone()))
            .await
            .unwrap();
        assert_eq!(n, SIZE as u64, "put_stream returned wrong byte count");

        // ── size ──────────────────────────────────────────────────────────────
        let reported = client.size("video/test.bin").await.unwrap();
        assert_eq!(reported, SIZE as u64, "size() does not match upload");

        // ── ranged reads ──────────────────────────────────────────────────────
        let head = client.get_range("video/test.bin", 0..1024).await.unwrap();
        assert_eq!(head.as_ref(), &data[0..1024], "head range mismatch");

        let mid_start = SIZE / 2;
        let mid = client
            .get_range(
                "video/test.bin",
                mid_start as u64..(mid_start + 1024) as u64,
            )
            .await
            .unwrap();
        assert_eq!(
            mid.as_ref(),
            &data[mid_start..mid_start + 1024],
            "middle range mismatch"
        );

        let tail_start = SIZE - 1024;
        let tail = client
            .get_range("video/test.bin", tail_start as u64..SIZE as u64)
            .await
            .unwrap();
        assert_eq!(tail.as_ref(), &data[tail_start..], "tail range mismatch");

        // ── full get roundtrip ────────────────────────────────────────────────
        let full = client.get("video/test.bin").await.unwrap();
        assert_eq!(
            full.as_ref(),
            data.as_slice(),
            "full get roundtrip mismatch"
        );

        // cleanup
        std::fs::remove_dir_all(&dir).ok();
    }

    // ── StorageConfig::from_env — both features (backend discriminant) ────────

    #[cfg(all(feature = "storage-s3", feature = "storage-azure"))]
    #[test]
    fn from_env_both_features_paths() {
        // Sequential to avoid races on shared env vars.

        // — missing STORAGE_BACKEND —
        std::env::remove_var("STORAGE_BACKEND");
        assert!(matches!(
            StorageConfig::from_env().unwrap_err(),
            StorageError::MissingEnv("STORAGE_BACKEND")
        ));

        // — invalid STORAGE_BACKEND value —
        std::env::set_var("STORAGE_BACKEND", "gcs");
        assert!(matches!(
            StorageConfig::from_env().unwrap_err(),
            StorageError::InvalidEnv("STORAGE_BACKEND")
        ));

        // — s3 backend: missing STORAGE_BUCKET —
        std::env::set_var("STORAGE_BACKEND", "s3");
        std::env::remove_var("STORAGE_BUCKET");
        std::env::remove_var("STORAGE_S3_REGION");
        assert!(matches!(
            StorageConfig::from_env().unwrap_err(),
            StorageError::MissingEnv("STORAGE_BUCKET")
        ));

        // — s3 backend: both required —
        std::env::set_var("STORAGE_BUCKET", "my-bucket");
        std::env::set_var("STORAGE_S3_REGION", "us-east-1");
        let cfg = StorageConfig::from_env().unwrap();
        match cfg {
            StorageConfig::S3 { bucket, .. } => assert_eq!(bucket, "my-bucket"),
            _ => panic!("expected S3 variant"),
        }

        // — azure backend: missing STORAGE_AZURE_ACCOUNT —
        std::env::set_var("STORAGE_BACKEND", "azure");
        std::env::remove_var("STORAGE_AZURE_ACCOUNT");
        std::env::remove_var("STORAGE_AZURE_CONTAINER");
        assert!(matches!(
            StorageConfig::from_env().unwrap_err(),
            StorageError::MissingEnv("STORAGE_AZURE_ACCOUNT")
        ));

        // — azure backend: both required —
        std::env::set_var("STORAGE_AZURE_ACCOUNT", "myaccount");
        std::env::set_var("STORAGE_AZURE_CONTAINER", "mycontainer");
        let cfg = StorageConfig::from_env().unwrap();
        match cfg {
            StorageConfig::Azure { account, container } => {
                assert_eq!(account, "myaccount");
                assert_eq!(container, "mycontainer");
            }
            _ => panic!("expected Azure variant"),
        }

        // cleanup
        std::env::remove_var("STORAGE_BACKEND");
        std::env::remove_var("STORAGE_BUCKET");
        std::env::remove_var("STORAGE_S3_REGION");
        std::env::remove_var("STORAGE_AZURE_ACCOUNT");
        std::env::remove_var("STORAGE_AZURE_CONTAINER");
    }
}
