use clap::Parser;

pub mod cli;
pub mod error;
pub mod exec;
pub mod util;

pub async fn execute() -> Result<(), error::Error> {
    let cli = crate::cli::Cli::parse();

    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

    let client = aws_sdk_lambda::Client::new(&config);

    let path = cli.path.clone();
    let s3_bucket = cli.s3_bucket.clone();

    if s3_bucket.is_some() && path.is_some() {
        return Err(crate::error::Error::InvalidOption(
            "Both S3 bucket and path are provided".to_string(),
        ));
    } else if let Some(path) = path {
        let zip_bytes = crate::util::zip::archive_to_zip_bytes(&path).await.unwrap();

        let _ = crate::exec::update_function_code::update_function_code(
            &client,
            &cli.function_name,
            zip_bytes,
            cli.publish,
            false,
        )
        .await;
    } else if let Some(s3_bucket) = s3_bucket {
        match cli.s3_key.clone() {
            Some(s3_key) => {
                todo!("Implement S3");
            }
            None => {
                return Err(crate::error::Error::InvalidOption(
                    "S3 key is not provided".to_string(),
                ));
            }
        }
    }

    Ok(())
}
