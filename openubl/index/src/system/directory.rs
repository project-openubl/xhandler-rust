use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use tantivy::directory::MmapDirectory;
use tantivy::schema::Schema;
use tantivy::tokenizer::TokenizerManager;
use tantivy::{Index, IndexSettings};

use crate::system::state::IndexState;
use crate::Error;

/// Represents state of the index on disk and managing index swaps.
#[derive(Debug)]
pub struct IndexDirectory {
    path: PathBuf,
    state: IndexState,
    digest: Vec<u8>,
}

impl IndexDirectory {
    /// Attempt to build a new index from the serialized zstd data
    pub fn sync(
        &mut self,
        schema: Schema,
        settings: IndexSettings,
        tokenizers: TokenizerManager,
        data: &[u8],
    ) -> Result<Option<Index>, Error> {
        let digest = Sha256::digest(data).to_vec();
        if self.digest != digest {
            let next = self.state.next();
            let path = next.directory(&self.path);
            let index = self.unpack(schema, settings, tokenizers, data, &path)?;
            self.state = next;
            self.digest = digest;
            Ok(Some(index))
        } else {
            Ok(None)
        }
    }

    pub fn new(path: &PathBuf) -> Result<IndexDirectory, Error> {
        if path.exists() {
            std::fs::remove_dir_all(path).map_err(|e| Error::Open(e.to_string()))?;
        }
        std::fs::create_dir_all(path).map_err(|e| Error::Open(e.to_string()))?;
        let state = IndexState::A;
        Ok(Self {
            digest: Vec::new(),
            path: path.clone(),
            state,
        })
    }

    pub fn reset(
        &mut self,
        settings: IndexSettings,
        schema: Schema,
        tokenizers: TokenizerManager,
    ) -> Result<Index, Error> {
        let next = self.state.next();
        let path = next.directory(&self.path);
        if path.exists() {
            std::fs::remove_dir_all(&path).map_err(|e| Error::Open(e.to_string()))?;
        }
        std::fs::create_dir_all(&path).map_err(|e| Error::Open(e.to_string()))?;
        let index = self.build_new(settings, schema, tokenizers, &path)?;
        self.state = next;
        Ok(index)
    }

    fn build_new(
        &self,
        settings: IndexSettings,
        schema: Schema,
        tokenizers: TokenizerManager,
        path: &Path,
    ) -> Result<Index, Error> {
        std::fs::create_dir_all(path).map_err(|e| Error::Open(e.to_string()))?;
        let dir = MmapDirectory::open(path).map_err(|e| Error::Open(e.to_string()))?;
        let builder = Index::builder()
            .schema(schema)
            .settings(settings)
            .tokenizers(tokenizers);
        let index = builder
            .open_or_create(dir)
            .map_err(|e| Error::Open(e.to_string()))?;
        Ok(index)
    }

    pub fn build(
        &self,
        settings: IndexSettings,
        schema: Schema,
        tokenizers: TokenizerManager,
    ) -> Result<Index, Error> {
        let path = self.state.directory(&self.path);
        self.build_new(settings, schema, tokenizers, &path)
    }

    fn unpack(
        &mut self,
        schema: Schema,
        settings: IndexSettings,
        tokenizers: TokenizerManager,
        data: &[u8],
        path: &Path,
    ) -> Result<Index, Error> {
        if path.exists() {
            std::fs::remove_dir_all(path).map_err(|e| Error::Open(e.to_string()))?;
        }
        std::fs::create_dir_all(path).map_err(|e| Error::Open(e.to_string()))?;

        let dec = zstd::stream::Decoder::new(data).map_err(Error::Io)?;
        let mut archive = tar::Archive::new(dec);
        archive.unpack(path).map_err(Error::Io)?;
        log::trace!("Unpacked into {:?}", path);

        let dir = MmapDirectory::open(path).map_err(|e| Error::Open(e.to_string()))?;
        let builder = Index::builder()
            .schema(schema)
            .settings(settings)
            .tokenizers(tokenizers);
        let inner = builder
            .open_or_create(dir)
            .map_err(|e| Error::Open(e.to_string()))?;
        Ok(inner)
    }

    pub fn pack(&mut self) -> Result<Vec<u8>, Error> {
        let path = self.state.directory(&self.path);
        let mut out = Vec::new();
        let enc = zstd::stream::Encoder::new(&mut out, 3).map_err(Error::Io)?;
        let mut archive = tar::Builder::new(enc.auto_finish());
        log::trace!("Packing from {:?}", path);
        archive.append_dir_all("", path).map_err(Error::Io)?;
        drop(archive);
        Ok(out)
    }
}
