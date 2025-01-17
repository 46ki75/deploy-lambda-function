use clap::Parser;

#[derive(Parser)]
#[command(name = "deploy_lambda_function")]
#[command(version)]
#[command(
    about = "Streamlines AWS Lambda deployment for CI/CD environments with a lightweight Rust-based CLI tool."
)]
#[command(author = "Chomolungma Shirayuki <shirayuki@46ki75.com>")]
pub struct Cli {
    #[arg(long)]
    pub region: Option<String>,

    #[arg(long)]
    pub profile: Option<String>,
}
