use std::collections::HashMap;

use testcontainers::clients::Cli;
use testcontainers::Container;

use crate::modules::nats::Nats;

pub struct MessagingServer<'a> {
    container: Container<'a, Nats>,
    properties: HashMap<String, String>,
}

impl MessagingServer<'_> {
    pub fn new() -> Self {
        let docker = Cli::default();
        let container = docker.run(Nats::new("nats", "latest"));

        Self {
            container,
            properties: Default::default(),
        }
    }
}
