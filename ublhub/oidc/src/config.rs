#[derive(clap::Args, Debug)]
pub struct Oidc {
    #[arg(id = "oidc-auth-server-url", long, env = "OIDC_AUTH-SERVER_URL")]
    pub auth_server_url: String,
}
