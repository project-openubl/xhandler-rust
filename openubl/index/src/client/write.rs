use tantivy::schema::Schema;
use tantivy::tokenizer::TokenizerManager;
use tantivy::{Document, IndexSettings, Term};

use crate::Error;

/// Defines the interface for an index that can be written to.
pub trait WriteIndex {
    /// Input document type expected by the index.
    type Document;

    /// Name of the index. Must be unique across trait implementations.
    fn name(&self) -> &str;

    /// Tokenizers used by the index.
    fn tokenizers(&self) -> Result<TokenizerManager, Error> {
        Ok(TokenizerManager::default())
    }

    /// Parse a document from a byte slice.
    fn parse_doc(&self, data: &[u8]) -> Result<Self::Document, Error>;

    /// Index settings required for this index.
    fn settings(&self) -> IndexSettings;

    /// Schema required for this index.
    fn schema(&self) -> Schema;

    /// Process an input document and return a tantivy document to be added to the index.
    fn index_doc(
        &self,
        id: &str,
        document: &Self::Document,
    ) -> Result<Vec<(String, tantivy::Document)>, Error>;

    /// Convert a document id to a term for referencing that document.
    fn doc_id_to_term(&self, id: &str) -> Term;
}

impl<DOC> WriteIndex for Box<dyn WriteIndex<Document = DOC>> {
    type Document = DOC;
    fn name(&self) -> &str {
        self.as_ref().name()
    }

    fn tokenizers(&self) -> Result<TokenizerManager, Error> {
        self.as_ref().tokenizers()
    }

    fn parse_doc(&self, data: &[u8]) -> Result<Self::Document, Error> {
        self.as_ref().parse_doc(data)
    }

    fn settings(&self) -> IndexSettings {
        self.as_ref().settings()
    }

    fn schema(&self) -> Schema {
        self.as_ref().schema()
    }

    fn index_doc(
        &self,
        id: &str,
        document: &Self::Document,
    ) -> Result<Vec<(String, Document)>, Error> {
        self.as_ref().index_doc(id, document)
    }

    fn doc_id_to_term(&self, id: &str) -> Term {
        self.as_ref().doc_id_to_term(id)
    }
}
