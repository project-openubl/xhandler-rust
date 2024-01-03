use crate::database::DatabaseServer;
use crate::messaging::MessagingServer;
use crate::oidc::OidcServer;
use crate::storage::StorageServer;
use testcontainers::clients::Cli;

pub mod database;
pub mod messaging;
pub mod modules;
pub mod oidc;
pub mod storage;

pub enum TestProfile {
    Prod(Cli),
}

// impl TestProfile<'_> {
//     pub fn new_prod() -> Self {
//         Self::Prod(Cli::default())
//     }
// }
//
// impl ProdTestProfile<'_> {
//     pub fn new() -> Self {
//         Self {
//             database: DatabaseServer::new(),
//             oidc: OidcServer::new(),
//             messaging: MessagingServer::new(),
//             storage: StorageServer::new(),
//         }
//     }
// }
