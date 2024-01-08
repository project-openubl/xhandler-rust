use std::fmt::Display;

#[derive(clap::Args, Debug)]
pub struct SearchEngine {
    /// Synchronization interval for index persistence.
    #[arg(
    id = "search-engine-mode",
    long,
    env = "SEARCH_ENGINE_MODE",
    default_value_t = IndexMode::File
    )]
    pub mode: IndexMode,

    #[arg(
        id = "search-engine-sync-interval",
        long,
        env = "SEARCH_ENGINE_SYNC_INTERVAL",
        default_value = "30S"
    )]
    pub sync_interval: humantime::Duration,
    #[arg(
        id = "search-engine-writer-memory",
        long,
        env = "SEARCH_ENGINE_WRITER_MEMORY",
        default_value = "default_value_t = ByteSize::mb(256)"
    )]
    pub index_writer_memory_bytes: bytesize::ByteSize,

    #[arg(
        id = "search-engine-dir",
        long,
        env = "SEARCH_ENGINE_DIR",
        default_value = "indexes"
    )]
    pub index_dir: Option<std::path::PathBuf>,
    #[arg(
        id = "search-engine-dir",
        long,
        env = "SEARCH_ENGINE_BUCKET",
        default_value = "openubl-indexes"
    )]
    pub bucket: String,
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum IndexMode {
    File,
    S3,
}

impl Display for IndexMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File => write!(f, "file"),
            Self::S3 => write!(f, "s3"),
        }
    }
}

impl Default for IndexMode {
    fn default() -> Self {
        Self::File
    }
}
