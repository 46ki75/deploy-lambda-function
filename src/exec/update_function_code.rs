/// @see <ttps://docs.aws.amazon.com/lambda/latest/api/API_UpdateFunctionCode.htm>
pub async fn update_function_code(
    client: &aws_sdk_lambda::Client,
    function_name: &str,
    zip_bytes: Vec<u8>,
    publish: bool,
    dry_run: bool,
) -> Result<(), crate::error::Error> {
    let zip_blob = aws_sdk_lambda::primitives::Blob::new(zip_bytes);

    let request = client
        .update_function_code()
        .function_name(function_name)
        .zip_file(zip_blob)
        .publish(publish)
        .dry_run(dry_run);

    let _response = request.send().await.map_err(|e| {
        println!("{:?}", e);
        crate::error::Error::UpdateFunctionCodeError(e.to_string())
    })?;

    Ok(())
}
