use std::collections::HashMap;

use testcontainers::core::WaitFor;
use testcontainers::{Image, ImageArgs};

const NAME: &str = "quay.io/minio/minio";
const TAG: &str = "latest";
const COMMAND: &str = "server /data";

#[derive(Debug)]
pub struct Minio {
    name: String,
    tag: String,
    env_vars: HashMap<String, String>,
}

impl Default for Minio {
    fn default() -> Self {
        let mut env_vars = HashMap::new();
        env_vars.insert("MINIO_ROOT_USER".to_string(), "admin".to_string());
        env_vars.insert("MINIO_ROOT_PASSWORD".to_string(), "admin".to_string());

        Self {
            name: NAME.to_owned(),
            tag: TAG.to_owned(),
            env_vars,
        }
    }
}

impl Minio {
    pub fn new<S: Into<String>>(name: S, tag: S) -> Minio {
        Self {
            name: name.into(),
            tag: tag.into(),
            ..Default::default()
        }
    }

    pub fn with_env_var<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.env_vars.insert(key.into(), value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct MinioArgs {
    pub command: String,
}

impl Default for MinioArgs {
    fn default() -> Self {
        Self {
            command: COMMAND.to_string(),
        }
    }
}

impl ImageArgs for MinioArgs {
    fn into_iterator(self) -> Box<dyn Iterator<Item = String>> {
        let args = vec![self.command.to_owned()];
        Box::new(args.into_iter())
    }
}

impl Image for Minio {
    type Args = MinioArgs;

    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn tag(&self) -> String {
        self.tag.to_owned()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stdout("API:")]
    }

    fn env_vars(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
        Box::new(self.env_vars.iter())
    }
}
