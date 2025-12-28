# API Reference

Complete API documentation for Snapshot Comparison.

## Structs

### SnapshotStore

Manages snapshot storage and operations.

```rust
pub struct SnapshotStore {
    base_dir: PathBuf,
}
```

#### Methods

##### new

```rust
pub fn new(base_dir: PathBuf) -> Self
```

Create a new snapshot store.

**Arguments:**
- `base_dir` - Directory to store snapshots

**Example:**
```rust
let store = SnapshotStore::new(PathBuf::from("snapshots"));
```

##### base_dir

```rust
pub fn base_dir(&self) -> &Path
```

Get the base directory for this store.

##### create

```rust
pub fn create(
    &self,
    name: &str,
    content: &[u8],
    config: &SnapshotConfig
) -> Result<Snapshot, SnapshotError>
```

Create a new snapshot.

**Arguments:**
- `name` - Snapshot name (no path separators)
- `content` - Content to store
- `config` - Configuration options

**Returns:**
- `Ok(Snapshot)` - The created snapshot
- `Err(SnapshotError)` - On failure

##### read

```rust
pub fn read(&self, name: &str) -> Result<Snapshot, SnapshotError>
```

Read a snapshot from disk.

##### check

```rust
pub fn check(
    &self,
    name: &str,
    content: &[u8],
    config: &SnapshotConfig
) -> Result<CompareResult, SnapshotError>
```

Compare content against a stored snapshot.

**Returns:**
- `CompareResult::Match` - Content matches
- `CompareResult::Mismatch { diff, .. }` - Content differs
- `CompareResult::Updated { .. }` - Updated (if `update_mode`)

##### update

```rust
pub fn update(
    &self,
    name: &str,
    content: &[u8],
    config: &SnapshotConfig
) -> Result<Snapshot, SnapshotError>
```

Update an existing snapshot.

##### delete

```rust
pub fn delete(&self, name: &str) -> Result<(), SnapshotError>
```

Delete a snapshot.

##### list

```rust
pub fn list(&self) -> Result<Vec<SnapshotMetadata>, SnapshotError>
```

List all snapshots.

##### clean

```rust
pub fn clean(
    &self,
    keep: &[&str],
    dry_run: bool
) -> Result<Vec<String>, SnapshotError>
```

Remove snapshots not in the keep list.

**Arguments:**
- `keep` - Names of snapshots to keep
- `dry_run` - If true, don't actually delete

**Returns:** Names of (would-be) deleted snapshots

---

### SnapshotConfig

Configuration for snapshot operations.

```rust
pub struct SnapshotConfig {
    pub file_type: Option<FileType>,
    pub color: bool,
    pub context_lines: usize,
    pub update_mode: bool,
}
```

#### Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `file_type` | `Option<FileType>` | `None` | File type hint (None = auto-detect) |
| `color` | `bool` | `false` | Enable colored diff output |
| `context_lines` | `usize` | `3` | Number of context lines in diffs |
| `update_mode` | `bool` | `false` | Auto-update on mismatch |

#### Default

```rust
impl Default for SnapshotConfig {
    fn default() -> Self {
        Self {
            file_type: None,
            color: false,
            context_lines: 3,
            update_mode: false,
        }
    }
}
```

---

### Snapshot

A stored snapshot with metadata and content.

```rust
pub struct Snapshot {
    pub metadata: SnapshotMetadata,
    pub content: Vec<u8>,
}
```

---

### SnapshotMetadata

Metadata about a snapshot.

```rust
pub struct SnapshotMetadata {
    pub name: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub content_hash: String,
    pub size: usize,
    pub file_type: FileType,
}
```

#### Fields

| Field | Type | Description |
|-------|------|-------------|
| `name` | `String` | Snapshot name |
| `created_at` | `u64` | Unix timestamp of creation |
| `updated_at` | `u64` | Unix timestamp of last update |
| `content_hash` | `String` | FNV-1a hash (hex string) |
| `size` | `usize` | Content size in bytes |
| `file_type` | `FileType` | Detected/specified file type |

---

## Enums

### CompareResult

Result of a snapshot comparison.

```rust
pub enum CompareResult {
    Match,
    Mismatch {
        snapshot: Snapshot,
        actual: Vec<u8>,
        diff: DiffOutput,
    },
    Updated {
        old_snapshot: Snapshot,
        new_content: Vec<u8>,
    },
}
```

#### Variants

##### Match

Content matches the stored snapshot exactly.

##### Mismatch

Content differs from the snapshot.

| Field | Type | Description |
|-------|------|-------------|
| `snapshot` | `Snapshot` | The stored snapshot |
| `actual` | `Vec<u8>` | The content being compared |
| `diff` | `DiffOutput` | Detailed diff result |

##### Updated

Snapshot was updated (only in `update_mode`).

| Field | Type | Description |
|-------|------|-------------|
| `old_snapshot` | `Snapshot` | Previous snapshot |
| `new_content` | `Vec<u8>` | New content stored |

#### Methods

```rust
impl CompareResult {
    pub fn matches(&self) -> bool;
    pub fn was_updated(&self) -> bool;
    pub fn diff(&self) -> Option<&DiffOutput>;
}
```

---

### FileType

Supported file types.

```rust
pub enum FileType {
    Text,
    Json,
    Binary,
}
```

Re-exported from `output_diffing_utility`.

---

### DiffOutput

Diff result from comparison.

```rust
pub enum DiffOutput {
    Text(TextDiffResult),
    Json(JsonDiffResult),
    Binary(BinaryDiffResult),
}
```

#### Methods

##### format

```rust
pub fn format(&self, snapshot_name: &str, color: bool) -> String
```

Format the diff as a human-readable string.

##### has_changes

```rust
pub fn has_changes(&self) -> bool
```

Check if there are any changes.

---

### SnapshotError

Error types for snapshot operations.

```rust
pub enum SnapshotError {
    NotFound(String),
    IoError(String),
    InvalidName(String),
    CorruptedSnapshot(String),
    InvalidUtf8(String),
}
```

#### Variants

| Variant | Description |
|---------|-------------|
| `NotFound(name)` | Snapshot does not exist |
| `IoError(msg)` | File system error |
| `InvalidName(name)` | Name contains forbidden characters |
| `CorruptedSnapshot(msg)` | Snapshot file is corrupted |
| `InvalidUtf8(msg)` | Content is not valid UTF-8 |

#### Traits

- `std::error::Error`
- `std::fmt::Display`
- `std::fmt::Debug`
- `Clone`
- `PartialEq`
- `Eq`

---

## Functions

### hash_content

```rust
pub fn hash_content(content: &[u8]) -> u64
```

Compute FNV-1a hash of content.

### format_hash

```rust
pub fn format_hash(hash: u64) -> String
```

Format hash as 16-character hex string.

---

## Re-exports

From `output_diffing_utility`:

```rust
pub use output_diffing_utility::FileType;
```

---

## Constants

### Validation

- Maximum snapshot name length: 255 characters
- Forbidden characters: `/`, `\`, `..`, `\0`

### File Format

- Extension: `.snap`
- Header delimiter: `---`
- Header prefix: `# `
