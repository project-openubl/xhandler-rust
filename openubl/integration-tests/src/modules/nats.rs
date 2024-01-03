use std::collections::HashMap;

use testcontainers::core::WaitFor;
use testcontainers::{Image, ImageArgs};

const NAME: &str = "nats";
const TAG: &str = "latest";
const COMMAND: &str = "-js";

#[derive(Debug)]
pub struct Nats {
    name: String,
    tag: String,
    env_vars: HashMap<String, String>,
}

impl Default for Nats {
    fn default() -> Self {
        Self {
            name: NAME.to_owned(),
            tag: TAG.to_owned(),
            env_vars: HashMap::new(),
        }
    }
}

impl Nats {
    pub fn new<S: Into<String>>(name: S, tag: S) -> Nats {
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
pub struct NatsArgs {
    pub command: String,
}

impl Default for NatsArgs {
    fn default() -> Self {
        Self {
            command: COMMAND.to_string(),
        }
    }
}

impl ImageArgs for NatsArgs {
    fn into_iterator(self) -> Box<dyn Iterator<Item = String>> {
        let args = vec![self.command.to_owned()];
        Box::new(args.into_iter())
    }
}

impl Image for Nats {
    type Args = NatsArgs;

    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn tag(&self) -> String {
        self.tag.to_owned()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::seconds(3)]
    }

    fn env_vars(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
        Box::new(self.env_vars.iter())
    }

    fn expose_ports(&self) -> Vec<u16> {
        vec![4222]
    }
}
