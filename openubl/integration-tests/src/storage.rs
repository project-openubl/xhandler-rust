use std::collections::HashMap;
use std::fs::read_to_string;

use testcontainers::clients::Cli;
use testcontainers::core::{ExecCommand, WaitFor};
use testcontainers::Container;

use crate::modules::minio::Minio;

pub struct StorageServer<'a> {
    container: Container<'a, Minio>,
    properties: HashMap<String, String>,
}

impl<'a> StorageServer<'a> {
    pub fn new(docker: &Cli) -> StorageServer<'a> {
        let minio = docker.run(
            Minio::new("quay.io/minio", "latest")
                .with_env_var("MINIO_ROOT_USER", "admin")
                .with_env_var("MINIO_ROOT_PASSWORD", "password")
                .with_env_var("MINIO_ACCESS_KEY", "access-key")
                .with_env_var("MINIO_SECRET_KEY", "secret-key")
                .with_env_var("MINIO_NOTIFY_NATS_ENABLE_OPENUBL", "on")
                .with_env_var("MINIO_NOTIFY_NATS_ADDRESS_OPENUBL", "nats:4222")
                .with_env_var("MINIO_NOTIFY_NATS_SUBJECT_OPENUBL", "openubl"),
        );
        let port = minio.get_host_port_ipv4(9000).to_string();

        minio.exec(ExecCommand {
            cmd: read_to_string("resources/minio-setup.sh").expect("minio-setup.sh not found"),
            ready_conditions: vec![WaitFor::Nothing],
        });

        let properties = HashMap::from([
            (
                "--storage-minio-host".to_string(),
                format!("http://localhost:{port}"),
            ),
            ("--minio-bucket".to_string(), "openubl".to_string()),
            (
                "--storage-minio-access-key".to_string(),
                "access-key".to_string(),
            ),
            (
                "--storage-minio-secret-key".to_string(),
                "secret-key".to_string(),
            ),
        ]);

        Self {
            container: minio,
            properties,
        }
    }
}
