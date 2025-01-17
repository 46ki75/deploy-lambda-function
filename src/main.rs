#[tokio::main]
async fn main() -> Result<(), deploy_lambda_function::error::Error> {
    deploy_lambda_function::execute().await
}
