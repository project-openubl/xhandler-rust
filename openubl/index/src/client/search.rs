use log::{debug, warn};
use tantivy::collector::TopDocs;
use tantivy::query::Query;
use tantivy::schema::{Field, Type};
use tantivy::{DateTime, DocAddress, Order, Searcher};

use crate::client::write::WriteIndex;
use crate::search_options::SearchOptions;
use crate::{Error, IndexStore};

/// A search query.
#[derive(Debug)]
pub struct SearchQuery {
    /// The tantivy query to execute.
    pub query: Box<dyn Query>,
    /// A custom sort order to apply to the results.
    pub sort_by: Option<(Field, Order)>,
}

/// Defines the interface for an index that can be searched.
pub trait SearchableIndex: WriteIndex {
    /// Type of the matched document returned from a search.
    type MatchedDocument: core::fmt::Debug;

    /// Prepare a query for searching and return a query object.
    fn prepare_query(&self, q: &str) -> Result<SearchQuery, Error>;

    /// Search the index for a query and return a list of matched documents.
    fn search(
        &self,
        searcher: &Searcher,
        query: &dyn Query,
        offset: usize,
        limit: usize,
    ) -> Result<(Vec<(f32, DocAddress)>, usize), Error>;

    /// Invoked for every matched document to process the document and return a result.
    fn process_hit(
        &self,
        doc: DocAddress,
        score: f32,
        searcher: &Searcher,
        query: &dyn Query,
        options: &SearchOptions,
    ) -> Result<Self::MatchedDocument, Error>;
}

impl<INDEX: SearchableIndex> IndexStore<INDEX> {
    /// Search the index for a given query and return matching documents.
    pub fn search(
        &self,
        q: &str,
        offset: usize,
        limit: usize,
        options: SearchOptions,
    ) -> Result<(Vec<INDEX::MatchedDocument>, usize), Error> {
        if limit == 0 {
            return Err(Error::InvalidLimitParameter(limit));
        }

        let inner = self.inner.read().unwrap();
        let reader = inner.reader()?;
        let searcher = reader.searcher();

        let query = self.index.prepare_query(q)?;

        log::trace!("Processed query: {:?}", query);

        let (top_docs, count) = if let Some(sort_by) = query.sort_by {
            let field = sort_by.0;
            let order = sort_by.1;
            let order_by_str = self.index.schema().get_field_name(field).to_string();
            let vtype = self
                .index
                .schema()
                .get_field_entry(field)
                .field_type()
                .value_type();
            let mut hits = Vec::new();
            let total = match vtype {
                Type::U64 => {
                    let result = searcher.search(
                        &query.query,
                        &(
                            TopDocs::with_limit(limit)
                                .and_offset(offset)
                                .order_by_fast_field::<u64>(&order_by_str, order.clone()),
                            tantivy::collector::Count,
                        ),
                    )?;
                    for r in result.0 {
                        hits.push((1.0, r.1));
                    }
                    result.1
                }
                Type::I64 => {
                    let result = searcher.search(
                        &query.query,
                        &(
                            TopDocs::with_limit(limit)
                                .and_offset(offset)
                                .order_by_fast_field::<i64>(&order_by_str, order.clone()),
                            tantivy::collector::Count,
                        ),
                    )?;
                    for r in result.0 {
                        hits.push((1.0, r.1));
                    }
                    result.1
                }
                Type::F64 => {
                    let result = searcher.search(
                        &query.query,
                        &(
                            TopDocs::with_limit(limit)
                                .and_offset(offset)
                                .order_by_fast_field::<f64>(&order_by_str, order.clone()),
                            tantivy::collector::Count,
                        ),
                    )?;
                    for r in result.0 {
                        hits.push((1.0, r.1));
                    }
                    result.1
                }
                Type::Bool => {
                    let result = searcher.search(
                        &query.query,
                        &(
                            TopDocs::with_limit(limit)
                                .and_offset(offset)
                                .order_by_fast_field::<bool>(&order_by_str, order.clone()),
                            tantivy::collector::Count,
                        ),
                    )?;
                    for r in result.0 {
                        hits.push((1.0, r.1));
                    }
                    result.1
                }
                Type::Date => {
                    let result = searcher.search(
                        &query.query,
                        &(
                            TopDocs::with_limit(limit)
                                .and_offset(offset)
                                .order_by_fast_field::<DateTime>(&order_by_str, order.clone()),
                            tantivy::collector::Count,
                        ),
                    )?;
                    for r in result.0 {
                        hits.push((1.0, r.1));
                    }
                    result.1
                }
                _ => return Err(Error::NotSortable(order_by_str)),
            };
            (hits, total)
        } else {
            self.index.search(&searcher, &query.query, offset, limit)?
        };

        log::info!("#matches={count} for query '{q}'");

        if options.summaries {
            let mut hits = Vec::new();
            for hit in top_docs {
                match self
                    .index
                    .process_hit(hit.1, hit.0, &searcher, &query.query, &options)
                {
                    Ok(value) => {
                        debug!("HIT: {:?}", value);
                        hits.push(value);
                    }
                    Err(e) => {
                        warn!("Error processing hit {:?}: {:?}", hit, e);
                    }
                }
            }

            debug!("Filtered to {}", hits.len());

            Ok((hits, count))
        } else {
            Ok((Vec::new(), count))
        }
    }
}
