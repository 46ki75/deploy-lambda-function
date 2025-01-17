mod validate;

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

    /// The path to the Lambda function's deployment package directory. (Not to need to be a ZIP file.)
    #[arg(long)]
    pub path: Option<String>,

    /// An Amazon S3 bucket in the same AWS Region as your function. The bucket can be in a different AWS account.
    /// Use only with a function defined with a .zip file archive deployment package.
    ///
    /// - Length Constraints: Minimum length of 3. Maximum length of 63.
    /// - Pattern: `^[0-9A-Za-z\.\-_]*(?<!\.)$`
    #[arg(long)]
    pub s3_bucket: Option<String>,

    /// The Amazon S3 key of the deployment package. Use only with a function defined with a .zip file archive deployment package.
    ///
    /// - Length Constraints: Minimum length of 1. Maximum length of 1024.
    #[arg(long)]
    pub s3_key: Option<String>,

    /// Set to true to publish a new version of the function after updating the code.
    #[arg(long)]
    pub publish: bool,
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
