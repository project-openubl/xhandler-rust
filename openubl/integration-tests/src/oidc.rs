use fs::read_to_string;
use std::collections::HashMap;
use std::fs;

use testcontainers::clients::Cli;
use testcontainers::core::{ExecCommand, WaitFor};
use testcontainers::Container;

use crate::modules::keycloak::KeycloakDev;

pub struct OidcServer<'a> {
    container: Container<'a, KeycloakDev>,
    properties: HashMap<String, String>,
}

impl OidcServer<'_> {
    pub fn new() -> Self {
        let docker = Cli::default();
        let keycloak = docker.run(
            KeycloakDev::new("quay.io/keycloak/keycloak", "23.0.3")
                .with_env_var("KEYCLOAK_ADMIN", "admin")
                .with_env_var("KEYCLOAK_ADMIN_PASSWORD", "admin"),
        );
        let keycloak_port = keycloak.get_host_port_ipv4(8080);

        keycloak.exec(ExecCommand {
            cmd: read_to_string("resources/keycloak-setup.sh")
                .expect("keycloak-setup.sh not found"),
            ready_conditions: vec![WaitFor::Nothing],
        });

        let properties = HashMap::from([(
            "--oidc-auth-server-url".to_string(),
            format!("http://localhost:{keycloak_port}/realms/openubl"),
        )]);

        Self {
            container: keycloak,
            properties,
        }
    }
}
