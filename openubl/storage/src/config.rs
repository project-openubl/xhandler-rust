#[derive(clap::Args, Debug)]
pub struct Storage {
    #[arg(id = "storage-type", long, env = "STORAGE_TYPE", default_value = "filesystem", value_parser=["filesystem", "minio"])]
    pub storage_type: String,
    #[command(flatten)]
    pub filesystem: FilesystemStorage,
    #[command(flatten)]
    pub minio: MinioStorage,
}

#[derive(clap::Args, Debug)]
pub struct FilesystemStorage {
    #[arg(
        id = "storage-filesystem-dir",
        long,
        env = "STORAGE_FILESYSTEM_DIR",
        default_value = "workspace"
    )]
    pub dir: String,
}

#[derive(clap::Args, Debug)]
pub struct MinioStorage {
    #[arg(id = "storage-minio-host", long, env = "STORAGE_MINIO_HOST")]
    pub host: String,
    #[arg(
        id = "storage-minio-bucket",
        long,
        env = "STORAGE_MINIO_BUCKET",
        default_value = "openubl"
    )]
    pub bucket: String,
    #[arg(
        id = "storage-minio-access-key",
        long,
        env = "STORAGE_MINIO_ACCESS_KEY"
    )]
    pub access_key: String,
    #[arg(
        id = "storage-minio-secret-key",
        long,
        env = "STORAGE_MINIO_SECRET_KEY"
    )]
    pub secret_key: String,
}
