#[derive(clap::Subcommand, Debug)]
pub enum Storage {
    Local(LocalStorage),
    Minio(MinioStorage),
    S3(S3Storage),
}

#[derive(clap::Args, Debug)]
pub struct LocalStorage {
    #[arg(
        id = "storage-local-dir-ubl",
        long,
        env = "STORAGE_LOCAL_DIR_UBL",
        default_value = "openubl_ubl"
    )]
    pub dir_ubl: String,
    #[arg(
        id = "storage-local-dir-index",
        long,
        env = "STORAGE_LOCAL_DIR_INDEX",
        default_value = "openubl_index"
    )]
    pub dir_index: String,
}

#[derive(clap::Args, Debug)]
pub struct MinioStorage {
    #[arg(id = "storage-minio-host", long, env = "STORAGE_MINIO_HOST")]
    pub host: String,

    #[arg(
        id = "storage-minio-bucket-ubl",
        long,
        env = "STORAGE_MINIO_BUCKET_UBL",
        default_value = "ubl"
    )]
    pub bucket_ubl: String,

    #[arg(
        id = "storage-minio-bucket-index",
        long,
        env = "STORAGE_MINIO_BUCKET_INDEX",
        default_value = "index"
    )]
    pub bucket_index: String,

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

#[derive(clap::Args, Debug)]
pub struct S3Storage {
    #[arg(id = "storage-s3-region", long, env = "STORAGE_S3_REGION")]
    pub region: String,

    #[arg(
        id = "storage-s3-bucket-ubl",
        long,
        env = "STORAGE_S3_BUCKET_UBL",
        default_value = "ubl"
    )]
    pub bucket_ubl: String,

    #[arg(
        id = "storage-s3-bucket-index",
        long,
        env = "STORAGE_S3_BUCKET_INDEX",
        default_value = "index"
    )]
    pub bucket_index: String,

    #[arg(id = "storage-s3-access-key", long, env = "STORAGE_S3_ACCESS_KEY")]
    pub access_key: String,

    #[arg(id = "storage-s3-secret-key", long, env = "STORAGE_S3_SECRET_KEY")]
    pub secret_key: String,
}
