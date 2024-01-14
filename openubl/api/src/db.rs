use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbBackend, DbErr, ExecResult,
    QueryResult, Statement,
};
use serde::Deserialize;

#[derive(Copy, Clone)]
pub enum Transactional<'db> {
    None,
    Some(&'db DatabaseTransaction),
}

impl<'db> From<&'db DatabaseTransaction> for Transactional<'db> {
    fn from(inner: &'db DatabaseTransaction) -> Self {
        Self::Some(inner)
    }
}

#[derive(Clone)]
pub enum ConnectionOrTransaction<'db> {
    Connection(&'db DatabaseConnection),
    Transaction(&'db DatabaseTransaction),
}

#[async_trait::async_trait]
impl ConnectionTrait for ConnectionOrTransaction<'_> {
    fn get_database_backend(&self) -> DbBackend {
        match self {
            ConnectionOrTransaction::Connection(inner) => inner.get_database_backend(),
            ConnectionOrTransaction::Transaction(inner) => inner.get_database_backend(),
        }
    }

    async fn execute(&self, stmt: Statement) -> Result<ExecResult, DbErr> {
        match self {
            ConnectionOrTransaction::Connection(inner) => inner.execute(stmt).await,
            ConnectionOrTransaction::Transaction(inner) => inner.execute(stmt).await,
        }
    }

    async fn execute_unprepared(&self, sql: &str) -> Result<ExecResult, DbErr> {
        match self {
            ConnectionOrTransaction::Connection(inner) => inner.execute_unprepared(sql).await,
            ConnectionOrTransaction::Transaction(inner) => inner.execute_unprepared(sql).await,
        }
    }

    async fn query_one(&self, stmt: Statement) -> Result<Option<QueryResult>, DbErr> {
        match self {
            ConnectionOrTransaction::Connection(inner) => inner.query_one(stmt).await,
            ConnectionOrTransaction::Transaction(inner) => inner.query_one(stmt).await,
        }
    }

    async fn query_all(&self, stmt: Statement) -> Result<Vec<QueryResult>, DbErr> {
        match self {
            ConnectionOrTransaction::Connection(inner) => inner.query_all(stmt).await,
            ConnectionOrTransaction::Transaction(inner) => inner.query_all(stmt).await,
        }
    }
}

#[derive(Deserialize)]
pub struct Paginated {
    pub limit: u64,
    pub offset: u64,
}

impl Default for Paginated {
    fn default() -> Self {
        Paginated {
            offset: 0,
            limit: 10,
        }
    }
}

impl Paginated {
    pub fn page_number(&self) -> u64 {
        self.offset * self.limit
    }

    pub fn page_size(&self) -> u64 {
        self.limit
    }
}

pub struct PaginatedResults<R> {
    pub items: Vec<R>,
    pub num_items: u64,
}
