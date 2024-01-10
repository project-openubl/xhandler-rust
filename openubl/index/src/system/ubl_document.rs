use tantivy::schema::{Field, Schema, STORED, TEXT};
use tantivy::Document;

use crate::system::{IndexError, SearchEngineSystem};

pub(crate) struct UblDocumentIndex {
    pub(crate) schema: Schema,
    pub(crate) fields: UblDocumentIndexFields,
}

pub(crate) struct UblDocumentIndexFields {
    id: Field,
    project_id: Field,
    document_type: Field,
    document_id: Field,
    supplier_id: Field,
}

impl UblDocumentIndex {
    pub(crate) fn new() -> Self {
        let mut schema = Schema::builder();
        let fields = UblDocumentIndexFields {
            id: schema.add_text_field("id", TEXT | STORED),
            project_id: schema.add_text_field("project_id", TEXT | STORED),
            document_type: schema.add_text_field("document_type", TEXT | STORED),
            document_id: schema.add_text_field("document_id", TEXT | STORED),
            supplier_id: schema.add_text_field("supplier_id", TEXT | STORED),
        };

        Self {
            schema: schema.build(),
            fields,
        }
    }
}

pub struct UblDocumentIndexModel {
    id: i32,
    project_id: i32,
    document_type: String,
    document_id: String,
    supplier_id: String,
}

impl SearchEngineSystem {
    pub async fn index_ubl_document(
        &mut self,
        model: &UblDocumentIndexModel,
    ) -> Result<(), IndexError> {
        match self {
            SearchEngineSystem::Local(ref mut index_writer) => {
                let ubl_document_index = UblDocumentIndex::new();
                let fields = ubl_document_index.fields;

                let mut document = Document::default();
                document.add_u64(fields.id, model.id as u64);
                document.add_u64(fields.project_id, model.project_id as u64);
                document.add_text(fields.document_type, &model.document_type);
                document.add_text(fields.document_id, &model.document_id);
                document.add_text(fields.supplier_id, &model.supplier_id);

                index_writer.add_document(document)?;
                index_writer.commit()?;

                Ok(())
            }
        }
    }
}
