use std::fs;
use std::fs::{rename, File};
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;

use anyhow::anyhow;
use minio::s3::args::UploadObjectArgs;
use minio::s3::client::Client;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use uuid::Uuid;
use zip::result::{ZipError, ZipResult};
use zip::write::FileOptions;
use zip::ZipWriter;

use crate::config::Storage;

pub mod config;

pub enum StorageSystem {
    FileSystem(String),
    Minio(String, Client),
}

#[derive(Debug, thiserror::Error)]
pub enum StorageSystemErr {
    #[error(transparent)]
    Filesystem(std::io::Error),
    #[error(transparent)]
    Minio(minio::s3::error::Error),
    #[error(transparent)]
    Zip(zip::result::ZipError),
}

impl From<std::io::Error> for StorageSystemErr {
    fn from(e: std::io::Error) -> Self {
        Self::Filesystem(e)
    }
}

impl From<minio::s3::error::Error> for StorageSystemErr {
    fn from(e: minio::s3::error::Error) -> Self {
        Self::Minio(e)
    }
}

impl From<zip::result::ZipError> for StorageSystemErr {
    fn from(e: zip::result::ZipError) -> Self {
        Self::Zip(e)
    }
}

impl StorageSystem {
    pub fn new(config: &Storage) -> anyhow::Result<Self> {
        match config.storage_type.as_str() {
            "filesystem" => Ok(Self::FileSystem(config.filesystem.dir.clone())),
            "minio" => {
                let static_provider =
                    StaticProvider::new(&config.minio.access_key, &config.minio.secret_key, None);
                let client = Client::new(
                    BaseUrl::from_str(&config.minio.host)?,
                    Some(Box::new(static_provider)),
                    None,
                    None,
                )?;
                Ok(Self::Minio(config.minio.bucket.clone(), client))
            }
            _ => Err(anyhow!("Not supported storage type")),
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
            StorageSystem::FileSystem(workspace) => {
                let object_name = Path::new(workspace)
                    .join(project_id.to_string())
                    .join(ruc)
                    .join(document_type)
                    .join(&zip_name);

                rename(zip_path, object_name)?;
                Ok(zip_name.clone())
            }
            StorageSystem::Minio(bucket, client) => {
                let object_name = format!("{project_id}/{ruc}/{document_type}/{zip_name}");

                let object = &UploadObjectArgs::new(bucket, &object_name, &zip_path)?;
                let response = client.upload_object(object).await?;

                // Clear temp files
                fs::remove_file(file_full_path)?;
                fs::remove_file(zip_path)?;

                Ok(response.object_name)
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
