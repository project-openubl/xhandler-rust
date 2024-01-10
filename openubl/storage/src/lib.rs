use std::fs;
use std::fs::{rename, File};
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;

use aws_config::meta::region::RegionProviderChain;
use aws_config::retry::RetryConfig;
use aws_config::BehaviorVersion;
use aws_sdk_s3::client::Client as S3Client;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::get_object::GetObjectError;
use aws_sdk_s3::operation::put_object::PutObjectError;
use aws_sdk_s3::primitives::{ByteStream, ByteStreamError};
use aws_smithy_runtime_api::client::orchestrator::HttpResponse;
use minio::s3::args::UploadObjectArgs;
use minio::s3::client::Client as MinioClient;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use uuid::Uuid;
use zip::result::{ZipError, ZipResult};
use zip::write::FileOptions;
use zip::ZipWriter;

use crate::config::Storage;

pub mod config;

pub struct LocalDir {
    path: String,
}

pub struct Bucket {
    name: String,
}

pub enum StorageSystem {
    Local(LocalDir),
    Minio(Bucket, MinioClient),
    S3(Bucket, S3Client),
}

#[derive(Debug, thiserror::Error)]
pub enum StorageSystemErr {
    #[error(transparent)]
    Local(std::io::Error),

    #[error(transparent)]
    Minio(MinioError),

    #[error(transparent)]
    S3(#[from] S3Error),

    #[error(transparent)]
    Zip(ZipError),

    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum MinioError {
    #[error(transparent)]
    Sdk(minio::s3::error::Error),
    #[error(transparent)]
    HttpClient(reqwest::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum S3Error {
    #[error(transparent)]
    Stream(ByteStreamError),

    #[error(transparent)]
    Get(SdkError<GetObjectError, HttpResponse>),

    #[error(transparent)]
    Put(SdkError<PutObjectError, HttpResponse>),
}

impl From<std::io::Error> for StorageSystemErr {
    fn from(e: std::io::Error) -> Self {
        Self::Local(e)
    }
}

impl From<minio::s3::error::Error> for StorageSystemErr {
    fn from(e: minio::s3::error::Error) -> Self {
        Self::Minio(MinioError::Sdk(e))
    }
}

impl From<reqwest::Error> for StorageSystemErr {
    fn from(e: reqwest::Error) -> Self {
        Self::Minio(MinioError::HttpClient(e))
    }
}

impl From<ByteStreamError> for StorageSystemErr {
    fn from(e: ByteStreamError) -> Self {
        Self::S3(S3Error::Stream(e))
    }
}

impl From<SdkError<GetObjectError, HttpResponse>> for StorageSystemErr {
    fn from(e: SdkError<GetObjectError, HttpResponse>) -> Self {
        Self::S3(S3Error::Get(e))
    }
}

impl From<SdkError<PutObjectError, HttpResponse>> for StorageSystemErr {
    fn from(e: SdkError<PutObjectError, HttpResponse>) -> Self {
        Self::S3(S3Error::Put(e))
    }
}

impl From<ZipError> for StorageSystemErr {
    fn from(e: ZipError) -> Self {
        Self::Zip(e)
    }
}

impl StorageSystem {
    pub async fn new(config: &Storage) -> anyhow::Result<Self> {
        match config {
            Storage::Local(config) => Ok(Self::Local(LocalDir {
                path: config.local_dir.clone(),
            })),
            Storage::Minio(config) => {
                let static_provider =
                    StaticProvider::new(&config.access_key, &config.secret_key, None);
                let client = MinioClient::new(
                    BaseUrl::from_str(&config.host)?,
                    Some(Box::new(static_provider)),
                    None,
                    None,
                )?;
                Ok(Self::Minio(
                    Bucket {
                        name: config.bucket.clone(),
                    },
                    client,
                ))
            }
            Storage::S3(config) => {
                let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
                let credentials_provider = aws_sdk_s3::config::Credentials::new(
                    &config.access_key,
                    &config.secret_key,
                    Some("atestsessiontoken".to_string()),
                    None,
                    "",
                );
                let sdk_config = aws_config::defaults(BehaviorVersion::latest())
                    .region(region_provider)
                    .credentials_provider(credentials_provider)
                    .retry_config(RetryConfig::standard().with_max_attempts(3))
                    .load()
                    .await;

                let client = S3Client::new(&sdk_config);
                Ok(Self::S3(
                    Bucket {
                        name: config.bucket.clone(),
                    },
                    client,
                ))
            }
        }
    }

    pub async fn upload_ubl_xml(
        &self,
        project_id: i32,
        ruc: &str,
        document_type: &str,
        document_id: &str,
        file_sha246: &str,
        file_full_path: &str,
    ) -> Result<String, StorageSystemErr> {
        let file_name_inside_zip = format!("{ruc}-{}.xml", document_id.to_uppercase());
        let zip_path = zip_file(file_full_path, &file_name_inside_zip)?;

        let short_sha256: String = file_sha246.chars().take(7).collect();
        let zip_name = format!("{}_{short_sha256}.zip", document_id.to_uppercase());

        match self {
            StorageSystem::Local(directories) => {
                let object_name = Path::new(&directories.path)
                    .join(project_id.to_string())
                    .join(ruc)
                    .join(document_type)
                    .join(&zip_name);

                rename(zip_path, object_name)?;
                Ok(zip_name.clone())
            }
            StorageSystem::Minio(bucket, client) => {
                let object_name = format!("{project_id}/{ruc}/{document_type}/{zip_name}");

                let object = &UploadObjectArgs::new(&bucket.name, &object_name, &zip_path)?;
                let response = client.upload_object(object).await?;

                // Clear temp files
                fs::remove_file(file_full_path)?;
                fs::remove_file(zip_path)?;

                Ok(response.object_name)
            }
            StorageSystem::S3(buckets, client) => {
                let object_name = format!("{project_id}/{ruc}/{document_type}/{zip_name}");
                let object = ByteStream::from_path(&zip_path).await?;

                client
                    .put_object()
                    .bucket(&buckets.name)
                    .key(&object_name)
                    .body(object)
                    .send()
                    .await?;

                // Clear temp files
                fs::remove_file(file_full_path)?;
                fs::remove_file(zip_path)?;

                Ok(object_name)
            }
        }
    }
}

pub fn zip_file(
    full_path_of_file_to_be_zipped: &str,
    file_name_to_be_used_in_zip: &str,
) -> ZipResult<String> {
    let zip_filename = format!("{}.zip", Uuid::new_v4());

    let mut file = File::open(full_path_of_file_to_be_zipped)?;
    let file_path = Path::new(full_path_of_file_to_be_zipped);
    let file_directory = file_path.parent().ok_or(ZipError::InvalidArchive(
        "Could not find the parent folder of given file",
    ))?;

    let zip_path = file_directory.join(zip_filename);
    let zip_file = File::create(zip_path.as_path())?;
    let mut zip = ZipWriter::new(zip_file);

    let file_options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Bzip2)
        .unix_permissions(0o755);

    zip.start_file(file_name_to_be_used_in_zip, file_options)?;

    let mut buff = Vec::new();
    file.read_to_end(&mut buff)?;
    zip.write_all(&buff)?;

    zip.finish()?;

    let result = zip_path.to_str().ok_or(ZipError::InvalidArchive(
        "Could not determine with zip filename",
    ))?;
    Ok(result.to_string())
}
