use std::sync::RwLock;

use openubl_storage::StorageSystem;
use tantivy::directory::INDEX_WRITER_LOCK;
use tantivy::{Directory, Index};

use crate::client::write::WriteIndex;
use crate::config::{IndexMode, SearchEngine};
use crate::system::directory::IndexDirectory;
use crate::{Error, IndexStore};

/// A writer for an index that allows batching document writes before committing a batch.
///
/// Batching document writes can improve performance by reducing the number of commits to the index.
pub struct IndexWriter {
    writer: tantivy::IndexWriter,
}

impl IndexWriter {
    /// Add a document to the batch.
    pub fn add_document<DOC>(
        &mut self,
        index: &dyn WriteIndex<Document = DOC>,
        id: &str,
        data: &[u8],
    ) -> Result<(), Error> {
        self.add_document_with_id(index, data, id, |_| id.to_string())
    }

    /// Add a document with a given identifier to the batch.
    pub fn add_document_with_id<DOC, F>(
        &mut self,
        index: &dyn WriteIndex<Document = DOC>,
        data: &[u8],
        name: &str,
        id: F,
    ) -> Result<(), Error>
    where
        F: FnOnce(&DOC) -> String,
    {
        match index.parse_doc(data) {
            Ok(doc) => {
                let id = &id(&doc);
                let docs = index.index_doc(id, &doc)?;
                for (i, doc) in docs {
                    self.delete_document(index, &i);
                    self.writer.add_document(doc)?;
                }
            }
            Err(e) => {
                log::warn!("Error parsing document '{name}': {e:?}");
            }
        }

        Ok(())
    }

    /// Commit the batch and consume the writer. May merge index segments.
    pub fn commit(mut self) -> Result<(), Error> {
        self.writer.commit()?;
        self.writer.wait_merging_threads()?;
        Ok(())
    }

    /// Add a delete operation to the batch.
    pub fn delete_document<DOC>(&self, index: &dyn WriteIndex<Document = DOC>, key: &str) {
        let term = index.doc_id_to_term(key);
        self.writer.delete_term(term);
    }
}

impl<INDEX: WriteIndex> IndexStore<INDEX> {
    pub fn new_in_memory(index: INDEX) -> Result<Self, Error> {
        let schema = index.schema();
        let settings = index.settings();
        let tokenizers = index.tokenizers()?;
        let builder = Index::builder()
            .schema(schema)
            .settings(settings)
            .tokenizers(tokenizers);
        let inner = builder.create_in_ram()?;
        let name = index.name().to_string();
        Ok(Self {
            inner: RwLock::new(inner),
            index,
            index_writer_memory_bytes: 32 * 1024 * 1024,
            index_dir: None,
        })
    }

    pub fn new(config: &SearchEngine, index: INDEX) -> Result<Self, Error> {
        todo!()
        // match config.mode {
        //     IndexMode::File => {
        //         let path = config
        //             .index_dir
        //             .clone()
        //             .unwrap_or_else(|| {
        //                 use rand::RngCore;
        //                 let r = rand::thread_rng().next_u32();
        //                 std::env::temp_dir().join(format!("index.{}", r))
        //             })
        //             .join(index.name());
        //
        //         let schema = index.schema();
        //         let settings = index.settings();
        //         let tokenizers = index.tokenizers()?;
        //
        //         let index_dir = IndexDirectory::new(&path)?;
        //         let inner = index_dir.build(settings, schema, tokenizers)?;
        //         let name = index.name().to_string();
        //         Ok(Self {
        //             inner: RwLock::new(inner),
        //             index_writer_memory_bytes: config.index_writer_memory_bytes.as_u64() as usize,
        //             index_dir: Some(RwLock::new(index_dir)),
        //             index,
        //         })
        //     }
        //     IndexMode::S3 => {
        //         todo!("To be implemented");
        //         // let bucket = config.bucket.clone().try_into()?;
        //         // let schema = index.schema();
        //         // let settings = index.settings();
        //         // let tokenizers = index.tokenizers()?;
        //         // let builder = Index::builder()
        //         //     .schema(schema)
        //         //     .settings(settings)
        //         //     .tokenizers(tokenizers);
        //         // let dir = S3Directory::new(bucket);
        //         // let inner = builder.open_or_create(dir)?;
        //         // let name = index.name().to_string();
        //         // Ok(Self {
        //         //     inner: RwLock::new(inner),
        //         //     index_writer_memory_bytes: config.index_writer_memory_bytes.as_u64() as usize,
        //         //     index_dir: None,
        //         //     index,
        //         // })
        //     }
        // }
    }

