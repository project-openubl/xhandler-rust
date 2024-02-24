#[derive(clap::Args, Debug)]
pub struct SenderRun {
    #[arg(id = "input", long)]
    pub input: Vec<String>,
}

impl SenderRun {
    // pub async fn run(self) -> anyhow::Result<ExitCode> {
    //     Ok(ExitCode::SUCCESS)
    // }
}
