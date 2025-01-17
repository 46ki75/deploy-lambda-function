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

    match path {
        Some(path) => {
            let zip_bytes = crate::util::zip::archive_to_zip_bytes(&path).await.unwrap();

            let _ = crate::exec::update_function_code::update_function_code(
                &client,
                &cli.function_name,
                zip_bytes,
                cli.publish,
                false,
            )
            .await;
        }
        None => {
            println!("No path provided");
        }
    };

    Ok(())
}
