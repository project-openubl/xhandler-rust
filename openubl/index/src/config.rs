#[derive(clap::Subcommand, Debug)]
pub enum SearchEngine {
    Local(LocalEngine),
}

#[derive(clap::Args, Debug)]
pub struct LocalEngine {
    #[arg(
        id = "search-engine-local-dir",
        long,
        env = "SEARCH_ENGINE_LOCAL_DIR",
        default_value = "index"
    )]
    pub index_dir: String,

    #[arg(
        id = "search-engine-local-sync-interval",
        long,
        env = "SEARCH_ENGINE_LOCAL_SYNC_INTERVAL",
        default_value = "30S"
    )]
    pub sync_interval: humantime::Duration,

    #[arg(
        id = "search-engine-local-writer-memory",
        long,
        env = "SEARCH_ENGINE_LOCAL_WRITER_MEMORY",
        default_value = "default_value_t = ByteSize::mb(256)"
    )]
    pub index_writer_memory_bytes: bytesize::ByteSize,
}
