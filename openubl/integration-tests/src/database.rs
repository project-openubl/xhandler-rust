use std::collections::HashMap;

use testcontainers::clients::Cli;
use testcontainers::Container;
use testcontainers_modules::postgres::Postgres;

pub struct DatabaseServer<'a> {
    container: Container<'a, Postgres>,
    properties: HashMap<String, String>,
}

impl DatabaseServer<'_> {
    pub fn new(docker: &Cli) -> Self {
        let container = docker.run(Postgres::default());

        let port = container.get_host_port_ipv4(5432).to_string();
        let properties = HashMap::from([
            ("--db-user".to_string(), "postgres".to_string()),
            ("--db-password".to_string(), "postgres".to_string()),
            ("--db-host".to_string(), "localhost".to_string()),
            ("--db-port".to_string(), port),
            ("--db-name".to_string(), "postgres".to_string()),
        ]);

        Self {
            container,
            properties,
        }
    }
}
