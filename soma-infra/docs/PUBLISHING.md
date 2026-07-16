# Publishing soma-infra to crates.io

soma-infra is a single Cargo package (not a workspace) with no intra-platform
crate dependencies, so there is only one publish step. The metadata, license,
README, and docs.rs configuration are already committed. `cargo publish` is the
complete workflow.

## Version history

| version | notes |
|---------|-------|
| `0.1.0` | Initial publish — all 13 features, 46 unit tests, golden-vector crypto tests. |
| `0.1.1` | Adds `[package.metadata.docs.rs] all-features = true` so docs.rs documents every feature (not just the default `db` + `tracing`). No API changes. |

## Prerequisites

You must be logged in to crates.io:

```sh
cargo login
# Paste your crates.io API token when prompted.
```

## Dry-run first

```sh
cd /path/to/soma-infra
cargo publish --dry-run
```

The dry-run verifies that the package builds from a clean source tree, all
`[package]` metadata is present, and the README and LICENSE files are
accessible. It does not upload anything.

Common dry-run failures:

- Uncommitted files in the working tree that are needed by the build — stage
  or stash first.
- A new dependency added to `Cargo.toml` but not yet committed to
  `Cargo.lock` — run `cargo build` first.
- `path =` dependencies that resolve to something not yet on crates.io — this
  crate has no path deps, so this does not apply.

## Publish

```sh
cargo publish
```

Wait for the upload to complete and the index to update (usually seconds).
The new version appears on [crates.io/crates/soma-infra](https://crates.io/crates/soma-infra)
and on [docs.rs/soma-infra](https://docs.rs/soma-infra) within minutes.

## docs.rs all-features

soma-infra's `Cargo.toml` includes:

```toml
[package.metadata.docs.rs]
all-features = true
```

This instructs docs.rs to build documentation with every feature enabled, so
`crypto`, `cache`, `storage-s3`, `storage-azure`, `llm`, `kg`, `http`, and
`signal` all appear in the public docs. Without this, only the default features
(`db`, `tracing`) would be documented — the other eleven modules would be
invisible.

This config was added in v0.1.1. If you are bumping from v0.1.0, make sure the
`[package.metadata.docs.rs]` table is committed before publishing.

## Bumping the version

1. Edit `version` in `Cargo.toml` (e.g. `"0.1.1"` → `"0.1.2"`).
2. Run `cargo build` to refresh `Cargo.lock` with the new version.
3. Commit both files.
4. Tag the commit: `git tag v0.1.2 && git push origin v0.1.2`.
5. Dry-run, then publish.

Follow [Semantic Versioning](https://semver.org):

- Patch (`0.1.x`) — bug fixes, additive doc changes, no API changes.
- Minor (`0.x.0`) — new public APIs, new features, backward-compatible.
- Major (`x.0.0`) — breaking API changes. At `0.x`, minor bumps may also
  include breaking changes (semver §4).

## Immutability warning

Published versions are **immutable** on crates.io. You cannot re-upload the
same version number, and you cannot edit or delete a published version (you can
only yank it, which prevents new `Cargo.lock` pins but does not remove it from
the registry).

Get the dry-run green and the code reviewed before publishing. If a mistake
slips through, publish a patch version.

## Consumers: crates.io vs path/git dep

Downstream consumers inside the soma-platform monorepo use a path dep during
active development:

```toml
soma-infra = { path = "../soma-infra", features = ["db", "tracing"] }
```

External consumers and pinned deployments use the crates.io form:

```toml
soma-infra = { version = "0.1", features = ["db", "tracing"] }
```

Both forms may coexist in the same `Cargo.toml` declaration. Cargo uses the
path locally and falls back to the registry version for non-path builds.

Note that the soma-platform services currently consume soma-infra via path dep
inside the monorepo. They do not pin to the published crates.io version. The
published version exists for external consumers and for the docs.rs build.
