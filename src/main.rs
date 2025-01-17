use clap::Parser;

#[tokio::main]
async fn main() {
    let cli = deploy_lambda_function::cli::Cli::parse();

    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

    let client = aws_sdk_lambda::Client::new(&config);

    let path = cli.path.clone();

    match path {
        Some(path) => {
            let zip_bytes = deploy_lambda_function::util::zip::archive_to_zip_bytes(&path)
                .await
                .unwrap();

            let _ = deploy_lambda_function::exec::update_function_code::update_function_code(
                &client,
                &cli.function_name,
                zip_bytes,
                false,
                false,
            )
            .await;
        }
        None => {
            println!("No path provided");
        }
    }
}
