use std::collections::HashMap;

use testcontainers::core::WaitFor;
use testcontainers::{Image, ImageArgs};

const NAME: &str = "quay.io/keycloak/keycloak";
const TAG: &str = "latest";
const COMMAND: &str = "start-dev";

#[derive(Debug)]
pub struct KeycloakDev {
    name: String,
    tag: String,
    env_vars: HashMap<String, String>,
}

impl Default for KeycloakDev {
    fn default() -> Self {
        let mut env_vars = HashMap::new();
        env_vars.insert("KEYCLOAK_ADMIN".to_string(), "admin".to_string());
        env_vars.insert("KEYCLOAK_ADMIN_PASSWORD".to_string(), "admin".to_string());

        Self {
            name: NAME.to_owned(),
            tag: TAG.to_owned(),
            env_vars,
        }
    }
}

impl KeycloakDev {
    pub fn new<S: Into<String>>(name: S, tag: S) -> KeycloakDev {
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
pub struct KeycloakArgs {
    pub command: String,
}

impl Default for KeycloakArgs {
    fn default() -> Self {
        Self {
            command: COMMAND.to_string(),
        }
    }
}

impl ImageArgs for KeycloakArgs {
    fn into_iterator(self) -> Box<dyn Iterator<Item = String>> {
        let args = vec![self.command.to_owned()];
        Box::new(args.into_iter())
    }
}

impl Image for KeycloakDev {
    type Args = KeycloakArgs;

    fn name(&self) -> String {
        self.name.clone()
    }

    fn tag(&self) -> String {
        self.tag.clone()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stdout(
            "Listening on: http://0.0.0.0:8080",
        )]
    }

    fn env_vars(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
        Box::new(self.env_vars.iter())
    }
}
