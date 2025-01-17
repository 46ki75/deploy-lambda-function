use clap::Parser;

#[derive(Parser)]
#[command(name = "deploy_lambda_function")]
#[command(version)]
#[command(
    about = "Streamlines AWS Lambda deployment for CI/CD environments with a lightweight Rust-based CLI tool."
)]
#[command(author = "Chomolungma Shirayuki <shirayuki@46ki75.com>")]
pub struct Cli {
    /// The AWS profile to use for deployment.
    #[arg(long)]
    pub profile: Option<String>,

    /// The AWS region to deploy the Lambda function to.
    #[arg(long)]
    pub region: Option<String>,

    /// The name of the Lambda function to deploy.
    #[arg(long)]
    pub function_name: String,
}
