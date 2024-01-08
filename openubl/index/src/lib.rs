use std::sync::RwLock;

use sha2::Digest;
use tantivy::query::Query;
use tantivy::{Directory, Index};

use client::write::WriteIndex;
use openubl_storage::StorageSystemErr;

use crate::system::directory::IndexDirectory;

mod client;
pub mod config;
mod search_options;
mod system;

/// A search index. This is a wrapper around the tantivy index that handles loading and storing of the index to object storage (via the local filesystem).
///
/// The index can be live-loaded and stored to/from object storage while serving queries.
pub struct IndexStore<INDEX> {
    inner: RwLock<Index>,
    index_dir: Option<RwLock<IndexDirectory>>,
    index: INDEX,
    index_writer_memory_bytes: usize,
}

/// Errors returned by the index.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error opening index {0}")]
    Open(String),
    #[error("error taking snapshot of index")]
    Snapshot,
    #[error("value for field {0} not found")]
    FieldNotFound(String),
    #[error("field {0} cannot be sorted")]
    NotSortable(String),
    #[error("operation cannot be done because index is not persisted")]
    NotPersisted,
    #[error("error parsing document {0}")]
    DocParser(String),
    #[error("error parsing query {0}")]
    QueryParser(String),
    #[error("error from storage {0}")]
    Storage(StorageSystemErr),
    #[error("invalid limit parameter {0}")]
    InvalidLimitParameter(usize),
    #[error("error from search {0}")]
    Search(tantivy::TantivyError),
    #[error("I/O error {0}")]
    Io(std::io::Error),
}

impl From<tantivy::TantivyError> for Error {
    fn from(e: tantivy::TantivyError) -> Self {
        Self::Search(e)
    }
}

impl From<StorageSystemErr> for Error {
    fn from(e: StorageSystemErr) -> Self {
        Self::Storage(e)
    }
}