    pub fn index(&self) -> &INDEX {
        &self.index
    }

    pub fn index_as_mut(&mut self) -> &mut INDEX {
        &mut self.index
    }

    /// Sync the index from a snapshot.
    ///
    /// NOTE: Only applicable for file indices.
    pub async fn sync(&self, storage: &StorageSystem) -> Result<(), Error> {
        if let Some(index_dir) = &self.index_dir {
            let data = storage.get_index(self.index.name()).await?;
            let mut index_dir = index_dir.write().unwrap();
            match index_dir.sync(
                self.index.schema(),
                self.index.settings(),
                self.index.tokenizers()?,
                &data,
            ) {
                Ok(Some(index)) => {
                    *self.inner.write().unwrap() = index;
                    log::debug!("Index reloaded");
                }
                Ok(None) => {
                    // No index change
                }
                Err(e) => {
                    log::warn!("Error syncing index: {:?}, keeping old", e);
                    return Err(e);
                }
            }
            log::debug!("Index reloaded");
        }
        Ok(())
    }

    // Reset the index to an empty state.
    pub fn reset(&mut self) -> Result<(), Error> {
        if let Some(index_dir) = &self.index_dir {
            let mut index_dir = index_dir.write().unwrap();
            let index = index_dir.reset(
                self.index.settings(),
                self.index.schema(),
                self.index.tokenizers()?,
            )?;
            let mut inner = self.inner.write().unwrap();
            *inner = index;
        }
        Ok(())
    }

    pub fn commit(&self, writer: IndexWriter) -> Result<(), Error> {
        writer.commit()?;
        Ok(())
    }

    /// Take a snapshot of the index and push to object storage.
    ///
    /// NOTE: Only applicable for file indices.
    ///
    ///
    // Disable the lint due to a [bug in clippy](https://github.com/rust-lang/rust-clippy/issues/6446).
    #[allow(clippy::await_holding_lock)]
    pub async fn snapshot(
        &mut self,
        writer: IndexWriter,
        storage: &StorageSystem,
        force: bool,
    ) -> Result<(), Error> {
        if let Some(index_dir) = &self.index_dir {
            writer.commit()?;

            let mut dir = index_dir.write().unwrap();
            let mut inner = self.inner.write().unwrap();
            inner.directory_mut().sync_directory().map_err(Error::Io)?;
            let lock = inner.directory_mut().acquire_lock(&INDEX_WRITER_LOCK);

            let managed_files = inner.directory().list_managed_files();

            let mut total_size: i64 = 0;
            for file in managed_files.iter() {
                log::trace!("Managed file: {:?}", file);
                let sz = std::fs::metadata(file).map(|m| m.len()).unwrap_or(0);
                total_size += sz as i64;
            }

            let gc_result = inner.directory_mut().garbage_collect(|| managed_files)?;
            log::trace!(
                "Gc result. Deleted: {:?}, failed: {:?}",
                gc_result.deleted_files,
                gc_result.failed_to_delete_files
            );
            let changed = !gc_result.deleted_files.is_empty();
            inner.directory_mut().sync_directory().map_err(Error::Io)?;
            if force || changed {
                log::info!("Index has changed, publishing new snapshot");
                let out = dir.pack()?;
                drop(lock);
                drop(inner);
                drop(dir);
                match storage.put_index(self.index.name(), &out).await {
                    Ok(_) => {
                        log::trace!("Snapshot published successfully");
                        Ok(())
                    }
                    Err(e) => {
                        log::warn!("Error updating index: {:?}", e);
                        Err(e.into())
                    }
                }
            } else {
                log::trace!("No changes to index");
                Ok(())
            }
        } else {
            log::trace!("Committing index");
            writer.commit()?;
            Ok(())
        }
    }

    pub fn writer(&mut self) -> Result<IndexWriter, Error> {
        let writer = self
            .inner
            .write()
            .unwrap()
            .writer(self.index_writer_memory_bytes)?;
        Ok(IndexWriter { writer })
    }
}
