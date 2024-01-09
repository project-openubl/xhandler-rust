use tantivy::{Index, IndexWriter, TantivyError};
use tantivy::directory::{MmapDirectory};

use crate::config::SearchEngine;
use crate::system::ubl_document::UblDocumentIndex;

mod ubl_document;

pub enum SearchEngineSystem {
    Local(IndexWriter),
}

#[derive(Debug, thiserror::Error)]
pub enum LocalIndexError {
    #[error(transparent)]
    Tantivy(TantivyError),
}

#[derive(Debug, thiserror::Error)]
pub enum IndexError {
    #[error(transparent)]
    Local(#[from] LocalIndexError),
}

impl From<TantivyError> for IndexError {
    fn from(e: TantivyError) -> Self {
        Self::Local(LocalIndexError::Tantivy(e))
    }
}

impl SearchEngineSystem {
    pub async fn new(config: &SearchEngine) -> anyhow::Result<Self> {
        match config {
            SearchEngine::Local(config) => {
                let ubl_document_index = UblDocumentIndex::new();

                let directory = MmapDirectory::open(&config.index_dir)?;
                let index = Index::open_or_create(directory, ubl_document_index.schema.clone())?;

                let index_writer = index.writer(config.index_writer_memory_bytes.as_u64() as usize)?;
                Ok(Self::Local(index_writer))
            }
        }
    }
}
