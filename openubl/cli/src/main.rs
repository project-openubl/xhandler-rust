use std::process::{ExitCode, Termination};

use clap::Parser;

#[allow(clippy::large_enum_variant)]
#[derive(clap::Subcommand, Debug)]
pub enum Command {
    Server(openubl_server::Run),
}

#[derive(clap::Parser, Debug)]
#[command(
    author,
    version = env ! ("CARGO_PKG_VERSION"),
    about = "openubl",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

impl Cli {
    async fn run(self) -> ExitCode {
        match self.run_command().await {
            Ok(code) => code,
            Err(err) => {
                eprintln!("Error: {err}");
                for (n, err) in err.chain().skip(1).enumerate() {
                    if n == 0 {
                        eprintln!("Caused by:");
                    }
                    eprintln!("\t{err}");
                }

                ExitCode::FAILURE
            }
        }
    }

    async fn run_command(self) -> anyhow::Result<ExitCode> {
        match self.command {
            Command::Server(run) => run.run().await,
        }
    }
}

#[actix_web::main]
async fn main() -> impl Termination {
    Cli::parse().run().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
